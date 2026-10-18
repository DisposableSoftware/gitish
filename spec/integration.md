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

## Implementation notes

- #27 needs a repository `LICENSE`, which does not exist yet, and a Homebrew tap repository, which is outside this repo. Both are decisions for the owner. The Cargo version and the flake version (`0.1.0` in both) must move together; add a release script in the style of the other apps' `scripts/release.sh` when #27 is built.
- Release notes carry the same tongue-in-cheek "vibe-coded slop" framing as the other DisposableSoftware apps.
