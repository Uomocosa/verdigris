use std::path::PathBuf;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ComprehensiveRulesError {
    #[error("could not read `{path}`")]
    LeleUnreadable {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
    #[error("could not parse `[verdigris.lint]` in `{path}`")]
    BadTable {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("`{path}` has no `[verdigris.lint] comprehensive_rules` key")]
    MissingKey { path: PathBuf },
    #[error("could not read Comprehensive Rules file `{path}`")]
    RulesUnreadable {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
