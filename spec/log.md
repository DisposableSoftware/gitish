# Commit log

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `LOG-1` | `l` opens an overlay that lists the last 20 commits of the current branch, newest first, one row each: short id, title, author and relative date. | planned | `git::repo::tests` for the loader; manual (#18) |
| `LOG-2` | In the overlay `j`/`k` move the selection and scroll the list. `Esc`, `q` or `l` close it. | planned | manual (#18) |
| `LOG-3` | On a repository with no commits the overlay says "No commits yet" and does not fail. | planned | `git::repo::tests` (#18) |

## Implementation notes

- #18 asks for "a button to display a window with a list of the last few commits". In a terminal that is an overlay. The key `l`, the count 20 and the columns are proposals: change them before building if you want others.
- An overlay needs `needs_clear` on open and close (`LOOK-7`).
