# Ranking rubric for propose-next-mtg-rules

Workers fill this in for each proposal; the orchestrator re-checks and uses the same table.
Higher total = do it sooner.

| Criterion | Points |
|---|---|
| Turns at least one `#[ignore = "needs …"]` test green | +3 |
| Deletes at least one `# Notes` section from a rule file's doc | +2 |
| Every CR rule it depends on is already implemented (`deps_missing` empty) | +3 |
| Its test can be red right now, without `#[ignore]` (compiles today, no `// ?`) | +2 |
| Only extends existing systems, no new system for the rule | −1 |
| Uses only existing fixture cards | +2 |
| Needs a new card, but existing card syntax can write it | +1 |
| Touches ≤ 4 files | +2 |
| Touches 5–8 files | +1 |
| Covers a rule that most real games hit (combat, mana, casting, SBAs) | +2 |
| Each CR rule still missing that it depends on | −2 |
| Needs new card syntax | −1 |
| Needs a new `RuleSet` or changes the `RuleSet` chain order | −1 |
| Duplicates an existing system or test | **disqualified** |
| Breaks an AGENTS "Verdigris rule" (e.g. test writes with `world_mut()`) | **disqualified** |

Ties: fewer files touched first, then the lower CR number.

Score table format in a proposal:

```markdown
| Criterion | Points |
|---|---|
| Unignores r704_5h_… | +3 |
| deps_missing: 506.1 | −2 |
| … | … |
| **Total** | **7** |
```
