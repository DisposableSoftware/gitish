# Watch

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `WATCH-1` | A change to a file in the working tree triggers a reload of the file list and diffs after a debounce of about 250 ms. Changes inside `.git/` are ignored, except `.git/index` and `.git/HEAD`. | planned | manual (#26) |
| `WATCH-2` | A change to `.git/index` or `.git/HEAD` triggers the same reload, so staging from another terminal shows up. | planned | manual (#26) |
| `WATCH-3` | A reload keeps the selected file when it still exists. If it is gone, the cursor moves to the nearest remaining file. A new file appears in sorted order without moving the cursor off the selected file. | planned | unit test on the merge-state function (#26) |
| `WATCH-4` | A reload does not flicker or scroll the diff to the top. The hunk cursor is kept as in `STAGE-6`. | planned | manual (#26) |
| `WATCH-5` | `watch = false` in `config.toml` turns the watcher off. It is on by default. | planned | `spec_tests::config_defaults_match_spec` (#26) |
| `WATCH-6` | The reload runs off the UI thread. The UI never blocks on it. | planned | manual (#26) |

## Implementation notes

- Needs the `notify` crate: ask before adding it (see `product.md`).
- The merge-state function is pure: old file list, old selection and new file list in, new selection out. That makes `WATCH-3` unit-testable without a watcher.
- Manual `r` (`VIEW-9`) stays, for when the watcher is off.
