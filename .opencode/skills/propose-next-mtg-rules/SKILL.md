---
name: propose-next-mtg-rules
description: Use when the user invokes /propose-next-mtg-rules [lane] in verdigris to find MTG Comprehensive Rules the rule engine has not implemented (or only partly), and propose for each one a red MockGame test, optional real test cards, and code previews for the Bevy system. PROPOSAL ONLY — the one file it writes is its own report under .local-run/. Also says how an orchestrator fans it out to several workers and ranks the results.
---

# propose-next-mtg-rules

Find the next MTG Comprehensive Rules (CR) worth implementing in verdigris and write up to
**3 proposals**. Each proposal is one Bevy system that covers one rule or a small group of rules.

Request / lane: $ARGUMENTS

**No edits to the repo.** The only file you may write is your report (see "Output"). Never
stage, commit or run anything that changes files outside `.local-run/`.

## 0. Load and read first

1. Load `lele-rs` (it binds `lele-syntax-rs`) and `bevy-rs`.
2. Read `AGENTS.md`, especially "Verdigris rules" 1–10. Every proposal must obey them.
3. Run `scripts/rule-coverage.sh`. It lists every system with the CR rules it quotes and its
   `RuleSet`, the ignored tests, the fixture cards and coverage per CR section. Treat it as the
   truth about what exists; do not rebuild that list by hand.
4. Read `crates/rule-engine/__methods__/rules_plugin/plugin_build.rs` (messages, `RuleSet`
   chain, registration) and two existing rule files close to your lane, with their tests.

## 1. Lane

`$ARGUMENTS` picks where to look. Empty = whole CR (sections 1–7; skip 8 Multiplayer and
9 Casual Variants unless asked).

| Lane | Looks at |
|---|---|
| `1xx-3xx` | Game concepts, parts of a card, card types (`100.`–`315.`) |
| `4xx-5xx` | Zones and turn structure, including combat (`400.`–`514.`) |
| `6xx` | Spells, abilities, effects (`600.`–`616.`) |
| `7xx` | Additional rules: keyword actions, keyword abilities, state-based actions (`700.`–`732.`) |
| `partial` | Already-covered sections: ignored tests, missing subrules of quoted rules |
| `<NNN>` or `<NNN.N>` | That one section or rule |

`--exclude <CR,CR,...>` skips rules another worker already took.

CR text: `docs/MagicCompRules-20250919.txt`. Section titles:
`grep -nE '^[0-9]{3}\. ' docs/MagicCompRules-20250919.txt`.

## 2. Pick candidates

A candidate is a CR rule (or group of rules) that is either:

- **missing** — no system quotes it, or
- **partial** — an ignored test needs it, or a system quotes the parent rule but leaves out a
  subrule that a real card can hit (e.g. `120.3` quoted but `120.3c` not handled).

Prefer, in order:

1. rules that turn an ignored test green;
2. rules whose prerequisites are already implemented (check the coverage list, not your
   memory of MTG);
3. rules a real, simple card exercises with existing card syntax;
4. small changes.

**Grouping.** Several rules may share one system when they fire on the same trigger, read the
same state and would otherwise be split for no reason (see `r117_3_priority`,
`r500_3_step_without_priority_ends`). Name the file after the main rule and quote every rule in
its doc comment. Never group rules only to make a proposal look bigger.

**Duplicates.** Before proposing a test, search for one of the same scenario
(`grep -rn "fn r<num>" crates/` plus a search on the card and the action). If one exists, the
proposal extends or reuses it (AGENTS rule 8).

## 3. Write each proposal

Every claim about existing code cites a `file:line` you actually read. Use exactly these
headings:

```markdown
### P<n>. <CR numbers> — <short title>

**Rules** — status `missing` | `partial`, then each rule number with its CR text quoted
verbatim.

**System** — file `crates/rule-engine/__rules__/r<num>[_<letter>]_<slug>.rs`, function name,
`RuleSet` and where in its `.chain()` (before/after which system and why), messages/components
it reads and writes, new message or component types. If it covers several rules: one line on
why they share a system. If the rule is a restriction inside an existing system (e.g. a
targeting rule in `r601_2c_targets`) rather than a new system, say so, name the systems it
extends, and say whether a dedicated system was considered and why not — the goal is one
system per rule (or rule group), so extending is the exception.

**Test** — the full red `MockGame` test (rust fence), named after the rule
(`r<num>_<behaviour>`), driven only through builder setup and player actions (`act`,
`pass_priority`, `pass_priority_all`, `end_turn`), asserting each state. Never writes messages
or components with `app.world_mut()`. If no action can reach the state yet:
`#[ignore = "needs <feature>: <why>"]` and say which proposal/feature unblocks it.
"Red now" means it compiles against today's API (plus only the cards/types this proposal
adds) and fails on the assert; a test with `// ?` placeholders or actions that don't exist yet
is **not** red now.
Then: the existing tests you searched, and why this one is not a duplicate.

**Cards** (optional) — only if no fixture card exercises the rule. Real MTG name and real
Oracle text, the `testing::Card` fixture (rust fence, like
`src/testing/__fixtures__/typhoid_rats.rs`), and any card syntax that must grow to write it.
Card text is the syntax: the definition uses the card's own words.

**Code previews** — `+`/`-`/`!` previews (diff fences) for: the new rule file (doc comment
quoting the CR, signature, key lines; no full bodies), the `plugin_build.rs` registration,
the `mod.rs` line, any new types, any fixture `mod.rs` line. Mark guesses with `// ?`.

**Size and risks** — files touched (count), new syntax (yes/no + what), CR rules it depends on
that are still missing, anything that could break existing tests.

**Score** — the rubric table from `references/RANKING.md`, filled in, and the total.
```

Rules for the code: one system per file, it only reads world state and writes
messages/choices/components, no time, no randomness except `GameRng`, no input; relative words
(`YOU`, `OPPONENT`, `ANY`) resolved at rule time for 2+ players; `thiserror` errors, no
`unwrap`/`expect`/`panic` in production code; the file ends with a `test_usage` test.

## 4. Output

Write the report to
`.local-run/orchestrator/<YYYY-MM-DD>-propose-next-mtg-rules/<lane-or-solo>.md`
(today's date; `solo` when run alone without a lane; append `-2`, `-3` if the file exists).
If you cannot write files (read-only agent), put the whole report in your final message
instead.

The report is: a one-paragraph summary of what you searched, the proposals (best first), then
**one** YAML block that an orchestrator parses:

```yaml
lane: 7xx
proposals:
  - id: P1
    rules: ["702.2b", "702.2c"]
    status: missing          # missing | partial
    title: Deathtouch combat damage
    system_file: r702_2b_deathtouch.rs   # null if it only extends existing systems
    extends: []                          # existing systems it changes, e.g. [r601_2c_targets]
    rule_set: StateBased
    red_now: true                        # see "Test" — false if it has `// ?` or is ignored
    files_touched: 4
    new_cards: ["Typhoid Rats"]   # [] if fixtures suffice
    new_syntax: false
    deps_missing: ["506.1"]       # CR rules still needed first; [] if none
    unignores: ["r704_5h_deathtouch_damage_destroys_a_larger_creature"]
    score: 11
```

Your final message: the report path (or the report) and one line per proposal.

## 5. When orchestrated (`/opencode-orchestrator … /propose-next-mtg-rules …`)

For the orchestrator, on top of `orchestrator-core`:

1. **One lane per worker** so they don't all propose the same easiest rule. For N workers use
   the lanes in this order: `4xx-5xx`, `6xx`, `7xx`, `partial`, `1xx-3xx`; past 5, split the
   busiest lane by section (e.g. `702`, `704`).
2. Worker prompt = `/propose-next-mtg-rules <lane>`. Workers write only their own report file,
   so several may run in the same directory at once; still no other edits.
3. When all are done: read every report's YAML block, merge, and drop duplicates (same CR rule
   in two proposals → keep the higher score, note the other).
4. **Check the top 3 yourself** against the code: cited `file:line`s are real, the rule really
   is missing/partial (`scripts/rule-coverage.sh`), the test only uses player actions, no
   existing test already covers it. Fix the score or drop it if not.
5. Rank with `references/RANKING.md` and write `report.md` in the same folder: ranked table
   (rank, CR rules, title, score, lane, deps), then the top 3 proposals in full, then cost per
   worker. Tell the user the path and the top 3 in one line each.
