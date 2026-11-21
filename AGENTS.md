# AGENTS.md

## Project overview

verdigris enforces the MTG Comprehensive Rules in Bevy, headlessly and with multiplayer.
The rules text lives in `docs/MagicCompRules-20250919.txt`.

## Build, lint, and test

Every shell call starts with the tool path:
`export PATH="/home/uomocosa/Syncthing/[R] Rust/bin:$HOME/.cargo/bin:$HOME/.local/bin:$PATH"`

Run tasks through the justfile; never pipe `just` into `tail`/`head`/`grep` (append `2>&1`).

```bash
just fmt        # cargo fmt --all -- --check
just clippy     # clippy, warnings deny
just lint       # lele-lint on every crate (root lele.toml)
just bevy-lint  # lele-bevy-lint on rule-engine and game
just test       # nextest workspace
just ci         # build + full prek gate
```

If a git hook fails, stop and report the root cause; never use `--no-verify`.

## Code style

- Rust 2024, nightly toolchain pinned in `rust-toolchain.toml`; formatting via `just fmt`.
- Errors are `thiserror` enums; no `unwrap`/`expect`/`panic` in production code.
- One primary item per file, named after it; methods are `#[atomic_delegates]` shells with bodies in `__methods__/`.
- Tests live inline (`#[cfg(test)]`) and each non-trivial file ends with a `test_usage` test (except `__rules__/` files, whose tests are named for the behaviour they check).
- Never add `#[allow(clippy::…)]` for pedantic/nursery without explicit user approval.
- Every claim about existing code cites a `file:line` you actually read.
- Load the `verdigris-style` skill before writing or reviewing Rust here; repo-specific style decisions live there.

## Working with AI assistants

1. Read the relevant files and existing patterns before changing anything.
2. Run `just fmt`, `just clippy`, `just lint`, `just bevy-lint`, `just test` after changes.
3. Keep the change behaviour-preserving unless the task says otherwise.
4. Never stage, commit, push, merge, rebase, or amend without an explicit user command.

## Verdigris rules

1. Each MTG Comprehensive Rule gets one file `crates/rule-engine/src/__rules__/r<num>[_<letter>]_<slug>.rs` with exactly one system; its doc comment quotes the CR text (docs/MagicCompRules-20250919.txt).
2. Card text is the syntax. A `CardDefinition` reads like the printed Oracle text: use the card's own words (`YOU`, `OPPONENT`, `ANY`, `target`, `HAND.of(YOU)`, `damage(3)`, `destroy()`). Never rewrite card text into engine terms in a card definition; if a card can't be written that way, the syntax grows to fit it.
3. Card definitions describe; the engine (`__rules__` systems) orchestrates. Relative words (YOU, OPPONENT, ANY) are resolved against the controller at rule time and must work for any number of players (2+).
4. Game state is absolute: a real card is in a real zone entity (`InZone(zone)` + `Owner(player)`), never a relative zone.
5. A rule system only reads world state and writes messages/choices/components. No time, no randomness except a seeded `GameRng` resource, no input.
6. Tests are mainly `MockGame` scenarios: defaults are 2 players, 20 life, 12 Grizzly Bears in each library, empty hand/graveyard/battlefield, turn 1 P1 precombat main. Tests override only what differs, perform player actions, and assert each state. Every behaviour change starts with a red MockGame test, named after the rule it checks.
7. Tests drive the game only through `MockGame` builder setup and player actions (`act`, `pass_priority`, `pass_priority_all`, `end_turn`). They never write messages or insert or change components with `app.world_mut()`; reading components in asserts is fine. When no action can reach a state yet, write the test as the real scenario and mark it `#[ignore = "needs <feature>: <why>"]`, never `todo!()`, a comment or a log. The one accepted exception is `r608_2b_illegal_targets.rs`.
8. One behaviour, one test: before adding a test, search for an existing test of the same scenario; keep only one, preferably the one named after the rule.
9. Test cards use their real MTG names and real Oracle text (Grizzly Bears, Lightning Bolt, Murder).
10. verdigris-lint logic waits for the user's explicit go.

## Shared work directory

Plans, briefs, reports, logs and scratch files go in
`<repo>/.local-run/orchestrator/<YYYY-MM-DD>-<topic>/` (gitignored).
Never create task files in `~/.claude/`, `~/.opencode/`, `~/.config/`, `/tmp`, or a session scratchpad.

## Card definitions and domains

Before writing or reviewing card definitions or adding domain items, read
`docs/RATIONALE.md` and `docs/SMELLS.md` (verdigris-specific; they extend the fleet
lele-syntax-rs RATIONALE/SMELLS).

## lele.toml is user policy

**CRITICAL: NEVER change a repo's `lele.toml` unless the user specifically requested that change in this conversation.**

- A lint error is fixed in the code, not in the config. If a rule looks wrong, or two rules seem to contradict each other, stop and show the user both diagnostics.
- Do not add an exemption, a `level = "off"`, or a `reason` on your own initiative.
- When the user does ask, make exactly that change, and put their request in the commit trailer `Lele-Config-Request: <their words>`. The `lele-config-guard` commit-msg hook rejects a `lele.toml` change that has no trailer.
- `LELECORE*` rules are locked: they can never be turned off, only exempted per path with a reason.
- Every lint run prints a "lele.toml relaxes rules" banner. Mention any new line in it to the user.
- Fixture configs under `lele-lint-test-crates/`, `rule_examples/`, `interactions/` and `tests/fixtures/` are test data. Linter work may edit them.
