# Configuration

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `CONF-1` | Settings are read from `<config dir>/gitish/config.toml` (`dirs::config_dir`, so `$XDG_CONFIG_HOME` on Linux) when the app starts. A missing file is not an error. Every key is optional. | done | `config::tests::load_returns_default_when_file_missing`, `load_defaults_nerd_fonts_true_when_field_absent`, `config_dir_returns_gitish_subdir` |
| `CONF-2` | A file that cannot be parsed is copied to `config.toml.bak`, the defaults are used, and the status bar shows the error. The message is never printed to stderr, which the alternate screen hides. | done | `config::tests::load_returns_default_and_error_on_invalid_toml`, `load_backs_up_corrupt_config`, `app::tests::corrupt_config_surfaces_error_in_status_msg` |
| `CONF-3` | Saving writes a temporary file and renames it over `config.toml`, so a crash never leaves a half-written file. | done | `config::tests::save_then_load_roundtrip` (the rename itself: manual) |
| `CONF-4` | The theme picker is the only thing in the app that writes `config.toml`. | done | manual: grep for `Preferences::save` |
| `CONF-5` | `README.md` lists every key in the table below. | done | `spec_tests::readme_lists_every_config_key` |

## Keys

<!-- spec:config:begin -->
| Key | Default | Status |
|-----|---------|--------|
| `theme` | — | done |
| `transparent` | `false` | done |
| `use_nerd_fonts` | `true` | done |
| `watch` | `true` | planned |
| `block_amend_pushed` | `true` | planned |
<!-- spec:config:end -->

`theme` has no default in the file: when it is missing the first theme in the picker is used, "Catppuccin Mocha" with the seeded themes. Planned keys are described in `WATCH-5` and `COMMIT-11`.

## Implementation notes

- To add a key: edit `Preferences`, the table above, `README.md`, the man page `data/gitish.1` (`INT-17`), the `--help` text in `args.rs`, and flip its row in the same commit. `spec_tests` fails until they agree.
- `apply_theme` and `toggle_transparent` build a full `Preferences` by hand. Adding a key means updating both, or the save drops the new key.
