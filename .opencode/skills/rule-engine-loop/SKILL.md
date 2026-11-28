---
name: rule-engine-loop
description: verdigris loop — FREE_MODEL proposers over /propose-next-mtg-rules, rank, implement the next CR rules red-first with workers, gate, commit, push on the current branch, report. Follows loop-core. User-invoked only (/rule-engine-loop [--hours H] [--mode propose] [tier overrides] [extra notes]).
disable-model-invocation: true
---

# verdigris rule-engine loop

**First load the `loop-core` skill** (it loads `opencode-orchestrator`, `orchestrator-core` and
`model-roster`) and follow it. This file fills loop-core's slots for verdigris.

Request: $ARGUMENTS

**Working directory.** The repo is `/home/uomocosa/Syncthing/[R] Rust/verdigris`, wherever the
session started. Every shell call starts with `cd "/home/uomocosa/Syncthing/[R] Rust/verdigris" &&`;
every relative path in this skill and its briefs (`.opencode/...`, `.local-run/...`, `just ...`)
is relative to that root, and workers are launched from it.

Run on your own from start to report. You and the workers may commit and push as you see fit,
always on the current branch.

## GOAL

Implement one more CR rule (or finish a partly implemented one), red first
(AGENTS.md "Verdigris rules" 6–8).

## PICKER

0. Reuse: if a ranked `report.md` from an earlier run of this loop exists and no commit since
   it touched `crates/` (other than that run's own items), reuse its ranking and skip research.
   Say so in plan.md. Gaps a previous report lists ("engine gaps found") are candidates too.
1. Research: 5 FREE_MODEL lanes in parallel, `--agent plan`, prompt
   `/propose-next-mtg-rules <lane> --out <work dir>`; lanes `1xx-3xx`, `4xx-5xx`, `6xx`, `7xx`,
   `partial` (rules already partly implemented). Plan agents cannot write files: when a lane
   ends, save its final message as `<lane>.md` yourself (last text part of its jsonl).
2. Rank yourself into one table in `report.md`: CR, title, worker score, lane, decision
   (IMPLEMENT / DEFERRED / CANNOT TODAY / DROPPED) and the reason. Check each claim before you
   rank: the card exists and is real (drop made-up cards), the proposed test is red, no
   rule file or system already covers it. In `propose` mode "red" is checked by reading (say so);
   in a normal run the item's plan step proves it by running the extended test before any fix.
3. The next item is the best IMPLEMENT row. Re-rank between items.

## ORACLE

- Fast: the test the item extended is red before the change and green after; `just fmt`,
  `just clippy`, `just lint` green for the touched crates.
- Full: the pre-push hook runs `just test`; a push is the full gate.

## Briefs (verdigris specifics)

- One brief per item (`brief-<X>.md`), one worker, one commit.
- Every brief tells the worker to read `.opencode/skills/verdigris-rules-style/SKILL.md` and
  its `references/CHANGEABLE_RULES.md` first.
- Every `worker-env.md` says: "Tests: red first, by adding a step or assert to an existing
  test. A new test needs a reason in the report."
- A brief that adds, removes or reshapes an event field lists EVERY construction site
  (`rg -n '<Event> \{' crates/`) and says where each gets the value, including producers that
  are not events (state-based actions, 608.2 resolution, steps: they use their rule's own
  `CAUSE`, CHANGEABLE_RULES §6.4). A missing value stops the worker mid-job.

## STOPS (in addition to loop-core)

- A proposal with a made-up card, or a rule already covered → DROPPED, not a failure.
- New card syntax is allowed; list every addition in the report for review.
- "Nothing worth implementing today" is a normal end; say why in the report.

## Run length

loop-core default (up to 3 h). A dirty tree at start: work out whose changes they are, commit or
stash them as you judge, and say so in the report.
