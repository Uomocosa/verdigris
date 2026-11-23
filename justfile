[unix]
set shell := ["bash", "-eu", "-o", "pipefail", "-c"]

[windows]
set shell := ["powershell.exe", "-NoLogo", "-Command"]

# Build outside `[R] Rust` (the path has spaces); persistent, portable.
export CARGO_TARGET_DIR := env("HOME") / ".cache/cargo-target/verdigris"

# Toolchain: rust-toolchain.toml (nightly).
# Extra tools: cargo-nextest, just, lele-lint / lele-bevy-lint, prek.

[doc('Show this recipe list.')]
default:
    @just --list --unsorted

[group('dev')]
[doc('Build the whole workspace (all targets, all features).')]
build:
    cargo build --workspace --all-targets --all-features

# PATTERN filters on the full test path; empty matches everything.
[group('dev')]
[doc('Run workspace tests matching PATTERN (nextest; empty matches all).')]
t PATTERN="" *FLAGS:
    cargo nextest run --workspace --all-targets --all-features --no-tests=pass -E 'test(/{{ PATTERN }}/)' {{ FLAGS }}

[group('dev')]
[doc('Clippy plus verdigris-lint for one package (e.g. just check-crate verdigris-game).')]
check-crate CRATE:
    #!/usr/bin/env bash
    set -euo pipefail
    case "{{ CRATE }}" in
        verdigris-rule-engine) dir=crates/rule-engine ;;
        verdigris-parser) dir=crates/parser ;;
        verdigris-game) dir=crates/game ;;
        verdigris-lint) dir=crates/verdigris-lint ;;
        *)
            echo "check-crate: unknown crate '{{ CRATE }}'; known: verdigris-rule-engine verdigris-parser verdigris-game verdigris-lint" >&2
            exit 1
            ;;
    esac
    cargo clippy -p {{ CRATE }} --all-targets --all-features -- -D warnings
    cargo run --quiet -p verdigris-lint -- --config lele.toml "$dir"

[group('dev')]
[doc('Check formatting (cargo fmt --all -- --check).')]
fmt:
    cargo fmt --all -- --check
    lele-fmt --check --config lele.toml

[group('dev')]
[doc('Lint the whole workspace; warnings deny the gate.')]
clippy:
    cargo clippy --workspace --all-targets --all-features -- -D warnings

[group('dev')]
[doc('Run verdigris-lint (all lele-lint and lele-bevy-lint rules) over every workspace crate dir.')]
lint:
    cargo run --quiet -p verdigris-lint -- --config lele.toml --sync-config --check crates/rule-engine
    cargo run --quiet -p verdigris-lint -- --config lele.toml crates/rule-engine
    cargo run --quiet -p verdigris-lint -- --config lele.toml crates/parser
    cargo run --quiet -p verdigris-lint -- --config lele.toml crates/game
    cargo run --quiet -p verdigris-lint -- --config lele.toml crates/verdigris-lint

# Regenerate the __methods__ indexes after adding or removing #[atomic_delegates] shells.
[group('dev')]
[doc('Regenerate __methods__ indexes after adding/removing atomic shells.')]
sync-methods:
    #!/usr/bin/env bash
    set -euo pipefail
    for dir in crates/*/; do
        if [ -d "${dir}__methods__" ]; then
            cargo run --quiet -p verdigris-lint -- --sync-methods --config lele.toml "${dir%/}"
        fi
    done

[group('gate')]
[doc('Run the full workspace test suite (nextest).')]
test:
    cargo nextest run --workspace --all-targets --all-features --no-tests=pass

[group('gate')]
[doc('Run the full prek gate on all files (same as CI).')]
check:
    prek run --all-files

[group('gate')]
[doc('Full gate: build, check.')]
ci: build check

[group('gate')]
[doc('Install the prek git hooks (one-time per clone).')]
setup:
    prek install -t pre-commit -t pre-push -t commit-msg

[doc('commit-msg hook: a commit that changes a real lele.toml must carry a Lele-Config-Request: trailer (the user request it implements).')]
lele-config-guard MSG_FILE:
    #!/usr/bin/env bash
    set -euo pipefail
    changed=$(git diff --cached --name-only | grep -E '(^|/)lele\.toml$' | grep -vE '(^|/)(lele-lint-test-crates|rule_examples|interactions|tests/fixtures)/' || true)
    [ -z "$changed" ] && exit 0
    echo "lele.toml changed in this commit:" >&2
    git diff --cached --stat -- $changed >&2
    git diff --cached -- $changed | grep -E '^[+-][^+-]' | grep -E 'level|reason|exempt|path|replaced_by|\[' >&2 || true
    if ! grep -qE '^Lele-Config-Request: .+' "{{ MSG_FILE }}"; then
        echo "lele.toml changed: add a 'Lele-Config-Request: <what the user asked>' trailer. Agents: only when the user asked for this change in this conversation." >&2
        exit 1
    fi
