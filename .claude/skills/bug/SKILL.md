---
name: bug
description: Turn a bug report for gitish into a reproduction, a root cause, and the spec change that makes the expected behaviour explicit, then ask the user to approve it and either implement the fix through /implement or leave it planned. Use when the user runs /bug.
argument-hint: <what went wrong, and what you expected>
disable-model-invocation: true
---

# /bug — reproduce, close the spec gap, approve

Report: $ARGUMENTS

Follow the spec-first workflow in `CLAUDE.md`. A bug means the spec failed to pin down the behaviour, or the checks failed to hold the code to it. This skill fixes both on paper: a reproduction that fails, and a spec change that says what should happen. The fix itself is built by `/implement` (`.claude/skills/implement/SKILL.md`), right away or later, as the user chooses in step 5.

## 1. Understand the report

1. Restate the bug as **observed** vs **expected**, and note the conditions (mode, focused panel, config, theme, terminal size, the state of the repository).
2. Search all of `spec/` for the behaviour: the requirement rows, implementation notes, the machine-checked tables and the verification backlog. Note every row that touches it.
3. Ask with `AskUserQuestion` only if the expected behaviour is genuinely unclear and the spec doesn't settle it. Offer concrete options and put the recommended one first.

## 2. Reproduce

Reproduce the bug before changing anything, and keep the reproduction as the regression check:

- **Unit test first** when the behaviour can be tested without a terminal (everything in `git/`, key translation, `App` state changes, parsing, config). `git/` tests use a temp repository. Write the test and confirm it fails.
- **Otherwise a manual script** in `scripts/manual/` (source `lib.sh`), or a new check in the script that already covers the area. Follow "Manual testing" in `CLAUDE.md`: it runs gitish in a detached tmux session against a temporary repository, so it never touches the user's terminal or repositories.
- Record what you saw (test output, a `tmux capture-pane` dump), so the report can show before and after.

If you can't reproduce it, say what you tried and ask the user for the missing detail. Don't guess at a fix.

## 3. Find the root cause

Trace the bug to its cause in the code. Read the ratatui, crossterm and git2 docs or source rather than guessing. Then look for the same mistake elsewhere: other modes, overlays or code paths with the same pattern. The spec change should state the general rule, not just the one instance. Don't change the code.

## 4. Draft the spec delta

Classify the bug against the spec:

| Case | Spec change |
|------|-------------|
| A requirement covers the behaviour and the code breaks it | The text is fine. Set the status to `partial` with the bug written in as the gap, and name the regression check in **Verified by**. |
| A requirement is vague or incomplete, so the bug was allowed | Reword the row so the expected behaviour is explicit and checkable. Reset its status to `planned`. |
| No requirement covers it | Add a row with the next free ID in the right prefix, with status `planned` (the bug becomes the reason the row exists). |
| The general rule belongs in more than one place | Update every affected row, or add one rule and reference it from the others. |
| A past fix failed, or a constraint isn't obvious | Add an **Implementation notes** entry saying why. |

Stop and ask with `AskUserQuestion` (quote the rows by ID and text, and give concrete options, recommended first) when:

- **The spec asks for the reported behaviour.** Then it isn't a bug against the spec. Offer: change the spec (then continue as `/feature` does from its step 3), or keep it as is.
- **The fix would change a `done` requirement beyond the bug**, or conflict with another requirement, a non-goal or a stack constraint in `spec/product.md`.
- **There are several reasonable expected behaviours**, and the spec's conventions (XDG, or a reference app named in `spec/product.md`) don't pick one.

Write each requirement as one checkable sentence about behaviour, and name the regression check from step 2 in its **Verified by**. Add each touched row to `roadmap.md`, near the top: a known bug usually comes before new features. Re-check the draft against the whole spec for new conflicts.

## 5. Ask for approval

Show the user:

- **Bug and cause:** observed vs expected, and the root cause in a sentence or two.
- **Spec delta:** each changed row before and after, and each new row in full.
- **Regression check:** what it is and what it showed (it fails now).
- Other places with the same mistake, and anything you assumed.

Then ask with `AskUserQuestion`:

- **Approve and implement** (recommended): commit the spec and the reproduction, then fix it now through `/implement`.
- **Approve spec only**: commit the spec and the reproduction, leave the rows in the roadmap for `/implement`, and stop.
- **Revise**: the user says what to change. Apply it, re-check it as in step 4, then return to the start of step 5.
- **Discard**: restore the spec files you changed and remove the reproduction, then stop.

Never treat silence, or an unrelated message, as approval.

## 6. After approval

1. Commit the spec on its own: `spec: <summary> (ID)`. In the body, give the bug in one line (observed vs expected) and the root cause. Use the usual trailers.
2. Commit the reproduction on its own: `Reproduce <summary>` with a `Spec: ID` trailer. A unit test that fails would break `cargo test` and CI, so commit it with `#[ignore = "<ID>: fails until fixed"]`. `/implement` removes the attribute. Manual scripts don't run in CI and need no marker.
3. **Approve spec only:** report the IDs now in the roadmap and stop. The user runs `/implement` when they want the fix.
4. **Approve and implement:** read `.claude/skills/implement/SKILL.md` and follow it with these IDs as the limit. Its report is this skill's report, and it should show the regression check failing before the fix and passing after.
