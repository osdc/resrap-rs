//! Ready-to-use Resrap grammars for common languages.
//!
//! The grammars are embedded in the crate, so they work without touching the
//! filesystem. Languages are available as typed [`Lang`] values or short
//! string names such as `"cpp"`.

use std::fmt;
use std::str::FromStr;

use crate::GrammarLoader;

/// Describes one embedded grammar.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Template {
    /// Short identifier used by the API, for example `"cpp"`.
    pub name: &'static str,
    /// Human-readable language name, for example `"C++"`.
    pub language: &'static str,
    /// One-line summary suitable for a picker.
    pub description: &'static str,
    /// The complete ABNF grammar text.
    pub grammar: &'static str,
}

/// Identifies one of the embedded language templates.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Lang {
    C,
    Cpp,
    Go,
    Haskell,
    Html,
    Java,
    JavaScript,
    Python,
    Rust,
    Sql,
}

impl Lang {
    pub const ALL: [Lang; 10] = [
        Lang::C,
        Lang::Cpp,
        Lang::Go,
        Lang::Haskell,
        Lang::Html,
        Lang::Java,
        Lang::JavaScript,
        Lang::Python,
        Lang::Rust,
        Lang::Sql,
    ];

    /// Returns the short template name, for example `"cpp"`.
    pub fn as_str(self) -> &'static str {
        match self {
            Lang::C => "c",
            Lang::Cpp => "cpp",
            Lang::Go => "go",
            Lang::Haskell => "haskell",
            Lang::Html => "html",
            Lang::Java => "java",
            Lang::JavaScript => "js",
            Lang::Python => "python",
            Lang::Rust => "rust",
            Lang::Sql => "sql",
        }
    }

    /// Reports whether this is an embedded template.
    pub fn valid(self) -> bool {
        get(self).is_ok()
    }

    /// Returns the grammar and metadata for this language.
    pub fn template(self) -> Result<Template, String> {
        get(self)
    }

    /// Parses this template into a Resrap instance under its template name.
    pub fn load<L: GrammarLoader>(self, loader: &mut L) -> Result<(), String> {
        load(loader, self)
    }
}

impl AsRef<str> for Lang {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl fmt::Display for Lang {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl FromStr for Lang {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = normalize(s);
        Lang::ALL
            .iter()
            .copied()
            .find(|lang| lang.as_str() == key)
            .ok_or_else(|| format!("templates: unknown template {:?}", s))
    }
}

struct Entry {
    name: &'static str,
    language: &'static str,
    description: &'static str,
    grammar: &'static str,
}

const TEMPLATES: &[Entry] = &[
    Entry {
        name: "c",
        language: "C",
        description: "Headers, loops, conditionals and printf output",
        grammar: include_str!("../templates/c.g4"),
    },
    Entry {
        name: "cpp",
        language: "C++",
        description: "Classes, helpers, lambdas, exceptions and complete functions",
        grammar: include_str!("../templates/cpp.g4"),
    },
    Entry {
        name: "go",
        language: "Go",
        description: "Package main with imports, loops and fmt output",
        grammar: include_str!("../templates/go.g4"),
    },
    Entry {
        name: "haskell",
        language: "Haskell",
        description: "Module Main with do blocks and list operations",
        grammar: include_str!("../templates/haskell.g4"),
    },
    Entry {
        name: "html",
        language: "HTML",
        description: "Document structure, headings, paragraphs and lists",
        grammar: include_str!("../templates/html.g4"),
    },
    Entry {
        name: "java",
        language: "Java",
        description: "A Main class with a static main method",
        grammar: include_str!("../templates/java.g4"),
    },
    Entry {
        name: "js",
        language: "JavaScript",
        description: "Browser-ready functions, loops and console output",
        grammar: include_str!("../templates/js.g4"),
    },
    Entry {
        name: "python",
        language: "Python",
        description: "A main function with loops and print statements",
        grammar: include_str!("../templates/python.g4"),
    },
    Entry {
        name: "rust",
        language: "Rust",
        description: "A main function with loops, ranges and println!",
        grammar: include_str!("../templates/rust.g4"),
    },
    Entry {
        name: "sql",
        language: "SQL",
        description: "SELECT, INSERT and CREATE TABLE statements",
        grammar: include_str!("../templates/sql.g4"),
    },
];

/// Returns the short names of every embedded template, sorted.
pub fn names() -> Vec<&'static str> {
    TEMPLATES.iter().map(|entry| entry.name).collect()
}

/// Returns every embedded language, sorted by short name.
pub fn langs() -> Vec<Lang> {
    Lang::ALL.to_vec()
}

/// Returns one template. The name is case-insensitive and may include the
/// `.g4` extension. It accepts both strings and [`Lang`] values.
pub fn get<N: AsRef<str>>(name: N) -> Result<Template, String> {
    let key = normalize(name.as_ref());
    TEMPLATES
        .iter()
        .find(|entry| entry.name == key)
        .map(|entry| Template {
            name: entry.name,
            language: entry.language,
            description: entry.description,
            grammar: entry.grammar,
        })
        .ok_or_else(|| {
            format!(
                "templates: unknown template {:?} (available: {})",
                name.as_ref(),
                names().join(", ")
            )
        })
}

/// [`get`] for callers that treat a missing built-in template as a
/// programming error.
pub fn must_get<N: AsRef<str>>(name: N) -> Template {
    get(name).unwrap_or_else(|err| panic!("{}", err))
}

/// Returns every embedded template with its grammar and metadata.
pub fn list() -> Vec<Template> {
    TEMPLATES
        .iter()
        .map(|entry| Template {
            name: entry.name,
            language: entry.language,
            description: entry.description,
            grammar: entry.grammar,
        })
        .collect()
}

/// Parses a template into a Resrap instance under the template name.
pub fn load<L: GrammarLoader, N: AsRef<str>>(loader: &mut L, name: N) -> Result<(), String> {
    load_as(loader, &name, &normalize(name.as_ref()))
}

/// Parses a template into a Resrap instance under a custom grammar name.
pub fn load_as<L: GrammarLoader, N: AsRef<str>>(
    loader: &mut L,
    name: N,
    as_name: &str,
) -> Result<(), String> {
    let template = get(name)?;
    loader.parse_grammar(as_name, template.grammar)
}

impl Template {
    /// Parses this template into a Resrap instance under its template name.
    pub fn load<L: GrammarLoader>(&self, loader: &mut L) -> Result<(), String> {
        loader.parse_grammar(self.name, self.grammar)
    }
}

fn normalize(name: &str) -> String {
    name.trim()
        .trim_end_matches(".g4")
        .trim_end_matches(".G4")
        .to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Resrap;

    #[test]
    fn exposes_all_named_templates() {
        assert_eq!(
            names(),
            vec![
                "c", "cpp", "go", "haskell", "html", "java", "js", "python", "rust", "sql"
            ]
        );
        assert_eq!(langs().len(), 10);
    }

    #[test]
    fn lookup_is_case_insensitive_and_accepts_extension() {
        assert_eq!(get("CPP").unwrap().name, "cpp");
        assert_eq!(get("cpp.g4").unwrap().name, "cpp");
        assert_eq!(get(Lang::Rust).unwrap().name, "rust");
        assert!(get("cobol").is_err());
    }

    #[test]
    fn templates_parse_and_generate() {
        for template in list() {
            assert!(!template.grammar.is_empty());
            assert!(!template.language.is_empty());
            assert!(!template.description.is_empty());

            let mut r = Resrap::new();
            template.load(&mut r).unwrap();

            let mut first: Option<String> = None;
            for seed in [1, 7, 42, 99] {
                let out = r.generate_complete_with_seeded(template.name, "program", seed, 60);
                assert!(
                    !out.trim().is_empty(),
                    "{} seed {} produced empty output",
                    template.name,
                    seed
                );
                if let Some(previous) = first.as_ref() {
                    assert_ne!(
                        out, *previous,
                        "{} seed {} repeated output",
                        template.name, seed
                    );
                } else {
                    first = Some(out);
                }
            }

            let out = r.generate_complete_with_seeded(template.name, "program", 7, 500);
            assert!(
                out.lines().count() >= 20,
                "{} 500-token output is too small ({} lines):\n{}",
                template.name,
                out.lines().count(),
                out
            );
        }
    }

    #[test]
    fn load_as_uses_custom_name() {
        let mut r = Resrap::new();
        load_as(&mut r, Lang::Cpp, "C++").unwrap();
        assert!(
            !r.generate_complete_with_seeded("C++", "program", 1, 40)
                .is_empty()
        );
    }

    #[test]
    fn lang_round_trips() {
        assert_eq!(Lang::from_str("js").unwrap(), Lang::JavaScript);
        assert_eq!(Lang::JavaScript.to_string(), "js");
        assert!(Lang::Cpp.valid());
        assert_eq!(must_get("sql").name, "sql");
    }
}
