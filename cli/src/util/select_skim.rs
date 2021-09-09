use std::{borrow::Cow, path::PathBuf, sync::Arc};

use prs_lib::{Key, Secret};
use skim::{
    prelude::{SkimItemReceiver, SkimItemSender, SkimOptionsBuilder},
    AnsiString, DisplayContext, Skim, SkimItem,
};

/// Show an interactive selection view for the given list of `items`.
/// The selected item is returned.  If no item is selected, `None` is returned
/// instead.
fn skim_select(items: SkimItemReceiver, prompt: &str) -> Option<String> {
    let mut skim_args = Vec::new();
    let default_height = String::from("50%");
    let default_margin = String::from("0%");
    let default_layout = String::from("default");
    let default_theme = String::from(
        "matched:108,matched_bg:0,current:254,current_bg:236,current_match:151,current_match_bg:\
         236,spinner:148,info:144,prompt:110,cursor:161,selected:168,header:109,border:59",
    );

    skim_args.extend(
        std::env::var("SKIM_DEFAULT_OPTIONS")
            .ok()
            .and_then(|val| shlex::split(&val))
            .unwrap_or_default(),
    );
    // For the  --color option, try and not pickup --preview, which can contain
    // '--color' (e.g., 'bat --color=always {}'). Could also check next item in
    // vector if item only equals '--color'.
    let prompt = format!("{}: ", prompt);
    let options = SkimOptionsBuilder::default()
        .prompt(Some(&prompt))
        .margin(Some(
            skim_args
                .iter()
                .find(|arg| arg.contains("--margin") && *arg != &"--margin".to_string())
                .unwrap_or_else(|| {
                    skim_args
                        .iter()
                        .position(|arg| arg.contains("--margin"))
                        .map_or(&default_margin, |pos| &skim_args[pos + 1])
                }),
        ))
        .height(Some(
            skim_args
                .iter()
                .find(|arg| arg.contains("--height") && *arg != &"--height".to_string())
                .unwrap_or_else(|| {
                    skim_args
                        .iter()
                        .position(|arg| arg.contains("--height"))
                        .map_or(&default_height, |pos| &skim_args[pos + 1])
                }),
        ))
        .layout(
            skim_args
                .iter()
                .find(|arg| arg.contains("--layout") && *arg != &"--layout".to_string())
                .unwrap_or_else(|| {
                    skim_args
                        .iter()
                        .position(|arg| arg.contains("--layout"))
                        .map_or(&default_layout, |pos| &skim_args[pos + 1])
                }),
        )
        .color(Some(
            skim_args
                .iter()
                .find(|arg| {
                    arg.contains("--color") && *arg != &"--color".to_string() && !arg.contains("{}")
                })
                .unwrap_or_else(|| {
                    skim_args
                        .iter()
                        .position(|arg| arg.contains("--color"))
                        .map_or(&default_theme, |pos| &skim_args[pos + 1])
                }),
        ))
        .bind(
            skim_args
                .iter()
                .filter(|arg| arg.contains("--bind"))
                .map(String::as_str)
                .collect::<Vec<_>>(),
        )
        .reverse(skim_args.iter().any(|arg| arg.contains("--reverse")))
        .tac(skim_args.iter().any(|arg| arg.contains("--tac")))
        .nosort(skim_args.iter().any(|arg| arg.contains("--no-sort")))
        .inline_info(skim_args.iter().any(|arg| arg.contains("--inline-info")))
        .multi(false)
        .build()
        .unwrap();

    // Run skim, get output, abort on close
    let output = Skim::run_with(&options, Some(items))?;
    if output.is_abort {
        return None;
    }

    // Get the first selected, and return
    output
        .selected_items
        .iter()
        .next()
        .map(|i| i.output().to_string())
}

/// Wrapped store secret item for skim.
pub struct SkimSecret(Secret);

impl From<Secret> for SkimSecret {
    fn from(secret: Secret) -> Self {
        Self(secret)
    }
}

impl SkimItem for SkimSecret {
    fn display(&self, _: DisplayContext) -> AnsiString {
        self.0.name.clone().into()
    }

    fn text(&self) -> Cow<str> {
        (&self.0.name).into()
    }

    fn output(&self) -> Cow<str> {
        self.0.path.to_string_lossy()
    }
}

/// Select secret.
pub fn select_secret(secrets: &[Secret]) -> Option<&Secret> {
    // Return if theres just one to choose
    if secrets.len() == 1 {
        return secrets.get(0);
    }

    // Let user select secret
    let items = skim_secret_items(secrets);
    let selected = skim_select(items, "Select secret")?;

    // Pick selected item from secrets list
    let path: PathBuf = selected.into();
    Some(secrets.iter().find(|e| e.path == path).unwrap())
}

/// Select key.
pub fn select_key<'a>(keys: &'a [Key], prompt: Option<&'a str>) -> Option<&'a Key> {
    // Let user select secret
    let items = skim_key_items(keys);
    let selected = skim_select(items, prompt.unwrap_or("Select key"))?;

    // Pick selected item from keys list
    Some(
        keys.iter()
            .find(|e| e.fingerprint(false) == selected)
            .unwrap(),
    )
}

/// Generate skim `SkimSecret` items from given secrets.
fn skim_secret_items(secrets: &[Secret]) -> SkimItemReceiver {
    skim_items(
        secrets
            .iter()
            .cloned()
            .map(|e| e.into())
            .collect::<Vec<SkimSecret>>(),
    )
}

/// Generate skim `SkimSecret` items from given secrets.
fn skim_key_items(keys: &[Key]) -> SkimItemReceiver {
    skim_items(
        keys.iter()
            .cloned()
            .map(|e| e.into())
            .collect::<Vec<SkimKey>>(),
    )
}

/// Create `SkimItemReceiver` from given array.
fn skim_items<I: SkimItem>(items: Vec<I>) -> SkimItemReceiver {
    let (tx_item, rx_item): (SkimItemSender, SkimItemReceiver) =
        skim::prelude::bounded(items.len());

    items.into_iter().for_each(|g| {
        let _ = tx_item.send(Arc::new(g));
    });

    rx_item
}

/// Wrapped store key item for skim.
pub struct SkimKey(Key);

impl From<Key> for SkimKey {
    fn from(key: Key) -> Self {
        Self(key)
    }
}

impl SkimItem for SkimKey {
    fn display(&self, _: DisplayContext) -> AnsiString {
        format!("{}", self.0).into()
    }

    fn text(&self) -> Cow<str> {
        format!("{}", self.0).into()
    }

    fn output(&self) -> Cow<str> {
        self.0.fingerprint(false).into()
    }
}
