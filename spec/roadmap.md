# Roadmap

The order of approved work: `planned` requirements, and `partial` ones with a known bug from `/bug`. Change it here when priorities change. `/implement` works through this file: the verification backlog first, then the requirements in the order listed.

1. `QUAL-4` — clippy clean, then run it in CI
2. `QUAL-2` — comment on the `RefCell` (#36)
3. `QUAL-1` — one argument parser (#37)
4. `LOG-1`…`LOG-3` — commit log (#18)
5. `VIEW-11`…`VIEW-15` — file filter (#22)
6. `STAGE-7`…`STAGE-11` — line-level staging (#24)
7. `UNDO-1`…`UNDO-6` — undo and redo (#23)
8. `COMMIT-7`…`COMMIT-11` — amend and message templates (#25)
9. `WATCH-1`…`WATCH-6` — file watcher (#26)
10. `INT-15`…`INT-20` — LICENSE, man page, metainfo, desktop entry, Nix install
11. `INT-10`…`INT-14` — release binaries and Homebrew (#27)

## Verification backlog

Built requirements that are not fully verified. Clear these before starting new planned work. These came from writing the spec for code that already existed: the rows are `done` because the behaviour is in the code, but the check named is manual.

- `VIEW-5`, `VIEW-8`: manual only.
- `STAGE-4`: `discard_hunk` has no unit test.
- `COMMIT-4`: no unit test on `App::do_commit`'s empty-title refusal.
- `COMMIT-2`, `COMMIT-6`: `smoke.sh` covers the happy path of `COMMIT-1` and the "Committed" text only; cancelling and the commit id are unchecked.
- `MERGE-3`, `MERGE-5`: manual with `scripts/make-conflict-repo.sh`.
- `LOOK-5`, `LOOK-8`: need a terminal with transparency, and an exit-path check.
- `CONF-3`, `CONF-4`: the atomic rename and the single writer are read from the code, not tested.
- `INT-2`, `INT-7`, `INT-8`, `INT-9`: manual.
