use lele_lint::PathExemption;

pub const RULES_ROOT: &str = "__rules__";

pub const EXEMPTIONS: [PathExemption; 5] = [
    PathExemption {
        code: "LELE008",
        root: RULES_ROOT,
        reason: "A rule file keeps the small helpers its Bevy system needs as module-private fns; there is no owner type to turn them into methods, and a helper type or a __methods__ body would break the one-system-per-rule-file convention.",
    },
    PathExemption {
        code: "LELE031",
        root: RULES_ROOT,
        reason: "Every rule file starts with a doc comment quoting the Comprehensive Rules text it enforces; that quotation is the rule's specification next to its system, and VRDG007 enforces the stricter verdigris version (only the system fn's doc comment, checked word-for-word against the rule text), outside the single constants.rs comment exception.",
    },
    PathExemption {
        code: "LELE034",
        root: RULES_ROOT,
        reason: "A rule file holds exactly one pub fn Bevy system named after the CR rule; the system takes Bevy system parameters and has no owner type to become a method of, so it stays a free function by convention.",
    },
    PathExemption {
        code: "LELEBEVY002",
        root: RULES_ROOT,
        reason: "CR rule systems live in __rules__/ (one file per rule) instead of a domain's __bevy_systems__/ so the rule-to-file mapping stays complete; moving them would lose that mapping.",
    },
    PathExemption {
        code: "LELETEST007",
        root: RULES_ROOT,
        reason: "A rule file's primary item is a Bevy system registered on the app; its test_usage drives that system through a MockGame scenario, so it cannot call the system directly or assert a value the system returns.",
    },
];
