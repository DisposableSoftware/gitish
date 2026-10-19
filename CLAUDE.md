# gitish

A terminal git staging UI in Rust: `git add -p`, but interactive. Binary `gitish`, crate `gitish`.

**What the app does is defined in [`spec/`](spec/README.md).** This file covers how to work on it.

## Spec-first workflow (mandatory)

Every change follows these steps, in order. Three skills run them. `/feature` (`.claude/skills/feature/`) turns a request into spec rows, and `/bug` (`.claude/skills/bug/`) reproduces a bug and finds the spec gap that let it happen. Both end by asking the user to **approve and implement** or **approve the spec only**, and neither writes code itself. `/implement` (`.claude/skills/implement/`) builds what is approved: it works through `spec/roadmap.md`, the verification backlog first and then the listed rows in order, with one commit per requirement.

1. **Spec.** Before touching code, update the relevant `spec/*.md`.
   - **New behaviour or a changed requirement:** add or edit a requirement row. A new row gets the next free ID in its prefix, with status `planned`. If the request is unclear, settle the wording with the user before coding.
   - **Bug fix:** find the requirement the bug breaks. If no requirement covers the behaviour, the bug is a gap in the spec, so add one.
   - **Refactor with no behaviour change:** no spec edit, but it must still pass step 3.
   - Commit the spec change on its own, before the code: `spec: <summary>`.
2. **Code.** Implement only what the spec says. If the code needs behaviour the spec doesn't describe, go back to step 1.
3. **Confirm against the spec.** Before the code commit:
   - Run `cargo test`. It includes `src/spec_tests.rs`, which checks the keybinding and config tables in the spec against the code.
   - For every requirement ID touched, verify it as its **Verified by** column says (unit test, or a manual check in tmux). Prefer adding a unit test where one is possible.
   - Re-read the changed code against the requirement text and look for behaviour the spec doesn't mention.
   - Update the row: set the status to `done` (or `partial` with the gap written in), and fill in **Verified by**. Update `roadmap.md` if a planned item landed.
4. **Commit** the code with a `Spec: ID, ID` trailer listing the requirements it implements or affects, and tell the user which IDs were verified and how.

When reporting to the user, say plainly if something could only be partly verified.

After a feature is complete, build the app and stop to ask the user for feedback before continuing to the next one.

## Commands

```bash
nix develop                                  # devshell: cargo, clippy, rust-analyzer, libgit2
cargo run                                    # run (inside devshell)
cargo clippy --all-targets -- -D warnings    # must pass
cargo fmt
cargo test                                   # unit tests + spec conformance tests
cargo llvm-cov --workspace --summary-only --fail-under-lines 80 --ignore-filename-regex 'src/ui/|src/main\.rs'   # what CI runs
nix build                                    # package -> ./result/bin/gitish
scripts/manual/smoke.sh                      # tmux check of the main flow (after cargo build)
```

The flake only sees git-tracked files. Run `git add` on new files before `nix build` or `nix develop`. After adding crates, commit `Cargo.lock`. `spec/` and `README.md` are part of the package source, because the spec tests `include_str!` them.

## Code layout

Keep this table current: one line per module, with the spec IDs it implements.

```
src/main.rs        entry point: parse args, open repo, terminal guard, run        INT-1…INT-3, LOOK-8
src/args.rs        --path, --open, --help                                         INT-1…INT-5
src/app.rs         App: all state, the run loop, one handler per Mode            VIEW STAGE COMMIT MERGE REMOTE LOOK-3…LOOK-7
src/events.rs      AppEvent and the key → event tables (the bindings live here)   KEY
src/config.rs      Preferences: config.toml load/save                             CONF
src/error.rs       AppError
src/theme.rs       base16 theme loading, ThemeList, fallback theme                LOOK-1…LOOK-3
src/seeds.rs       bundled Catppuccin YAML                                        LOOK-1
src/glyphs.rs      Nerd Font glyphs and their ASCII fallbacks                     LOOK-6
src/ui_helpers.rs  pure helpers for rendering (hunk counter, help scroll bound)   VIEW-4, VIEW-10
src/git/repo.rs    stateless: open repo, list files, parse diffs and conflicts    VIEW-2, VIEW-4, MERGE
src/git/stage.rs   stateless: stage, unstage, discard, delete, resolve conflicts  STAGE, MERGE-4
src/git/commit.rs  stateless: create a commit                                     COMMIT-5
src/git/remote.rs  stateless: push and pull via the git binary                    REMOTE
src/ui/mod.rs      draw(), the help overlay, the status bar                       VIEW-8, VIEW-10
src/ui/*.rs        file_panel, diff_panel, commit_bar, theme_picker,              VIEW LOOK
                   confirm_dialog, loading_overlay, layout: pure render functions
src/spec_tests.rs  spec ↔ code conformance tests
scripts/manual/    tmux checks (lib.sh helpers; smoke.sh)
scripts/           make-conflict-repo.sh, tui-screenshot.sh
spec/              the specification
```

## Architecture

A monolithic `App` over a stateless git layer. A Container/UseCase/Router/Gateway split was considered (issue #28) and deferred on purpose; don't reintroduce it without a spec change.

- `App` is the single source of truth for mutable state. It handles every key through `handle_normal`, `handle_commit_title`, `handle_commit_body`, `handle_theme_picker`, `handle_help` and `handle_confirming`.
- `git/` is pure functions: no state, no side effects beyond libgit2 and the `git` binary. It is the data layer.
- `ui/` is pure render functions: they take `&App` and mutate nothing.
- Bindings stay in `events.rs` (no separate `keybinds.rs`); `spec_tests` checks them there (`QUAL-3`).

## Coding conventions

- No `unwrap()` or `expect()` in non-test code: propagate with `?`.
- Small, focused functions. If a function needs a comment to explain what it does, split it.
- Comments only when the *why* is non-obvious (upstream quirk, hidden constraint, workaround).
- Unit tests for all `git/` logic. UI code does not need tests. Keep line coverage at 80% or more outside `src/ui/` and `src/main.rs`, and aim for more; don't test static values.
- Keep `main.rs` thin: initialise and hand off to `app.rs`.
- Synchronous code only. Slow work goes on a worker thread that reports through a channel (`PROD-2`).
- Ask before adding a crate.
- Commits: `spec: <summary>` for spec changes, a `Spec: ID, ID` trailer on code commits. No Claude-Session links in commit messages.

## Rules

- **Docs.** Keep `README.md`, the in-app help (`ui/mod.rs`) and `--help` (`args.rs`) current with every user-facing change (`INT-8`).
- **Screenshots.** When making a PR, remove a screenshot in `docs/screenshots/` if its ticket is closed. Keep the repo small.
- **UI redraws.** Set `app.needs_clear = true` (it triggers `terminal.clear()` before the next draw) when: (1) a floating overlay (Help, ThemePicker, Confirming, Loading) opens or closes; (2) the terminal is resized; (3) global visual properties change (theme, transparency). The run loop handles (1) and (2) through `is_overlay_mode()`. Operations that change global appearance set it themselves (`LOOK-7`).
- **Pull requests.** Changes land by pull request from a branch, not by pushing to `main`. Don't push or open a PR without being asked.

## Manual testing

Prefer adding a script to `scripts/manual/` (source `lib.sh`) over ad-hoc commands. Each script names the spec IDs it checks. Build first (`cargo build`), then run `scripts/manual/<name>.sh`.

- `lib.sh` runs gitish in a detached tmux session on a **private tmux server** (`tmux -L`), against a temporary repository and a temporary `XDG_CONFIG_HOME`. It never touches the user's terminal, tmux sessions, repositories or config. Keep it that way: never point a script at the user's repositories or `~/.config/gitish`.
- `check ID "text"` waits up to 3 seconds for text on the screen. `keys` sends keys (`Tab`, `Enter`, `Esc`, or literal text).
- For states that need a special repository, build one in `$WORK` (`make_repo` is the pattern). `scripts/make-conflict-repo.sh` makes one in a merge-conflict state.
- For a picture of a visual change, run `scripts/tui-screenshot.sh out.png` (needs `tmux`; it fetches `termshot` through nix) and name the path in the report.
