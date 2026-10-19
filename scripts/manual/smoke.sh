#!/usr/bin/env bash
# Checks VIEW-1…VIEW-4 and COMMIT-1 in a real terminal (tmux).
# Usage: scripts/manual/smoke.sh   (after: nix develop -c cargo build)
source "$(dirname "$0")/lib.sh"

repo=$(make_repo)
start "$repo"

check VIEW-1 " Files "
check VIEW-1 " Commit "
check VIEW-2 "numbers.txt"
check VIEW-2 "new.txt"
check VIEW-2 "untracked.txt"
check VIEW-4 "new.txt [1/1]"
check VIEW-4 "Staged"

keys j
check VIEW-6 "numbers.txt [1/2]"
check VIEW-4 "Unstaged"

keys Tab
check VIEW-3 "j/k: nav"
keys Tab

keys c
check COMMIT-1 "Commit Title"
keys "smoke test" Enter
check COMMIT-1 "Commit Body"
keys Enter
check COMMIT-1 "Committed"

[ "$(git -C "$repo" log -1 --format=%s)" = "smoke test" ] ||
  { echo "FAIL  COMMIT-1: HEAD is not the new commit" >&2; FAILED=1; }

keys q
finish
