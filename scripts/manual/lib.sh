#!/usr/bin/env bash
# Helpers for the manual checks. Source this file; do not run it.
#
# gitish runs in a detached tmux session on a private tmux server (-L), against a
# temporary repository and a temporary XDG_CONFIG_HOME. It never touches the
# user's terminal, tmux sessions, repositories or config.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
GITISH_BIN="${GITISH_BIN:-$ROOT/target/debug/gitish}"
WORK="$(mktemp -d)"
SOCKET="gitish-manual-$$"
SESSION=t
COLS=120
ROWS=36
FAILED=0

tm() { tmux -L "$SOCKET" "$@"; }

cleanup() {
  tm kill-server 2>/dev/null || true
  rm -rf "$WORK"
}
trap cleanup EXIT

[ -x "$GITISH_BIN" ] || { echo "build first: nix develop -c cargo build" >&2; exit 2; }

# Makes a repo with one committed file changed in two places, one new staged
# file and one untracked file. Prints its path.
make_repo() {
  local repo="$WORK/repo"
  mkdir -p "$repo"
  git -C "$repo" init -q
  git -C "$repo" config user.email test@test.com
  git -C "$repo" config user.name Test
  seq 1 40 >"$repo/numbers.txt"
  git -C "$repo" add numbers.txt
  git -C "$repo" commit -q -m "base"
  sed -i -e '3s/.*/three/' -e '35s/.*/thirty-five/' "$repo/numbers.txt"
  echo staged >"$repo/new.txt"
  git -C "$repo" add new.txt
  echo scratch >"$repo/untracked.txt"
  echo "$repo"
}

# start REPO [gitish args...]
start() {
  local repo=$1
  shift
  mkdir -p "$WORK/config"
  tm new-session -d -s "$SESSION" -x "$COLS" -y "$ROWS" \
    "cd '$repo' && XDG_CONFIG_HOME='$WORK/config' '$GITISH_BIN' $*"
  sleep 1
}

screen() { tm capture-pane -t "$SESSION" -p; }
keys() { tm send-keys -t "$SESSION" "$@"; sleep 0.3; }

# check ID "text"  — passes when the screen shows the text within 3 seconds.
check() {
  local id=$1 text=$2
  for _ in 1 2 3 4 5 6; do
    if screen | grep -qF -- "$text"; then
      echo "ok    $id: $text"
      return 0
    fi
    sleep 0.5
  done
  echo "FAIL  $id: screen does not show: $text" >&2
  screen >&2
  FAILED=1
}

finish() {
  [ "$FAILED" = 0 ] && echo "all checks passed" || { echo "some checks failed" >&2; exit 1; }
}
