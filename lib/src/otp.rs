use crate::{crypto::IsContext, store::{Store, Secret}, types::Plaintext, OTP_DEFUALT_FILE};
use anyhow::Result;
use data_encoding::{DecodeError, BASE32_NOPAD};
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    convert::TryInto,
    fmt, fs, io,
    path::PathBuf,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum OtpError {
    #[error("Decoding failed for key '{}': {}", key, cause)]
    KeyDecode {
        key:   String,
        cause: Box<DecodeError>,
    },

    #[error("failure to write to file: {0}")]
    WriteFile(#[source] io::Error),

    #[error("failure to serialize/deserialize file to string: {0}")]
    SerDeserialization(#[from] serde_json::Error),

    /// Invalid time
    #[error("invalid time provided")]
    InvalidTimeError(#[source] SystemTimeError),

    /// Invalid digest
    #[error("invalid digest provided: {:?}", _0)]
    InvalidDigest(Vec<u8>),

    #[error("failed to write decrypted unserialized file")]
    Write(#[source] std::io::Error),

    #[error("failed to decrypt otp file")]
    Decrypt(#[source] anyhow::Error),

    #[error("failed to encrypt otp file")]
    Encrypt(#[source] anyhow::Error),

    #[error("failed to read from file")]
    ReadFile(#[source] std::io::Error),
}

/// Available hashing algorithms
#[derive(Debug, Clone)]
pub enum HashFunction {
    Sha1,
    Sha256,
    Sha384,
    Sha512,
}

impl HashFunction {
    /// List all supported hashing functions
    pub fn variants() -> &'static [HashFunction] {
        &[
            HashFunction::Sha1,
            HashFunction::Sha256,
            HashFunction::Sha384,
            HashFunction::Sha512,
        ]
    }

    /// Convert `str` to variant (default is `SHA1`)
    pub fn from_str(hash: &str) -> HashFunction {
        match hash.trim().to_ascii_lowercase().as_str() {
            "sha256" | "256" => HashFunction::Sha256,
            "sha384" | "384" => HashFunction::Sha384,
            "sha512" | "512" => HashFunction::Sha512,
            _ => HashFunction::Sha1,
        }
    }

    /// Get hash function name
    pub fn name(self) -> &'static str {
        match self {
            HashFunction::Sha1 => "sha1",
            HashFunction::Sha256 => "sha256",
            HashFunction::Sha384 => "sha384",
            HashFunction::Sha512 => "sha512",
        }
    }
}

impl fmt::Display for HashFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.clone().name())
    }
}

// USER INTERACTION

/// OTP representation with all its options
#[derive(Debug, Clone)]
pub struct OneTimePassword {
    key:           Vec<u8>,
    counter:       u64,
    totp:          bool,
    output_len:    usize,
    output_base:   Vec<u8>,
    hash_function: HashFunction,
    raw_key:       String,
}

impl OneTimePassword {
    pub fn new(
        key: &str,
        totp: bool,
        hash_function: &str,
        counter: Option<u64>,
        output_len: Option<usize>,
    ) -> Result<Self> {
        Ok(Self {
            key: BASE32_NOPAD
                .decode(key.as_bytes())
                .map_err(|err| OtpError::KeyDecode {
                    key:   key.to_owned(),
                    cause: Box::new(err),
                })?,
            counter: counter.unwrap_or(0_u64),
            totp,
            output_len: output_len.unwrap_or(6_usize),
            output_base: "0123456789".to_owned().into_bytes(),
            hash_function: HashFunction::from_str(hash_function),
            raw_key: key.to_string(),
        })
    }

    /// Return OTP code
    pub fn generate(&self) -> String {
        type HF = HashFunction;
        let digest = hmac::sign(
            &match self.hash_function {
                HF::Sha1 => hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &self.key),
                HF::Sha256 => hmac::Key::new(hmac::HMAC_SHA256, &self.key),
                HF::Sha384 => hmac::Key::new(hmac::HMAC_SHA384, &self.key),
                HF::Sha512 => hmac::Key::new(hmac::HMAC_SHA512, &self.key),
            },
            &self.get_counter().to_be_bytes(),
        );

        self.encode_digest(digest.as_ref()).unwrap()
    }

    /// Encodes the HMAC digest into a 6-digit integer.
    fn encode_digest(&self, digest: &[u8]) -> Result<String, OtpError> {
        // let offset: usize = (digest[digest.len() - 1] & 0xf) as usize;
        // let b: &[u8; 4] = (&digest[offset..offset + 4]).try_into().unwrap();
        // let base = u32::from_be_bytes(*b) & 0x7fff_ffff;
        //
        // let code = format!(
        //     "{:01$}",
        //     base % (10 as u32).pow(self.output_len as u32),
        //     self.output_len as usize
        // );

        // TODO: Add time until expiration

        let offset = (*digest.last().unwrap() & 0xf) as usize;
        let code_bytes: [u8; 4] = match digest[offset..offset + 4].try_into() {
            Ok(x) => x,
            Err(_) => return Err(OtpError::InvalidDigest(Vec::from(digest))),
        };

        let base = self.output_base.len() as u32;
        let hotp_code = (u32::from_be_bytes(code_bytes) & 0x7fffffff)
            % 1_000_000
            % base.pow(self.output_len as u32);

        let code = format!("{:0width$}", hotp_code, width = self.output_len);
        Ok(code)
    }

    // Calculate counter based on whether the OTP is time based or counter based
    fn get_counter(&self) -> u64 {
        if self.totp {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(OtpError::InvalidTimeError)
                .unwrap()
                .as_secs() as u64;
            timestamp / 30
        } else {
            self.counter
        }
    }

    pub fn generate2(&self) -> String {
        let counter = self.get_counter();
        let message: [u8; 8] = [
            ((counter >> 56) & 0xff) as u8,
            ((counter >> 48) & 0xff) as u8,
            ((counter >> 40) & 0xff) as u8,
            ((counter >> 32) & 0xff) as u8,
            ((counter >> 24) & 0xff) as u8,
            ((counter >> 16) & 0xff) as u8,
            ((counter >> 8) & 0xff) as u8,
            (counter & 0xff) as u8,
        ];
        let signing_key = match self.hash_function {
            HashFunction::Sha1 => hmac::Key::new(hmac::HMAC_SHA1_FOR_LEGACY_USE_ONLY, &self.key),
            HashFunction::Sha256 => hmac::Key::new(hmac::HMAC_SHA256, &self.key),
            HashFunction::Sha384 => hmac::Key::new(hmac::HMAC_SHA384, &self.key),
            HashFunction::Sha512 => hmac::Key::new(hmac::HMAC_SHA512, &self.key),
        };
        let digest = hmac::sign(&signing_key, &message);
        self.encode_digest2(digest.as_ref())
    }

    fn encode_digest2(&self, digest: &[u8]) -> String {
        let offset = (*digest.last().unwrap() & 0xf) as usize;
        let snum: u32 = ((u32::from(digest[offset]) & 0x7f) << 24)
            | ((u32::from(digest[offset + 1]) & 0xff) << 16)
            | ((u32::from(digest[offset + 2]) & 0xff) << 8)
            | (u32::from(digest[offset + 3]) & 0xff);
        let base = self.output_base.len() as u32;
        let hotp_code = snum % base.pow(self.output_len as u32);
        let code = format!("{:0width$}", hotp_code, width = self.output_len);
        code
    }
}

/// Struct that OTP is stored
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Account {
    pub name:          String,
    pub path:          PathBuf,
    pub key:           String,
    pub totp:          bool,
    pub hash_function: String,
    pub counter:       Option<u64>,
}

impl From<Account> for Secret {
    fn from(account: Account) -> Self {
        Self {
            name: account.name,
            path: account.path
        }
    }
}

/// File that keeps track of all files containing OTP's
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtpFile(BTreeMap<String, Account>);

impl Default for OtpFile {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

impl OtpFile {
    /// Create a new instance of or open an existing OTP hashing file
    pub fn new(store: &Store) -> Result<Self> {
        let otp_file = store.root.join(OTP_DEFUALT_FILE);
        if !otp_file.exists() {
            tracing::debug!("creating parent dir of otp_file");
            otp_file.parent().map(fs::create_dir_all).transpose()?;
        } else {
            let plaintext = crate::crypto::context(&crate::CONFIG)?
                .decrypt_file(&otp_file)
                .map_err(OtpError::Decrypt)?;
            tracing::debug!("decrypting file");
            let read: BTreeMap<String, Account> = serde_json::from_slice(plaintext.unsecure_ref())
                .map_err(OtpError::SerDeserialization)?;
            tracing::debug!("READ: {:#?}", read);

            fs::write(
                &otp_file,
                serde_json::to_string_pretty(&read).map_err(OtpError::SerDeserialization)?,
            )
            .map_err(OtpError::Write)?;
        }

        let reader = match fs::File::open(otp_file.as_path()) {
            Ok(file) => file,
            Err(err) =>
                if err.kind() == io::ErrorKind::NotFound {
                    tracing::debug!("creating new otp file");
                    return Ok(Self::default());
                } else {
                    tracing::error!(error=%err);
                    return Err(err.into());
                },
        };

        serde_json::from_reader(reader).map_err(Into::into)
    }

    /// Close the OTP file by encrypting it and writing back to disk
    pub fn close(store: &Store) -> Result<()> {
        let otp_file = store.root.join(OTP_DEFUALT_FILE);
        let recipients = store.recipients()?;
        crate::crypto::context(&crate::CONFIG)?
            .encrypt_file(
                &recipients,
                Plaintext::from(
                    fs::read(store.root.join(OTP_DEFUALT_FILE)).map_err(OtpError::ReadFile)?,
                ),
                &otp_file,
            )
            .map_err(OtpError::Encrypt)?;

        Ok(())
    }

    /// Get the OTP account information
    pub fn get(&self, sec_path: &str) -> Option<&Account> {
        self.0.get(sec_path)
    }

    /// List the OTP account names and OTP code values
    pub fn list(&self) -> &BTreeMap<String, Account> {
        &self.0
    }

    /// Return an iterator of OTP names
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }

    pub fn len(&self) -> usize {
        self.0.keys().len()
    }

    /// Add an account to the OTP hash
    pub fn add(&mut self, account: Account) {
        self.0.insert(account.name.clone(), account);
    }

    /// Delete an account from the OTP hash
    pub fn delete(&mut self, sec_path: String) -> Option<Account> {
        self.0.remove(&sec_path)
    }

    /// Save the modified OTP hash
    pub fn save(&self, store: &Store) -> Result<()> {
        fs::write(
            store.root.join(OTP_DEFUALT_FILE),
            serde_json::to_string_pretty(&self.0)?,
        )
        .map_err(OtpError::WriteFile)?;
        Ok(())
    }
}
