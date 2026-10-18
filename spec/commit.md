# Commit

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `COMMIT-1` | `c` opens the commit bar for a title. `Enter` moves to the optional body. `Enter` in the body commits what is staged. | done | manual: `scripts/manual/smoke.sh` |
| `COMMIT-2` | `Esc` in the title input cancels the commit and clears the title. `Esc` in the body input returns to normal mode and keeps the title. | done | manual |
| `COMMIT-3` | In both inputs every printable key is text, so `q`, `s` and the other bindings type letters. `Ctrl-c` still quits. | done | `events::tests::translate_key_input_*` |
| `COMMIT-4` | A commit with an empty title (blank counts as empty) is refused with "Commit title cannot be empty". | done | manual (no unit test on `App::do_commit`) |
| `COMMIT-5` | The message is the title, or the title, a blank line and the trimmed body when the body is not blank. | done | `git::commit::tests::commit_with_title_only`, `commit_with_body` |
| `COMMIT-6` | After a commit the title and body are cleared, the status bar shows "Committed " and the 7-character id, and the lists reload. | done | manual: `scripts/manual/smoke.sh` (status text only) |
| `COMMIT-7` | `A` in normal mode turns amend mode on or off. Turning it on fills the title and body from `HEAD` and opens the commit bar. | planned | unit test (#25) |
| `COMMIT-8` | In amend mode the commit bar title says so. | planned | manual (#25) |
| `COMMIT-9` | Committing in amend mode replaces `HEAD` with a commit that has the new message and the current index, and keeps `HEAD`'s parents. It does not add a commit. | planned | `git::commit::tests` (#25) |
| `COMMIT-10` | When a new commit starts, the body is filled from the first template found: `.gitmessage` in the repo root, then `$XDG_CONFIG_HOME/gitish/template.txt`. With neither, it starts empty. | planned | `git::commit::tests` for the loader (#25) |
| `COMMIT-11` | Amend is refused with an error that says why when `HEAD` is already on the upstream branch. `block_amend_pushed = false` turns the check off. This is best effort: with no upstream, amend is allowed. | planned | `git::commit::tests` (#25) |

## Implementation notes

- The body is a single line today, so the commit bar cannot hold a multi-line template or a multi-line `HEAD` body. `COMMIT-7` and `COMMIT-10` need a multi-line body input, or must define how newlines show in one line. This is a decision for `/implement` to put to the user.
- `block_amend_pushed` is a name proposed here for the "configurable" part of #25. Rename it before building if you prefer another.
