# Merge conflicts

## Requirements

| ID | Requirement | Status | Verified by |
|----|-------------|--------|-------------|
| `MERGE-1` | A file with unresolved conflicts is listed as conflicted. Its diff panel shows each conflict block with the ours side and the incoming side, and the title shows the position as "conflict n/N". | done | `git::repo::tests::list_changed_files_detects_conflicted`, `ui_helpers::tests::hunk_counter_shows_conflict_position`; manual: `scripts/make-conflict-repo.sh` |
| `MERGE-2` | Both 2-way and diff3 conflict markers are read. The base section of diff3 is skipped. | done | `git::repo::tests::parse_conflict_*` |
| `MERGE-3` | On a conflicted file, `j`/`k` and `n`/`p` move between conflict blocks. | done | manual: `scripts/make-conflict-repo.sh` |
| `MERGE-4` | `o` replaces the selected block in the file with the ours side, `i` with the incoming side, and `b` with both. The status bar says "Conflict resolved (ours\|theirs\|both)". | done | `git::stage::tests::resolve_conflict_*` |
| `MERGE-5` | A conflicted file with no markers left shows "No conflict markers found". | done | manual |
