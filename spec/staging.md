# Staging

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `STAGE-1` | `s` in the file panel stages the whole selected file. `u` unstages it. A file deleted from the working tree is staged as a deletion. | done | `git::stage::tests::stage_and_unstage_file`, `stage_deleted_file_*` |
| `STAGE-2` | `s` in the diff panel stages the selected unstaged hunk. On a staged hunk it says "Hunk is already staged" and changes nothing. | done | `git::stage::tests::stage_hunk_applies_to_index`; manual for the message |
| `STAGE-3` | `u` in the diff panel unstages the selected staged hunk. On an unstaged hunk it says "Hunk is not staged" and changes nothing. | done | `git::stage::tests::unstage_hunk_restores_index`; manual for the message |
| `STAGE-4` | `d` discards the selected unstaged hunk from the working tree. On a staged hunk it says "Unstage the hunk first to discard it". | done | manual (no unit test for `discard_hunk` yet) |
| `STAGE-5` | `X` on an untracked file asks "Delete untracked file '…'?". `Enter`, `y` or `Y` deletes it. `Esc`, `n` or `N` cancels. On any other file it says "Not an untracked file". | done | `git::stage::tests::delete_untracked_removes_file`; manual for the dialog |
| `STAGE-6` | After a stage, unstage, discard or delete, the lists reload and the hunk cursor stays at its index, or on the last hunk if there are fewer. | done | `app::tests::refresh_preserves_hunk_cursor_in_bounds`, `refresh_clamps_hunk_cursor_when_out_of_bounds`, `refresh_resets_hunk_cursor_when_no_hunks` |
| `STAGE-7` | `v` in the diff panel enters line-selection mode at the first changed line of the selected hunk. `Esc` leaves it. | planned | manual (#24) |
| `STAGE-8` | In line-selection mode `j`/`k` extend the selection across the `+` and `-` lines of the current hunk, and never beyond it. The selection is highlighted. | planned | unit test on selection bounds; manual (#24) |
| `STAGE-9` | In line-selection mode `s` stages only the selected lines, by building a partial patch and applying it to the index. | planned | `git::stage::tests` (#24) |
| `STAGE-10` | In line-selection mode on a staged hunk, `S` unstages only the selected lines. | planned | `git::stage::tests` (#24) |
| `STAGE-11` | The partial patch includes the context lines it needs, and applies cleanly for any selection of changed lines, including a selection that is only `-` lines or only `+` lines. | planned | `git::stage::tests` (#24) |

## Implementation notes

- `unstage_hunk` pipes the staged patch to `git apply --cached -R` and lets git do the reversal. Reversing by hand drifts line numbers in files with several hunks.
- The partial-patch builder for `STAGE-9`…`STAGE-11` belongs in `git/stage.rs` next to `build_patch`, so it stays stateless and unit-testable. An unselected `-` line becomes context. An unselected `+` line is left out.
- Undo of these actions is specified in [undo.md](undo.md).
