---
name: feature
description: Turn a feature or change request for gitish into spec changes and get the user's approval, then either implement it through /implement or leave it planned. Use when the user runs /feature, or asks for new behaviour or a change to existing behaviour.
argument-hint: <what the feature should do>
disable-model-invocation: true
---

# /feature — spec first, then approval

Request: $ARGUMENTS

Follow the spec-first workflow in `CLAUDE.md`. This skill writes the spec change and gets it approved. **It writes no code itself.** The code is built by `/implement` (`.claude/skills/implement/SKILL.md`), right away or later, as the user chooses in step 4.

## 1. Understand

1. Read `spec/README.md`, `spec/roadmap.md`, and every spec file the request touches. Search all of `spec/` for related IDs. A feature often touches several areas: a new action needs `KEY`, a new option needs `CONF`, and a new panel or overlay needs `VIEW` or `LOOK` (every overlay needs a full redraw, `LOOK-7`).
2. Look at the code only to see what exists now. Don't plan code yet.
3. If the request has a real ambiguity that changes the behaviour, ask before drafting: use `AskUserQuestion` with concrete options, and put a recommended option first. Don't ask about details the spec's conventions already settle (e.g. XDG, or a reference app named in `spec/product.md`).

## 2. Check the request for contradictions

Before drafting, test the request against the whole spec. Stop and confirm with the user when the request:

- **Doesn't make sense as written:** it can't be checked, it describes something impossible with the stack (e.g. needs an async runtime, or mouse input), or it conflicts with itself.
- **Directly contradicts an existing requirement:** for example, "`u` undoes the last action" when `KEY` binds `u` to `Unstage` and `STAGE-1` and `STAGE-3` say it unstages.
- **Conflicts with a non-goal or stack constraint** in `spec/product.md` (e.g. "add fetch and branch management").

Don't guess and don't quietly pick a side. Ask with `AskUserQuestion`. Quote the request and every conflicting row (ID and text). Explain the conflict in a sentence, and offer concrete options:

- **Update the conflicting requirements** (when the request is clearly the newer intent): list exactly how each conflicting row will change.
- **Adjust the request** so it fits the existing spec: give the reworded requirement.
- **Something else / cancel.**

If the user confirms updating the conflicts, change every affected row as part of this spec change:

- **Reworded requirement:** edit its text in place and reset its status to `planned`, because the code must change.
- **Requirement that no longer applies:** set the status to `dropped`, with the reason `superseded by <new ID>`.
- **Non-goal or stack constraint:** edit `spec/product.md` and note it in the commit message.
- **Machine-checked tables** (`spec:keybindings`, `spec:config`): update the rows, e.g. move a key from one event to another, so the tables stay consistent.
- **References:** search `spec/` for the old IDs and behaviour (implementation notes, `roadmap.md`, the verification backlog) and update them.

These edits are part of the diff in step 4, and listed separately there as **contradictions resolved**.

If the user rejects the request, stop without editing the spec.

## 3. Draft the spec change

Edit `spec/` only:

- **New behaviour:** add rows with the next free ID in the right prefix. Never reuse an ID. Set the status to `planned`. Write **Verified by** as the test or manual check you will use.
- **Changed behaviour:** edit the existing row's text and reset its status to `planned`. Don't create a duplicate row.
- **Removed behaviour:** set the status to `dropped` and give the reason. Keep the row.
- **New binding or config key:** add it to the machine-checked tables (`spec:keybindings`, `spec:config`) with status `planned`. Keep their columns unchanged. `cargo test` still passes, because a `planned` row must not exist in the code yet. It starts failing when the code lands without the row flipped to `done`, which is intended.
- Put each new planned item in `spec/roadmap.md` at a sensible position.
- Each requirement is one checkable sentence about behaviour, not about implementation. Add **Implementation notes** only for constraints that aren't obvious.
- Re-check the finished draft against step 2. Writing the rows sometimes exposes a conflict the request hid. If it does, go back to step 2 before asking for approval.

## 4. Ask for approval

Show the user:

- The `git diff -- spec/` (summarise it if it's long, but show every new or changed row in full).
- The IDs you added or changed, and anything you assumed.
- **Contradictions resolved:** each conflicting row, as it was and as it is now, and the user's decision from step 2.
- Other trade-offs you noticed.
- How each requirement will be verified.

Then ask with `AskUserQuestion`:

- **Approve and implement** (recommended): commit the spec, then build it now through `/implement`.
- **Approve spec only**: commit the spec with the rows left `planned` in the roadmap for `/implement`, and stop.
- **Revise**: the user says what to change. Apply it, re-run step 2 on the change, then return to the start of step 4.
- **Discard**: restore the spec files you changed (`git checkout -- <those files>`, and delete new files you created), then stop.

Never treat silence, or an unrelated message, as approval.

## 5. After approval

1. Commit the spec on its own: `spec: <summary>` with the usual trailers.
2. **Approve spec only:** report the IDs now in the roadmap and stop. The user runs `/implement` when they want them built.
3. **Approve and implement:** read `.claude/skills/implement/SKILL.md` and follow it with these IDs as the limit. Its report is this skill's report.
