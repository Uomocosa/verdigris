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
plugin_dir="$root/crates/rule-engine/__methods__/rules_plugin"
fixtures_dir="$root/crates/rule-engine/src/testing/__fixtures__"
comp_rules="$root/docs/MagicCompRules-20250919.txt"

# A quoted rule is a doc line "/// 704.5g If ..." or "/// 121.1. A ..." (the CR
# puts a period after an unlettered rule number). Continuation lines such as
# "/// 704.5g)." are not quotes. Normalise away that trailing period.
quoted_rules() {
    grep -oE '^/// [0-9]{3}\.[0-9]+[a-z]?\.? [A-Z]' "$1" \
        | awk '{rule = $2; sub(/\.$/, "", rule); print rule}' || true
}

# The RuleSet a system is registered in. Registrations live in add_*_systems.rs,
# not plugin_build.rs, and an app.add_systems(...) call can span lines and mention
# several systems. Collect the RuleSet:: names from every block that registers
# this system: one name is its set; several names mean it is only being ordered
# between sets (the layers system), so report that set rather than a single lie.
rule_set_of() {
    awk -v name="$1" '
        function add_unique(list, value) {
            if (index(" " list " ", " " value " ") == 0)
                return (list == "" ? value : list " " value)
            return list
        }
        function parse_block(block,    rest, tok, r, rs, schedule, tmp, parts, found) {
            tmp = block
            sub(/^.*add_systems\(/, "", tmp)
            sub(/^[[:space:]]+/, "", tmp)
            split(tmp, parts, /[[:space:],]/)
            schedule = parts[1]

            rest = block
            while (match(rest, /rules::[a-z0-9_]+::/)) {
                tok = substr(rest, RSTART, RLENGTH)
                rest = substr(rest, RSTART + RLENGTH)
                sub(/^rules::/, "", tok)
                sub(/::$/, "", tok)
                if (tok != name) continue
                seen[name] = 1
                found = 0
                r = block
                while (match(r, /RuleSet::[A-Za-z]+/)) {
                    rs = substr(r, RSTART + 9, RLENGTH - 9)
                    r = substr(r, RSTART + RLENGTH)
                    sets[name] = add_unique(sets[name], rs)
                    found = 1
                }
                if (!found) schedules[name] = add_unique(schedules[name], schedule)
            }
        }
        /\.add_systems\(/ { in_block = 1; block = "" }
        in_block {
            block = block " " $0
            if ($0 ~ /\);[[:space:]]*$/) {
                parse_block(block)
                in_block = 0
                block = ""
            }
        }
        END {
            if (!(name in seen)) exit
            n = split(sets[name], order, " ")
            if (n == 0) print "no set (" schedules[name] ")"
            else if (n == 1) print order[1]
            else {
                printf "ordered ("
                for (i = 1; i <= n; i++) printf "%s%s", (i > 1 ? ", " : ""), order[i]
                print ")"
            }
        }
    ' "$plugin_dir"/add_*_systems.rs || true
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
{ grep -hoE 'pub const [A-Z_]+: testing::Card' "$fixtures_dir"/cards/*.rs || true; } \
    | awk '{print $3}' | tr -d ':' \
    | while read -r const; do
        file="$fixtures_dir/cards/$(echo "$const" | tr '[:upper:]' '[:lower:]').rs"
        card="$(grep -oE 'TestCardName\("[^"]+"\)' "$file" | head -1 | cut -d'"' -f2 || true)"
        echo "$const | ${card:-?}"
    done

echo
echo "## 5. CR sections with quoted rules (section | count | rules)"
printf '%s' "$all_quotes" | awk 'NF { split($1, p, "."); s = p[1]; n[s]++; r[s] = r[s] " " $1 }
    END { for (s in n) print s " | " n[s] " |" r[s] }' | sort -n
echo
echo "CR section titles: grep -nE '^[0-9]{3}\\. ' \"$comp_rules\""
