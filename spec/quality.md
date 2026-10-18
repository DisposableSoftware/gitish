# Quality

Items that change no behaviour. They have IDs so `/implement` can track them, but they follow the refactor rule in `CLAUDE.md`: no behaviour spec edit, and the tests must still pass.

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `QUAL-1` | Argument parsing has one implementation, `parse_from(&[String])`. The production entry point passes `std::env::args()` to it and the tests call it directly. The test-only copy of the parser is gone, and nothing outside the entry point reads `std::env`. | planned | `args::tests`; `grep -n "env::" src/args.rs` shows one use (#37) |
| `QUAL-2` | The `RefCell<Vec<Hunk>>` in `parse_diff_hunks` has a comment saying why: `Diff::foreach` takes several `FnMut` closures that all push to the same `Vec`. | done | read `src/git/repo.rs` (#36) |
| `QUAL-3` | The code layout in `CLAUDE.md` matches `src/`. The decision on `keybinds.rs` is written down: bindings stay in `events.rs`, because `spec_tests` checks them there. | done | manual: compare `CLAUDE.md` with `git ls-files src` (#35) |
| `QUAL-4` | `cargo clippy --all-targets -- -D warnings` passes, and CI runs it. | done | `cargo clippy --all-targets -- -D warnings` clean; CI step `Clippy` |

