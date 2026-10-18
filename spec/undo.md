# Undo

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `UNDO-1` | The app records each stage, unstage and discard (files, hunks and lines) in an in-memory history. | planned | unit test on the action stack (#23) |
| `UNDO-2` | `U` undoes the most recent recorded action. `Ctrl-r` redoes the most recently undone one. | planned | unit test (#23) |
| `UNDO-3` | The history keeps the last 50 actions. A successful commit clears it. A new action clears the redo stack. | planned | unit test (#23) |
| `UNDO-4` | Before a discard or an untracked-file delete, the content is saved to a per-session temporary directory, so undo restores it. The directory is removed when the app exits. | planned | unit test; manual (#23) |
| `UNDO-5` | Undo or redo with nothing to undo or redo says so in the status bar and changes nothing. | planned | unit test (#23) |
| `UNDO-6` | Each undo and redo reports what it did in the status bar, for example "Undid: staged hunk in src/app.rs". | planned | manual (#23) |

## Implementation notes

- #23 asks for `u` to undo. `u` already unstages (`STAGE-1`, `STAGE-3`), so this spec uses `U`. `Ctrl-r` is as in the issue. Say if you want `u` rebound instead: that changes `STAGE-1` and `STAGE-3` and counts as BREAKING.
- Whether deleting an untracked file (`X`) is undoable is not in the issue. `UNDO-4` includes it because it is the other destructive action; drop that part if you don't want it.
