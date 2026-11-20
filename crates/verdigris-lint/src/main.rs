use std::process::ExitCode;

use clap::Parser;
use lele_lint::CommonArgs;
use lele_lint::Config;
use lele_lint::Project;
use verdigris_lint::VerdigrisLinter;

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
        return ExitCode::SUCCESS;
    }

    if let Some(code) = args.common.explain {
        let Some(text) = linter.explain(&code) else {
            eprintln!("verdigris-lint: no rule with code or name `{code}`");
            return ExitCode::FAILURE;
        };
        print!("{text}");
        return ExitCode::SUCCESS;
    }

    let mut project = match Project::try_from_start_dir_and_scan_folders(
        args.common.path.as_deref(),
        args.common.scan_folder.as_deref(),
    ) {
        Ok(project) => project,
        Err(error) => {
            eprintln!("verdigris-lint: {}", lele_lint::Error::with_causes(&error));
            return ExitCode::FAILURE;
        }
    };

    let (config, config_dir) = match Config::resolve(args.common.config.as_deref(), &project) {
        Ok(resolved) => resolved,
        Err(error) => {
            eprintln!(
                "verdigris-lint: lele.toml: {}",
                lele_lint::Error::with_causes(&error)
            );
            return ExitCode::FAILURE;
        }
    };
    project.config_dir = config_dir;

    if let Err(error) = VerdigrisLinter::include_rules_root(&mut project) {
        eprintln!("verdigris-lint: {}", lele_lint::Error::with_causes(&error));
        return ExitCode::FAILURE;
    }

    if let Err(error) = project.apply_layout(&config) {
        eprintln!("verdigris-lint: {}", lele_lint::Error::with_causes(&error));
        return ExitCode::FAILURE;
    }

    if !project.parse_failures.is_empty() {
        for failure in &project.parse_failures {
            eprintln!(
                "verdigris-lint: could not parse {}: {}",
                failure.path.display(),
                failure.cause
            );
        }
        return ExitCode::FAILURE;
    }

    if args.sync_methods {
        if let Err(error) = project.sync_methods() {
            eprintln!("verdigris-lint: {}", lele_lint::Error::with_causes(&error));
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    if linter.run(&project) == 0 {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}
