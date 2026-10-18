# Integration

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `INT-1` | `gitish` opens the repository found from the current directory, searching upwards. `--path <path>` searches from `<path>` instead. | done | `args::tests::path_flag_sets_path`; manual |
| `INT-2` | With no repository, the app prints "No git repository found in '…'" with a hint, and exits before the screen is taken over. | done | manual |
| `INT-3` | `--open theme-picker` starts with the theme picker open. `--open spinner` starts with the loading overlay showing, for screenshots. | done | `args::tests::open_theme_picker_sets_mode`; manual |
| `INT-4` | `--help`, `-h` and `-?` print the help and exit. | done | `args::tests::help_*` |
| `INT-5` | An unknown option, a missing value or an unknown `--open` state is an error that names the problem and points to `--help`. | done | `args::tests::unknown_flag_is_error`, `path_flag_missing_value_is_error`, `open_*` |
| `INT-6` | CI runs `cargo test` on every pull request and push to `main`, and fails if line coverage is below 80%, not counting `src/ui/` and `src/main.rs`. | done | `.github/workflows/ci.yml` |
| `INT-7` | `nix build` builds the package and `nix develop` gives a shell with the build dependencies, on Linux and macOS. | done | manual: `nix build` (macOS not checked) |
| `INT-8` | `README.md`, the in-app help and `--help` list every binding, option and config key. | done | partly: `spec_tests::readme_lists_every_config_key`; the rest manual |
| `INT-9` | `scripts/tui-screenshot.sh` renders a PNG of the running UI in a detached tmux pane. | done | manual |
| `INT-10` | A pushed `v*` tag runs `.github/workflows/release.yml`, which builds release binaries for `x86_64-unknown-linux-gnu`, `aarch64-unknown-linux-gnu`, `x86_64-apple-darwin` and `aarch64-apple-darwin` and attaches them to the GitHub release. | planned | manual: push a test tag (#27) |
| `INT-11` | Each archive holds the `gitish` binary, `LICENSE` and `README.md`. SHA256 checksums are published next to the archives. | planned | manual (#27) |
| `INT-12` | A release updates the `westongpt/homebrew-tap` formula to the new tag and checksum, by `brew bump-formula-pr` or a templated commit. | planned | manual (#27) |
| `INT-13` | `README.md` documents `brew install westongpt/tap/gitish`. | planned | manual (#27) |
| `INT-14` | The Nix flake either points at the same release artifacts or stays a source build, and `README.md` says which. | planned | manual (#27) |
| `INT-15` | A `LICENSE` file at the repo root holds the MIT license, "Copyright (c) 2026 DisposableSoftware", and `README.md` links to it. | done | manual: `LICENSE` exists, README links it |
| `INT-16` | `data/gitish.1` is a man page with NAME, SYNOPSIS, DESCRIPTION, OPTIONS, KEY BINDINGS, FILES and CONFIGURATION sections. | done | `spec_tests::man_page_matches_spec` |
| `INT-17` | The man page `.TH` line carries the `Cargo.toml` version. Its OPTIONS list every CLI option, its KEY BINDINGS list every `done` binding, and its CONFIGURATION lists every `done` config key. | done | `spec_tests::man_page_matches_spec` |
| `INT-18` | `data/io.github.DisposableSoftware.Gitish.metainfo.xml` is valid AppStream for a console application: id, name, summary, description, `CC0-1.0` metadata license, `MIT` project license, project URLs, `<provides><binary>gitish</binary></provides>`, and a `<release>` for the current version. | done | `spec_tests::metainfo_matches_package`; `appstreamcli validate` passed |
| `INT-19` | `data/io.github.DisposableSoftware.Gitish.desktop` has `Terminal=true`, `Exec=gitish`, categories `Development;RevisionControl;ConsoleOnly;` and no icon. | done | `spec_tests::desktop_entry_matches_package`; `desktop-file-validate` passed |
| `INT-20` | `nix build` installs the binary, the man page, the metainfo, the desktop entry, `LICENSE` and `README.md` under their standard prefixes. | done | `nix build` output lists all six paths |

## Implementation notes

- #27 needs a repository `LICENSE`, which does not exist yet, and a Homebrew tap repository, which is outside this repo. Both are decisions for the owner. The Cargo version and the flake version (`0.1.0` in both) must move together; add a release script in the style of the other apps' `scripts/release.sh` when #27 is built.
- Release notes carry the same tongue-in-cheek "vibe-coded slop" framing as the other DisposableSoftware apps.
- `INT-15`…`INT-20` are the packaging files #27 and the release script need. The version in `Cargo.toml`, `flake.nix`, the man page and the metainfo must agree; `INT-17` and `INT-18` check the last two against `Cargo.toml`, and the flake should read the version from `Cargo.toml` instead of repeating `0.1.0`.
- No icon yet (`INT-19`): a TUI needs none, and a desktop entry without one is valid. Add one by a later row.
- NixOS and home-manager modules are not planned. gitish has no daemon and its config is a file the user edits.
