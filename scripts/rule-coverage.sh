#!/usr/bin/env bash
# Print what the rule engine already covers, so a proposal starts from facts:
#   1. every __rules__ system with the CR rules its doc comment quotes and its RuleSet
#   2. CR rules quoted by more than one system (possible duplicates)
#   3. every #[ignore = "needs ..."] test (rules that are only partly done)
#   4. the fixture cards tests can already use
#   5. CR sections (e.g. 506.) and how many quoted rules each one has
# Read-only. Usage: scripts/rule-coverage.sh   (from anywhere inside the repo)
set -euo pipefail

root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
rules_dir="$root/crates/rule-engine/__rules__"
plugin="$root/crates/rule-engine/__methods__/rules_plugin/plugin_build.rs"
fixtures_dir="$root/crates/rule-engine/src/testing/__fixtures__"
comp_rules="$root/docs/MagicCompRules-20250919.txt"

# A quoted rule is a doc line "/// 704.5g If ..." (number, space, capital letter);
# continuation lines such as "/// 704.5g)." are not quotes.
quoted_rules() {
    grep -oE '^/// [0-9]{3}\.[0-9]+[a-z]? [A-Z]' "$1" | awk '{print $2}' || true
}

# The RuleSet a system is registered in: the first "RuleSet::X" after its line.
rule_set_of() {
    awk -v name="rules::$1::$1" '
        index($0, name) { found = 1 }
        found && match($0, /RuleSet::[A-Za-z]+/) {
            print substr($0, RSTART + 9, RLENGTH - 9); exit
        }
    ' "$plugin"
}

echo "## 1. Systems (file | quoted CR rules | RuleSet)"
all_quotes=""
for file in "$rules_dir"/r[0-9]*.rs; do
    name="$(basename "$file" .rs)"
    quotes="$(quoted_rules "$file" | tr '\n' ' ')"
    set_name="$(rule_set_of "$name")"
    echo "$name | ${quotes:-<none>} | ${set_name:-NOT REGISTERED}"
    for q in $quotes; do all_quotes+="$q $name"$'\n'; done
done

echo
echo "## 2. CR rules quoted by more than one system"
dups="$(printf '%s' "$all_quotes" | awk 'NF { n[$1]++; f[$1] = f[$1] " " $2 }
    END { for (r in n) if (n[r] > 1) print r ":" f[r] }' | sort -V)"
echo "${dups:-<none>}"

echo
echo "## 3. Ignored tests (file:line | test | reason)"
grep -rnE -A1 '#\[ignore = "needs' "$root/crates" --include='*.rs' \
    | awk '
        /#\[ignore = "/ {
            split($0, parts, ":"); loc = parts[1] ":" parts[2]
            match($0, /"[^"]*"/); reason = substr($0, RSTART, RLENGTH); next
        }
        match($0, /fn [a-z0-9_]+/) {
            sub(/^.*\/crates\//, "crates/", loc)
            print loc " | " substr($0, RSTART + 3, RLENGTH - 3) " | " reason
        }
    ' || true

echo
echo "## 4. Fixture cards (constant | name)"
grep -hoE 'pub const [A-Z_]+: testing::Card' "$fixtures_dir"/*.rs | awk '{print $3}' | tr -d ':' \
    | while read -r const; do
        file="$fixtures_dir/$(echo "$const" | tr 'A-Z' 'a-z').rs"
        card="$(grep -oE 'TestCardName\("[^"]+"\)' "$file" | head -1 | cut -d'"' -f2)"
        echo "$const | ${card:-?}"
    done

echo
echo "## 5. CR sections with quoted rules (section | count | rules)"
printf '%s' "$all_quotes" | awk 'NF { split($1, p, "."); s = p[1]; n[s]++; r[s] = r[s] " " $1 }
    END { for (s in n) print s " | " n[s] " |" r[s] }' | sort -n
echo
echo "CR section titles: grep -nE '^[0-9]{3}\\. ' \"$comp_rules\""
