# View

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `VIEW-1` | The screen has a file panel on the left (30% width), a diff panel on the right (70%), a three-row commit bar and a one-row status bar at the bottom. | done | manual: `scripts/manual/smoke.sh` |
| `VIEW-2` | The file panel lists every changed file, sorted by path. Each row has a staging marker (staged, partially staged, unstaged) and a type marker (untracked, new, modified, deleted, conflicted). | done | `git::repo::tests`, `glyphs::tests`; manual: `scripts/manual/smoke.sh` |
| `VIEW-3` | `Tab` moves focus between the file panel and the diff panel. The focused panel has a highlighted border. | done | manual: `scripts/manual/smoke.sh` |
| `VIEW-4` | The diff panel shows the selected file's staged hunks under a "Staged" header and its unstaged hunks under an "Unstaged" header. One cursor moves through both lists, staged first. The title shows the file name and the cursor position. | done | `ui_helpers::tests::hunk_counter_*`; manual: `scripts/manual/smoke.sh` |
| `VIEW-5` | With no file selected the diff panel says "No File Selected". With a file that has no hunks it says "No changes". | done | manual |
| `VIEW-6` | Moving the file cursor loads that file's diff and puts the hunk cursor on the first hunk. | done | manual: `scripts/manual/smoke.sh` |
| `VIEW-7` | If a diff cannot be loaded, the status bar says "Failed to load diff: …" and the diff panel is empty. The error is never swallowed. | done | `app::tests::reload_hunks_surfaces_diff_error_in_status_msg`, `load_hunks_for_propagates_diff_error` |
| `VIEW-8` | The status bar shows the result of the last action. The next key press clears it. | done | manual |
| `VIEW-9` | `r` reloads the file list and diffs, keeps the file cursor in range, and says "Refreshed". | done | `app::tests::refresh_*` |
| `VIEW-10` | `?` opens a help overlay that lists every binding. `j`/`k` scroll it, the scroll is clamped to the content, and `?`, `Esc` or `q` close it. | done | `app::tests::help_*`, `ui_helpers::tests::help_max_scroll_*` |
| `VIEW-11` | `/` in the file panel opens a filter input at the top of the panel. | planned | manual (#22) |
| `VIEW-12` | While the filter is open, typing narrows the file list in real time to files whose path contains the text, ignoring case. | planned | unit test on the matcher (#22) |
| `VIEW-13` | The file cursor stays on a visible file as the filter narrows. If the selected file is filtered out, the cursor moves to the first match. | planned | unit test (#22) |
| `VIEW-14` | `Enter` closes the filter input and keeps the filter applied. `Esc` clears the filter and restores the full list. | planned | unit test (#22) |
| `VIEW-15` | While a filter is applied, the file panel title shows it. | planned | manual (#22) |

## Implementation notes

- Moving the cursor to a file reloads its diff, so the filter (`VIEW-11`…`VIEW-15`) must reload it when the selected file changes.
