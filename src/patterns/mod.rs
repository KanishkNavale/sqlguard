pub mod abstracts;
use abstracts::{DUCKDB, GENERAL, POSTGRES, SQLITE};
use regex::RegexSet;
use std::{collections::HashSet, sync::LazyLock};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub enum Database {
    General,
    Sqlite,
    Postgres,
    DuckDb,
}

impl Database {
    pub fn as_str(&self) -> &'static str {
        match self {
            Database::General => "general",
            Database::Sqlite => "sqlite",
            Database::Postgres => "postgres",
            Database::DuckDb => "duckdb",
        }
    }
}

#[derive(Debug)]
pub struct InjectionAnalysis {
    pub is_malicious: bool,
    pub affected_databases: Vec<Database>,
}

static DB_PATTERNS: &[(Database, &[&str])] = &[
    (Database::General, GENERAL),
    (Database::Postgres, POSTGRES),
    (Database::Sqlite, SQLITE),
    (Database::DuckDb, DUCKDB),
];

struct CompiledPatterns {
    set: RegexSet,
    index_map: Vec<Database>,
}

static COMPILED: LazyLock<CompiledPatterns> = LazyLock::new(|| {
    let mut all_patterns = Vec::new();
    let mut index_map = Vec::new();

    for (db, patterns) in DB_PATTERNS {
        for &pattern in *patterns {
            all_patterns.push(pattern);
            index_map.push(db.clone());
        }
    }

    CompiledPatterns {
        set: RegexSet::new(&all_patterns).expect("Failed to compile SQL injection patterns"),
        index_map,
    }
});

pub fn matched_patterns(input: &str) -> Vec<usize> {
    COMPILED.set.matches(input).into_iter().collect()
}

pub fn audit_patterns(input: &str) -> InjectionAnalysis {
    let matches: Vec<usize> = COMPILED.set.matches(input).into_iter().collect();

    if matches.is_empty() {
        return InjectionAnalysis {
            is_malicious: false,
            affected_databases: vec![],
        };
    }

    let mut seen = HashSet::new();
    let affected_databases = matches
        .iter()
        .filter_map(|&idx| {
            let db = COMPILED.index_map[idx].clone();
            seen.insert(db.clone()).then_some(db)
        })
        .collect();

    InjectionAnalysis {
        is_malicious: true,
        affected_databases,
    }
}
