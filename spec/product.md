# Product

gitish is a terminal git staging UI: `git add -p`, but interactive. Binary `gitish`.

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `PROD-1` | The user can browse changed files and their hunks, stage, unstage and discard changes, write a commit message and commit, without leaving the terminal. | done | see `VIEW`, `STAGE`, `COMMIT` |
| `PROD-2` | The app is synchronous. Slow work (push, pull) runs on a worker thread while the UI keeps drawing. There is no async runtime. | done (ongoing constraint) | `app::tests` (`spawn_remote_worker_*`, `finish_remote_op_*`) |
| `PROD-3` | Every user-facing change updates `README.md` and the in-app help (`INT-8`). | done (ongoing constraint) | `spec_tests::readme_lists_every_config_key`, manual |

## Non-goals

- Not a full git client: no branch, rebase, stash or remote management.
- No mouse support. The UI is keyboard-driven.

## Stack constraints

- Rust (edition 2024), `ratatui` + `crossterm` for the UI, `git2` for git.
- Error handling: `anyhow` for application errors, `thiserror` for library-facing errors. No `unwrap()` or `expect()` outside tests.
- Config: `serde` + `toml`, paths from `dirs`.
- Ask before adding a crate. Planned work that needs one: `notify` (`WATCH`).
- `git/` is stateless and holds all git access. Where libgit2 cannot do the job it runs the `git` binary: push, pull and `unstage_hunk` do.
