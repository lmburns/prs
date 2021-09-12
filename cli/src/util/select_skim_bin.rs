use std::{
    collections::HashMap,
    env,
    io::Write,
    process::{Command, Stdio},
};

use prs_lib::{
    otp::{Account, OtpFile},
    Key, Secret,
};

/// Binary name.
#[cfg(not(windows))]
const BIN_NAME: &str = "sk";
#[cfg(windows)]
const BIN_NAME: &str = "sk.exe";

/// Select secret.
pub(crate) fn select_secret(secrets: &[Secret]) -> Option<&Secret> {
    // Return if theres just one to choose
    if secrets.len() == 1 {
        return secrets.get(0);
    }

    let map: HashMap<_, _> = secrets
        .into_iter()
        .map(|secret| (secret.name.clone(), secret))
        .collect();
    let items: Vec<_> = map.keys().collect();
    select_item("Select key", &items)
        .as_ref()
        .map(|item| map[item])
}

/// Select key.
pub(crate) fn select_key<'a>(keys: &'a [Key], prompt: Option<&'a str>) -> Option<&'a Key> {
    let map: HashMap<_, _> = keys.into_iter().map(|key| (key.to_string(), key)).collect();
    let items: Vec<_> = map.keys().collect();
    select_item(prompt.unwrap_or("Select key"), &items)
        .as_ref()
        .map(|item| map[item])
}

/// Select otp
pub(crate) fn select_otp(otp: &OtpFile) -> Option<&Account> {
    if otp.len() == 1 {
        return otp.get(otp.keys().collect::<Vec<_>>()[0]);
    }

    select_item("Select otp", &otp.keys().collect::<Vec<_>>())
        .as_ref()
        .map(|key| otp.get(key).unwrap())
}

/// Interactively select one of the given items.
fn select_item<'a, S: AsRef<str>>(prompt: &'a str, items: &'a [S]) -> Option<String> {
    // Build sorted list of string references as items
    let mut items = items.into_iter().map(|i| i.as_ref()).collect::<Vec<_>>();
    items.sort_unstable();

    // Spawn skim
    let mut command = Command::new(BIN_NAME);
    command
        .arg("--prompt")
        .arg(format!("{}: ", prompt))
        .arg("--height")
        .arg("50%")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::inherit());

    if let Some(skim_opts) = env::var_os("SKIM_DEFAULT_OPTIONS") {
        command.env("SKIM_DEFAULT_OPTIONS", skim_opts);
    }

    let mut child = command.spawn().expect("failed to spawn skim");

    // Communicate list of items to skim
    let data = items.join("\n");
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(data.as_bytes())
        .expect("failed to communicate list of items to skim");

    let output = child
        .wait_with_output()
        .expect("failed to select with skim");

    // No item selected on non-zero exit code
    if !output.status.success() {
        return None;
    }

    // Get selected item, assert validity
    let stdout = std::str::from_utf8(&output.stdout).unwrap();
    let stdout = stdout.strip_suffix("\n").unwrap_or(stdout);
    assert!(items.contains(&stdout));

    Some(stdout.into())
}
