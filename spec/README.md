# gitish: specification

This folder is the source of truth for what gitish does. The code implements the spec. If the code and the spec disagree, the code is wrong, or the spec must be changed first.

## Files

| File | Prefix | Covers |
|------|--------|--------|
| [product.md](product.md) | `PROD` | Goals, non-goals, stack constraints |
| [view.md](view.md) | `VIEW` | The panels, the status bar, the help overlay, file filtering |
| [staging.md](staging.md) | `STAGE` | Stage, unstage, discard and delete: files, hunks and lines |
| [commit.md](commit.md) | `COMMIT` | The commit composer, amend, message templates |
| [merge.md](merge.md) | `MERGE` | Merge-conflict resolution |
| [remote.md](remote.md) | `REMOTE` | Push and pull |
| [log.md](log.md) | `LOG` | The commit log |
| [undo.md](undo.md) | `UNDO` | Undo and redo of staging actions |
| [watch.md](watch.md) | `WATCH` | Automatic refresh when files change on disk |
| [keybindings.md](keybindings.md) | `KEY` | Default bindings |
| [appearance.md](appearance.md) | `LOOK` | Themes, transparency, icons, redraws |
| [configuration.md](configuration.md) | `CONF` | `config.toml` |
| [integration.md](integration.md) | `INT` | Command line, docs, CI, packaging |
| [quality.md](quality.md) | `QUAL` | Code-quality items that change no behaviour |
| [roadmap.md](roadmap.md) | — | The order of planned work, which `/implement` follows |

Add a file with a new prefix when a feature area doesn't fit these, and list it here.

## Requirement format

Each requirement is one line in a table:

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `STAGE-1` | One sentence that can be checked. | done | `git::stage::tests::name` |

- **IDs are permanent.** Do not renumber them or reuse one. When a requirement is dropped, keep its row, set the status to `dropped`, and give the reason.
- **Status** is one of:
  - `planned`: agreed but not built.
  - `partial`: built, but with a known gap. The gap is written in the row.
  - `done`: built and verified.
  - `dropped`: no longer wanted.
- Write behaviour, not implementation. Keep the "how" short, and put it under **Implementation notes** when it is not obvious or a past attempt failed.
- **Verified by** names the test, or says `manual` with what to check. Manual checks run in a detached tmux session (see `CLAUDE.md`).
- Rows that came from a GitHub issue end with `(#N)`. Closed issues are history and are not listed.

## Machine-checked sections

Tables between `<!-- spec:NAME:begin -->` and `<!-- spec:NAME:end -->` markers are parsed by `src/spec_tests.rs`. `cargo test` (and so CI) fails when they disagree with the code. Keep their column layout unchanged.

Both tables have a **Status** column. A `done` row must match the code. A `planned` row must *not* exist in the code yet, so the test fails when someone builds it without flipping the status.
