# Remote

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `REMOTE-1` | `P` runs `git push` and `L` runs `git pull` in the repository's working directory, on a worker thread. | done | `app::tests::spawn_remote_worker_sets_receiver` |
| `REMOTE-2` | While one runs, a loading overlay with a spinner and the operation's name covers the UI, and the UI keeps redrawing. Only quit is honoured. | done | `app::tests::loading_input_only_honours_quit`; manual: `gitish --open spinner` |
| `REMOTE-3` | When it ends, the status bar shows one line: "Push: …", "Push failed: …", "Pull: …" or "Pull failed: …". On failure the line is the first one git marks as an error, else the last line. | done | `git::remote::tests::summarize_*`, `app::tests::finish_remote_op_*` |
| `REMOTE-4` | A successful pull reloads the file list and diffs. | done | `app::tests::finish_remote_op_pull_success_sets_status_msg` |
| `REMOTE-5` | Fetch is not offered. | done (non-goal for now) | — |
