# verdigris

A Magic: The Gathering rules enforcer and playable game in Bevy, with multiplayer (2+ players) over the lobby-room p2p plugin.

## Crates

| Crate | Path | Role |
|---|---|---|
| `verdigris-rule-engine` | `crates/rule-engine/` | One Bevy system per Comprehensive Rule, headless, proven by `MockGame` tests. |
| `verdigris-parser` | `crates/parser/` | Oracle text → `CardDefinition`; cards are data, not hand-written Rust. |
| `verdigris-game` | `crates/game/` | The playable multiplayer app: rule engine, p2p lobby, choice UI. |
| `verdigris-lint` | `crates/verdigris-lint/` | lele-lint plus verdigris rules; placeholder until custom rules are supported. |

## Development

```bash
just setup   # install the prek git hooks (one-time per clone)
just t       # run the tests (nextest, optional name pattern)
just ci      # build, then the full prek gate (fmt, clippy, lint, test)
```

`just lint` reads the root `lele.toml` and runs lele-lint, lele-bevy-lint and the verdigris
rules; boundaries are workspace-relative.

## Adding a rule

1. Add one file `crates/rule-engine/src/__rules__/r<num>_<slug>.rs` (letters for subrules: `r704_5a_...`).
2. Write exactly one system; its doc comment quotes the rule text from `docs/MagicCompRules-20250919.txt`.
3. Register the system in `RulesPlugin`.
4. Start with a red `MockGame` test named after the rule, then make it green.

Card text is the syntax: a `CardDefinition` uses the card's own words (`YOU`, `target`, `damage(3)`), never engine terms.
