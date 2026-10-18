//! Checks the machine-readable tables in `spec/` against the code, so the spec
//! can't silently drift. See spec/README.md.

use std::collections::BTreeSet;

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

use crate::config::Preferences;
use crate::events::{translate_key, AppEvent};

const KEYBINDINGS_MD: &str = include_str!("../spec/keybindings.md");
const CONFIGURATION_MD: &str = include_str!("../spec/configuration.md");
const README_MD: &str = include_str!("../README.md");

/// Body rows of the table between `<!-- spec:NAME:begin -->` and `...:end -->`.
fn table(doc: &str, name: &str) -> Vec<Vec<String>> {
    let begin = format!("<!-- spec:{name}:begin -->");
    let end = format!("<!-- spec:{name}:end -->");
    let start = doc
        .find(&begin)
        .unwrap_or_else(|| panic!("missing {begin}"))
        + begin.len();
    let stop = doc[start..]
        .find(&end)
        .unwrap_or_else(|| panic!("missing {end}"))
        + start;
    doc[start..stop]
        .lines()
        .map(str::trim)
        .filter(|l| l.starts_with('|'))
        .skip(2)
        .map(|l| {
            l.trim_matches('|')
                .split('|')
                .map(|c| c.trim().to_owned())
                .collect()
        })
        .collect()
}

/// "`a`, `b`" -> ["a", "b"]
fn code_list(cell: &str) -> Vec<String> {
    cell.split(',')
        .map(|s| s.trim().trim_matches('`').to_owned())
        .filter(|s| !s.is_empty())
        .collect()
}

fn code(cell: &str) -> &str {
    cell.trim_matches('`')
}

fn parse_key(name: &str) -> KeyEvent {
    let plain = |code| KeyEvent::new(code, KeyModifiers::NONE);
    if let Some(c) = name.strip_prefix("Ctrl-") {
        let ch = c.chars().next().expect("Ctrl- needs a character");
        return KeyEvent::new(KeyCode::Char(ch), KeyModifiers::CONTROL);
    }
    match name {
        "Enter" => plain(KeyCode::Enter),
        "Esc" => plain(KeyCode::Esc),
        "Tab" => plain(KeyCode::Tab),
        "Backspace" => plain(KeyCode::Backspace),
        "Up" => plain(KeyCode::Up),
        "Down" => plain(KeyCode::Down),
        s if s.chars().count() == 1 => plain(KeyCode::Char(s.chars().next().unwrap())),
        other => panic!("spec names a key the test can't parse: {other}"),
    }
}

/// The event a key produces, ignoring the `Char` fallthrough for unbound keys.
fn bound_event(key: KeyEvent) -> Option<AppEvent> {
    translate_key(key).filter(|e| !matches!(e, AppEvent::Char(_)))
}

fn status_ok(status: &str) {
    assert!(
        status == "done" || status == "planned",
        "machine-checked tables take only `done` or `planned`, got {status:?}"
    );
}

#[test]
fn keybindings_match_spec() {
    let rows = table(KEYBINDINGS_MD, "keybindings");
    assert!(!rows.is_empty(), "keybindings table is empty");
    for row in rows {
        let (keys, event, status) = (code_list(&row[0]), code(&row[1]), row[3].as_str());
        status_ok(status);
        for k in keys {
            let got = bound_event(parse_key(&k));
            match status {
                "done" => assert_eq!(
                    got.as_ref().map(|e| format!("{e:?}")).as_deref(),
                    Some(event),
                    "key {k}: code vs spec/keybindings.md"
                ),
                _ => assert_eq!(
                    got, None,
                    "key {k} is bound in the code but the spec says `planned`: flip the row to `done`"
                ),
            }
        }
    }
}

#[test]
fn every_bound_key_is_in_spec() {
    let documented: BTreeSet<String> = table(KEYBINDINGS_MD, "keybindings")
        .iter()
        .flat_map(|r| code_list(&r[0]))
        .collect();

    let mut candidates: Vec<String> = (0x20u8..0x7f).map(|b| (b as char).to_string()).collect();
    candidates.extend(
        ["Enter", "Esc", "Tab", "Backspace", "Up", "Down"].map(String::from),
    );
    candidates.extend(('a'..='z').map(|c| format!("Ctrl-{c}")));

    for k in candidates {
        if bound_event(parse_key(&k)).is_some() {
            assert!(
                documented.contains(&k),
                "key {k} is bound in the code but missing from spec/keybindings.md"
            );
        }
    }
}

/// Every key `Preferences` writes, with its value as TOML text. `theme` is
/// `Option`, so it only shows up when set.
fn preference_keys() -> Vec<(String, String)> {
    let prefs = Preferences {
        theme: Some("x".into()),
        ..Preferences::default()
    };
    let text = toml::to_string(&prefs).expect("serialize preferences");
    let parsed: toml::Table = toml::from_str(&text).expect("parse preferences");
    parsed.into_iter().map(|(k, v)| (k, v.to_string())).collect()
}

#[test]
fn config_defaults_match_spec() {
    let in_code = preference_keys();
    let mut done = BTreeSet::new();
    for row in table(CONFIGURATION_MD, "config") {
        let (key, default, status) = (code(&row[0]), code(&row[1]), row[2].as_str());
        status_ok(status);
        let found = in_code.iter().find(|(k, _)| k == key);
        match status {
            "done" => {
                let (_, value) = found.unwrap_or_else(|| panic!("spec lists `{key}`, code lacks it"));
                done.insert(key.to_owned());
                if default != "—" {
                    assert_eq!(value, default, "`{key}`: Preferences::default() vs spec");
                }
            }
            _ => assert!(
                found.is_none(),
                "`{key}` exists in the code but the spec says `planned`: flip the row to `done`"
            ),
        }
    }
    let code_keys: BTreeSet<_> = in_code.into_iter().map(|(k, _)| k).collect();
    assert_eq!(done, code_keys, "spec/configuration.md must list every config key");
}

#[test]
fn default_for_unset_theme_is_absent() {
    let text = toml::to_string(&Preferences::default()).expect("serialize preferences");
    assert!(!text.contains("theme"), "the spec says `theme` has no default");
}

#[test]
fn readme_lists_every_config_key() {
    for row in table(CONFIGURATION_MD, "config") {
        if row[2] == "done" {
            let key = code(&row[0]);
            assert!(
                README_MD.contains(&format!("`{key}`")),
                "README.md does not mention config key `{key}`"
            );
        }
    }
}
