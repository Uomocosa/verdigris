use std::process::ExitCode;

use clap::Parser;
use lele_lint::CommonArgs;
use lele_lint::LintSession;
use verdigris_lint::VerdigrisLinter;
use verdigris_lint::rules;

#[derive(Parser)]
#[command(
    name = "verdigris-lint",
    version,
    about = "Enforce lele-lint and lele-bevy-lint rules over every verdigris crate"
)]
struct Args {
    #[command(flatten)]
    common: CommonArgs,

    #[arg(long = "sync-methods")]
    sync_methods: bool,
}

fn main() -> ExitCode {
    let args = Args::parse();

    let mut linter = VerdigrisLinter::from(args.common.error_format);

    if args.common.rule_list || args.common.explain.is_some() {
        if let Some(codes) = &args.common.only {
            let unknown = linter.retain_codes(codes);
            if !unknown.is_empty() {
                eprintln!(
                    "verdigris-lint: no rule with code or name `{}`",
                    unknown.join("`, `")
                );
                return ExitCode::FAILURE;
            }
        }

        if args.common.rule_list {
            linter.print_rule_list();
        } else if let Some(code) = args.common.explain {
            let Some(text) = linter.explain(&code) else {
                eprintln!("verdigris-lint: no rule with code or name `{code}`");
                return ExitCode::FAILURE;
            };
            print!("{text}");
        }
        return ExitCode::SUCCESS;
    }

    let code = LintSession {
        binary: "verdigris-lint",
    }
    .run(
        &mut linter,
        &args.common,
        args.sync_methods,
        |project, config_dir| {
            rules::comprehensive_rules::ComprehensiveRules::try_from(
                config_dir.join(lele_lint::config::CONFIG_FILENAME).as_path(),
            )
            .map_err(|error| lele_lint::Error::Prepare(Box::new(error)))?;
            VerdigrisLinter::include_rules_root(project)
                .map_err(|error| lele_lint::Error::Prepare(Box::new(error)))
        },
        |_| None,
    );
    if code == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
