# Keybindings

Bindings are fixed. There is no user remapping.

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `KEY-1` | Every binding is listed in the table below, and no other key is bound. | done | `spec_tests::keybindings_match_spec`, `spec_tests::every_bound_key_is_in_spec` |
| `KEY-2` | Rows marked `planned` are not bound in the code yet. | done | `spec_tests::keybindings_match_spec` |
| `KEY-3` | While the commit bar takes text (`COMMIT-3`), every printable key is text. Only `Enter`, `Esc`, `Backspace` and `Ctrl-c` keep their meaning. | done | `events::tests::translate_key_input_*` |
| `KEY-4` | An unbound printable key reaches the mode handler as `Char`, so the confirm dialog can read `y`, `Y`, `n` and `N`. A mode that does not use it ignores it. | done | `events::tests::translate_key_confirm_shortcuts_fall_through_to_char` |
| `KEY-5` | `Ctrl` with any key other than `c` or `q` does nothing, unless the table binds it. | done | `events::tests::translate_key_ctrl_other_is_none` |

## Default bindings

Keys are `Ctrl-x` for a control chord, a name (`Enter`, `Esc`, `Tab`, `Backspace`, `Up`, `Down`) or the character itself. The event is the `AppEvent` variant the key produces. What it does depends on the mode and the focused panel; the other spec files say how.

<!-- spec:keybindings:begin -->
| Keys | Event | Action | Status |
|------|-------|--------|--------|
| `q`, `Ctrl-c`, `Ctrl-q` | `Quit` | quit; close an overlay | done |
| `j`, `Down` | `MoveDown` | move down | done |
| `k`, `Up` | `MoveUp` | move up | done |
| `n` | `NextHunk` | next hunk or conflict | done |
| `p` | `PrevHunk` | previous hunk or conflict | done |
| `Tab` | `ToggleFocus` | switch panel | done |
| `s` | `Stage` | stage file or hunk | done |
| `u` | `Unstage` | unstage file or hunk | done |
| `d` | `Discard` | discard hunk | done |
| `X` | `DeleteUntracked` | delete untracked file | done |
| `o` | `AcceptOurs` | resolve conflict: ours | done |
| `i` | `AcceptTheirs` | resolve conflict: incoming | done |
| `b` | `AcceptBoth` | resolve conflict: both | done |
| `c` | `Commit` | open the commit bar | done |
| `P` | `Push` | push | done |
| `L` | `Pull` | pull | done |
| `t` | `OpenThemePicker` | theme picker | done |
| `?` | `OpenHelp` | help overlay | done |
| `r` | `Refresh` | refresh | done |
| `Enter` | `Confirm` | confirm; next field | done |
| `Esc` | `Cancel` | cancel; close | done |
| `Backspace` | `Backspace` | delete a character | done |
| `l` | `OpenLog` | commit log (`LOG-1`) | planned |
| `/` | `FilterFiles` | filter files (`VIEW-11`) | planned |
| `v` | `VisualMode` | line selection (`STAGE-7`) | planned |
| `S` | `UnstageLines` | unstage selected lines (`STAGE-10`) | planned |
| `A` | `ToggleAmend` | amend mode (`COMMIT-7`) | planned |
| `U` | `Undo` | undo (`UNDO-2`) | planned |
| `Ctrl-r` | `Redo` | redo (`UNDO-2`) | planned |
<!-- spec:keybindings:end -->

## Implementation notes

- To add a binding: edit the table, `events::translate_key`, the help overlay in `ui/mod.rs`, the man page `data/gitish.1` (`INT-17`), the bar hints in `ui/commit_bar.rs` if they apply, and `README.md`. The spec tests fail until the table and the code agree. Flip the row from `planned` to `done` in the same commit.
- `s` in line-selection mode (`STAGE-9`) is the existing `Stage` event: the mode decides what it does.
- Bindings live in `events.rs`, not a separate `keybinds.rs` (`QUAL-3`).
