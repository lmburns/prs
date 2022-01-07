use crate::{crypto::IsContext, store::Store, types::Plaintext, OTP_DEFUALT_FILE};
use anyhow::Result;
use colored::Colorize;
use data_encoding::{DecodeError, BASE32_NOPAD};
use derive_builder::Builder;
use once_cell::sync::Lazy;
use regex::Regex;
use ring::hmac;
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs, io,
    path::PathBuf,
    string::ToString,
    time::{SystemTime, SystemTimeError, UNIX_EPOCH},
};
use strum_macros::Display;
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

    #[error("failed to get regex captures")]
    RegexCaptures,

    #[error("failed to get {0} captures")]
    RegexCaptureName(String),

    #[error("failed parse integer")]
    ParseInt(#[source] std::num::ParseIntError),
}

static URI_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(
        r"(?x)
        otpauth://
        (?P<type>totp|hotp)/                       # otp type
        (?P<label>[^?\#]*)                         # label
        (?:[?&]secret=(?P<secret>[^&]*))           # secret
        (?:[?&]issuer=(?P<issuer>[^&\#]*))?        # issuer
        (?:[?&]algorithm=(?P<algorithm>[^&\#]*))?  # algorithm
        (?:[?&]digits=(?P<digits>[^&\#]*))?        # digits
        (?:[?&]period=(?P<period>[^&\#]*))?        # period/interval
    ",
    )
    .unwrap()
});

#[derive(Debug, Copy, Clone, PartialEq, Default, Display)]
pub enum OTPType {
    #[strum(serialize = "hotp")]
    HOTP,
    #[default]
    #[strum(serialize = "totp")]
    TOTP,
}

/// Available hashing algorithms
#[derive(Debug, Copy, Clone, Default, Display, PartialEq, Deserialize, Serialize)]
pub enum HashFunction {
    #[default]
    #[strum(serialize = "sha1")]
    #[serde(rename = "SHA1")]
    Sha1,
    #[strum(serialize = "sha256")]
    #[serde(rename = "SHA256")]
    Sha256,
    #[strum(serialize = "sha384")]
    #[serde(rename = "SHA384")]
    Sha384,
    #[strum(serialize = "sha512")]
    #[serde(rename = "SHA512")]
    Sha512,
}

impl HashFunction {
    /// Convert `str` to variant (default is `SHA1`)
    pub fn from_str(hash: &str) -> HashFunction {
        match hash.to_ascii_lowercase().trim() {
            "sha256" | "256" => HashFunction::Sha256,
            "sha384" | "384" => HashFunction::Sha384,
            "sha512" | "512" => HashFunction::Sha512,
            _ => HashFunction::Sha1,
        }
    }

    // /// Get hash function name
    // pub fn name(self) -> &'static str {
    //     match self {
    //         HashFunction::Sha1 => "sha1",
    //         HashFunction::Sha256 => "sha256",
    //         HashFunction::Sha384 => "sha384",
    //         HashFunction::Sha512 => "sha512",
    //     }
    // }
}

// impl fmt::Display for HashFunction {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.clone().name())
//     }
// }

// TODO: use or delete

#[derive(Debug, Copy, Clone, Default, Display, PartialEq)]
pub enum OTPLength {
    #[default]
    #[strum(serialize = "6")]
    Six = 6,
    #[strum(serialize = "8")]
    Eight = 8,
}

// otpauth://totp/GitHub:username?secret=BASE32&issuer=GitHub
#[derive(Debug, Builder, Default, Clone)]
#[builder(default)]
pub struct OTPLabel {
    #[builder(default = "None")]
    pub issuer:      Option<String>,
    #[builder(default = "String::new()")]
    pub accountname: String,
}

#[derive(Debug, Builder, Clone, Default)]
#[builder(default)]
pub struct OTPUri {
    #[builder(default = "Vec::new()")]
    pub secret:        Vec<u8>,
    #[builder(default = "OTPType::TOTP")]
    pub otptype:       OTPType,
    #[builder(default = "None")]
    pub hash_function: Option<HashFunction>,
    #[builder(default = "None")]
    pub counter:       Option<u64>,
    #[builder(default = "None")]
    pub period:        Option<u64>,
    #[builder(default = "None")]
    pub output_len:    Option<OTPLength>,
    #[builder(default = "OTPLabel::default()")]
    pub label:         OTPLabel,
}

/// OTP representation with all its options
#[allow(dead_code)]
#[derive(Debug, Clone, Builder, Default)]
#[builder(default)]
pub struct OneTimePassword {
    key:           Vec<u8>,
    #[builder(default = "None")]
    uri:           Option<String>,
    #[builder(default = "0_u64")]
    counter:       u64,
    #[builder(default = "30_u64")]
    pub period:    u64,
    #[builder(default = "true")]
    pub totp:      bool,
    #[builder(default = "6_usize")]
    output_len:    usize,
    #[builder(default = "\"0123456789\".to_owned().into_bytes()")]
    output_base:   Vec<u8>,
    #[builder(default = "HashFunction::Sha1")]
    hash_function: HashFunction,
    raw_key:       String,
}

impl From<Account> for OneTimePassword {
    fn from(account: Account) -> Self {
        Self {
            key: parse_base32(&account.key).unwrap(),
            uri: account.uri,
            counter: account.counter.unwrap_or_default(),
            period: account.period,
            totp: account.totp,
            hash_function: account.hash_function,
            raw_key: account.key,
            ..Default::default()
        }
    }
}

impl OneTimePassword {
    // Calculate counter based on whether the OTP is time based or counter based
    pub fn get_counter(&self) -> u64 {
        if self.totp {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map_err(OtpError::InvalidTimeError)
                .unwrap()
                .as_secs() as u64;
            timestamp / self.period
        } else {
            self.counter
        }
    }

    pub fn generate(&self) -> String {
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
        self.encode_digest(digest.as_ref())
    }

    fn encode_digest(&self, digest: &[u8]) -> String {
        let offset = (*digest.last().unwrap() & 0xf) as usize;
        let snum: u32 = ((u32::from(digest[offset]) & 0x7f) << 24)
            | ((u32::from(digest[offset + 1]) & 0xff) << 16)
            | ((u32::from(digest[offset + 2]) & 0xff) << 8)
            | (u32::from(digest[offset + 3]) & 0xff);
        let base = self.output_base.len() as u32;
        let hotp_code = snum % base.pow(self.output_len as u32);
        format!("{:0width$}", hotp_code, width = self.output_len)
    }

    /// Write 6 or 8 digit code to standard output
    pub fn display_code(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let duration = self.period - (now % self.period);
        #[allow(clippy::cast_possible_truncation)]
        let elapsed = (self.period - duration) as usize;
        #[allow(clippy::cast_possible_truncation)]
        let remaining = (duration % self.period) as usize;

        // If this check is not here, a stack overflow will occur
        if elapsed == 0 {
            println!(
                "{} {} [{}{}]",
                self.generate().magenta().bold(),
                format!("{}s", duration).green(),
                "<".green().bold(),
                "=".repeat(remaining - 1).green(),
            );
        } else {
            println!(
                "{} {} [{}{}{}]",
                self.generate().magenta().bold(),
                format!("{}s", duration).color(if remaining > 12 {
                    "green"
                } else if remaining > 6 {
                    "yellow"
                } else {
                    "red"
                }),
                "-".repeat(elapsed + 1).red(),
                "<".green().bold(),
                "=".repeat(remaining - 1).green()
            );
        }
    }
}

/// Struct that OTP is stored
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Builder, Default)]
#[builder(default)]
pub struct Account {
    pub name:          String,
    #[builder(default = "None")]
    pub uri:           Option<String>,
    pub path:          PathBuf,
    pub key:           String,
    #[builder(default = "true")]
    pub totp:          bool,
    pub hash_function: HashFunction,
    #[builder(default = "None")]
    pub counter:       Option<u64>,
    #[builder(default = "30_u64")]
    pub period:        u64,
}

/// File that keeps track of all files containing OTP's
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OtpFile(BTreeMap<String, Account>);

impl Default for OtpFile {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}

// TODO: fix failure to communicate with gpg binary error not trapping
impl OtpFile {
    /// Create a new instance of or open an existing OTP hashing file
    pub fn new(store: &Store) -> Result<Self> {
        let otp_file = store.root.join(OTP_DEFUALT_FILE);
        if !otp_file.exists() {
            tracing::debug!("creating parent dir of otp_file");
            otp_file.parent().map(fs::create_dir_all).transpose()?;
            Ok(Self::default())
        } else {
            let plaintext = crate::crypto::context(&crate::CONFIG)?
                .decrypt_file(&otp_file)
                .map_err(OtpError::Decrypt)?;

            serde_json::from_slice(plaintext.unsecure_ref())
                .map_err(|e| OtpError::SerDeserialization(e).into())
        }
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

    /// Return the length of the keys
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
        let otp_file = store.root.join(OTP_DEFUALT_FILE);
        let recipients = store.recipients()?;

        crate::crypto::context(&crate::CONFIG)?
            .encrypt_file(
                &recipients,
                Plaintext::from(serde_json::to_string_pretty(&self.0)?),
                &otp_file,
            )
            .map_err(OtpError::Encrypt)?;
        Ok(())
    }
}

pub fn parse_base32(key: &str) -> Result<Vec<u8>> {
    Ok(BASE32_NOPAD
        .decode(key.as_bytes())
        .map_err(|err| OtpError::KeyDecode {
            key:   key.to_owned(),
            cause: Box::new(err),
        })?)
}

/// Check whether the user input matches the URI schema
pub fn has_uri(uri: &str) -> bool {
    if !URI_RE.is_match(uri) {
        return false;
    }
    uri_secret(uri).map_or(false, |s| parse_base32(&s).is_ok())
}

/// Get the secret key of the URI
pub fn uri_secret(uri: &str) -> Result<String> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("secret")
        .ok_or(OtpError::RegexCaptureName("secret".to_string()))?
        .as_str()
        .to_owned())
}

/// Get the issuer of the URI
pub fn uri_issuer(uri: &str) -> Result<String> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("issuer")
        .ok_or(OtpError::RegexCaptureName("issuer".to_string()))?
        .as_str()
        .to_owned())
}

/// Get the URI refresh period (num of seconds) from the given URI
pub fn uri_period(uri: &str) -> Result<u64> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("period")
        .map_or(30, |p| {
            p.as_str()
                .parse::<u64>()
                .map_err(OtpError::ParseInt)
                .unwrap()
        }))
}

/// Get the URI number of digits from the given URI
pub fn uri_digits(uri: &str) -> Result<usize> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("digits")
        .map_or(6, |p| {
            p.as_str()
                .parse::<usize>()
                .map_err(OtpError::ParseInt)
                .unwrap()
        }))
}

/// Get the URI algorithm from the given URI
//   - Sha1
//   - Sha256
//   - Sha384
//   - Sha512
pub fn uri_algorithm(uri: &str) -> Result<HashFunction> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("algorithm")
        .map_or(HashFunction::Sha1, |f| HashFunction::from_str(f.as_str())))
}

/// Get the URI type from the given URI
pub fn uri_type(uri: &str) -> Result<bool> {
    Ok(URI_RE
        .captures(uri)
        .ok_or(OtpError::RegexCaptures)?
        .name("type")
        .map_or(true, |p| p.as_str() == "totp"))
}
