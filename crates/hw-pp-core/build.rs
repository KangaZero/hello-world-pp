//! Generates each catalog's enums, `mod`s and table from its language files
//! (`src/data/<catalog>/<shard>/<name>.rs`, read via their `language_names!`
//! block), so adding a language means adding one file. The shard is the first
//! letter of the display name, or `#` when that is not a letter (`0587`, `μλ`).

use std::collections::{HashMap, HashSet};
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

// Shared with the library, so names are folded exactly as `FromStr` folds them.
#[path = "src/data/normalize.rs"]
#[expect(
    dead_code,
    reason = "`folded_eq` is only used by the library's `FromStr`"
)]
mod normalize;
use normalize::normalize_text;

/// The macro call whose body declares a language file's names.
const MARKER: &str = "language_names!";

/// Rust keywords (strict, reserved and edition-2024), which cannot name a module or variant.
const RUST_KEYWORDS: &[&str] = &[
    "abstract", "as", "async", "await", "become", "box", "break", "const", "continue", "crate",
    "do", "dyn", "else", "enum", "extern", "false", "final", "fn", "for", "gen", "if", "impl",
    "in", "let", "loop", "macro", "match", "mod", "move", "mut", "override", "priv", "pub", "ref",
    "return", "self", "Self", "static", "struct", "super", "trait", "true", "try", "type",
    "typeof", "unsafe", "unsized", "use", "virtual", "where", "while", "yield",
];

/// One enum generated from a `language_names!` key.
struct EnumSpec {
    /// Key in `language_names!` that lists this enum's variants.
    key: &'static str,
    /// The enum's `///` doc comment, as emitted into the generated source.
    doc: &'static str,
    /// `Name: ParseError("kind")`, as `name_enum!` expects.
    header: &'static str,
    /// Whether several files may declare the same variant (e.g. shared scripts).
    shared: bool,
    /// Whether a file may leave the list empty (e.g. sign languages have no script).
    allow_empty: bool,
}

/// One catalog: a folder of language files and what to generate from it.
struct Catalog {
    /// Folder under the crate root, e.g. `src/data/programming_languages`.
    dir: &'static str,
    /// Output file name in `OUT_DIR`.
    output: &'static str,
    /// `language:` (a single pair) comes first, then the list keys.
    enums: [EnumSpec; 3],
    /// The table's `///` doc comment, as emitted into the generated source.
    table_doc: &'static str,
    /// `NAME: &[Type]`.
    table: &'static str,
}

/// The programming- and human-language catalogs.
const CATALOGS: [Catalog; 2] = [
    Catalog {
        dir: "src/data/programming_languages",
        output: "programming_languages_registry.rs",
        enums: [
            EnumSpec {
                key: "language",
                doc: "/// A recorded programming language.",
                header: "ProgrammingLanguageName: ParseProgrammingLanguageNameError(\"programming language\")",
                shared: false,
                allow_empty: false,
            },
            EnumSpec {
                key: "versions",
                doc: "/// A recorded version or standard of a programming language.",
                header: "ProgrammingLanguageDialect: ParseLanguageDialectError(\"language version\")",
                shared: false,
                allow_empty: false,
            },
            EnumSpec {
                key: "extensions",
                doc: "/// A source file extension, without the leading dot. Several languages may\n    \
                      /// share one (e.g. `m` is both Objective-C and MATLAB).",
                header: "FileExtension: ParseFileExtensionError(\"file extension\")",
                shared: true,
                // Many languages have no conventional extension.
                allow_empty: true,
            },
        ],
        table_doc: "/// Every recorded programming language, in alphabetical order of file name.",
        table: "PROGRAMMING_LANGUAGES: &[ProgrammingLanguage]",
    },
    Catalog {
        dir: "src/data/human_languages",
        output: "human_languages_registry.rs",
        enums: [
            EnumSpec {
                key: "language",
                doc: "/// A recorded human language.",
                header: "HumanLanguageName: ParseHumanLanguageNameError(\"language\")",
                shared: false,
                allow_empty: false,
            },
            EnumSpec {
                key: "dialects",
                doc: "/// A spoken variety of a recorded human language.",
                header: "DialectName: ParseDialectNameError(\"dialect\")",
                shared: false,
                allow_empty: false,
            },
            EnumSpec {
                key: "scripts",
                doc: "/// A writing system used by at least one recorded human language.",
                header: "ScriptName: ParseScriptNameError(\"script\")",
                shared: true,
                allow_empty: true,
            },
        ],
        table_doc: "/// Every recorded human language, in alphabetical order of file name.",
        table: "HUMAN_LANGUAGES: &[HumanLanguage]",
    },
];

/// One `Variant => "Display Name"` pair, optionally with `| "Endonym"`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Named {
    /// The enum variant's identifier.
    variant: String,
    /// The display name as written in the source, including its quotes.
    display: String,
    /// The endonym as written in the source, including its quotes.
    endonym: Option<String>,
}

/// The declarations read from one language file.
#[derive(Debug)]
struct LanguageFile {
    /// The module name, from the file stem.
    module: String,
    /// The file's path, for `#[path]` and error messages.
    path: PathBuf,
    /// `language_names!` key → its pairs (`language` has exactly one).
    fields: HashMap<String, Vec<Named>>,
}

fn main() {
    if let Err(error) = run() {
        eprintln!("error: {error}");
        std::process::exit(1);
    }
}

/// Writes each catalog's registry to `OUT_DIR`.
fn run() -> Result<(), String> {
    println!("cargo::rerun-if-changed=build.rs");
    let manifest_dir = PathBuf::from(env("CARGO_MANIFEST_DIR")?);
    let out_dir = PathBuf::from(env("OUT_DIR")?);
    for catalog in &CATALOGS {
        // A directory path makes Cargo rescan it recursively, so new files count too.
        println!("cargo::rerun-if-changed={}", catalog.dir);
        let languages = language_files(&manifest_dir.join(catalog.dir))?
            .iter()
            .map(|path| read_language_file(catalog, path))
            .collect::<Result<Vec<_>, _>>()?;
        let mut modules = HashSet::new();
        if let Some(file) = languages.iter().find(|f| !modules.insert(&f.module)) {
            return Err(format!(
                "{}: another folder already has a `{}.rs`; module names must be unique",
                file.path.display(),
                file.module
            ));
        }
        let out = out_dir.join(catalog.output);
        std::fs::write(&out, generate(catalog, &languages)?)
            .map_err(|e| format!("writing {}: {e}", out.display()))?;
    }
    Ok(())
}

/// Reads the environment variable `name`, naming it in the error.
fn env(name: &str) -> Result<String, String> {
    std::env::var(name).map_err(|e| format!("{name}: {e}"))
}

/// `<dir>/<shard>/<name>.rs`, sorted by file name.
fn language_files(dir: &Path) -> Result<Vec<PathBuf>, String> {
    let read_dir = |path: &Path| {
        std::fs::read_dir(path)
            .map_err(|e| format!("reading {}: {e}", path.display()))?
            .map(|entry| entry.map(|e| e.path()).map_err(|e| e.to_string()))
            .collect::<Result<Vec<_>, String>>()
    };
    let mut files = Vec::new();
    for shard in read_dir(dir)?.into_iter().filter(|p| p.is_dir()) {
        files.extend(
            read_dir(&shard)?
                .into_iter()
                .filter(|p| p.extension().is_some_and(|ext| ext == "rs")),
        );
    }
    files.sort_by_key(|path| path.file_name().map(ToOwned::to_owned));
    Ok(files)
}

/// Reads one language file and checks its `language_names!` block against `catalog`.
fn read_language_file(catalog: &Catalog, path: &Path) -> Result<LanguageFile, String> {
    let at = |message: String| format!("{}: {message}", path.display());
    let module = module_name(path).map_err(at)?;
    let source = std::fs::read_to_string(path).map_err(|e| at(e.to_string()))?;
    let block =
        marker_block(&source).ok_or_else(|| at(format!("no `{MARKER} {{ ... }}` block")))?;
    let fields = parse_fields(&tokenize(block).map_err(at)?).map_err(at)?;

    for spec in &catalog.enums {
        let pairs = fields
            .get(spec.key)
            .ok_or_else(|| at(format!("`{MARKER}` is missing `{}:`", spec.key)))?;
        if spec.key == "language" && pairs.len() != 1 {
            return Err(at(
                "`language:` must be a single `Variant => \"Name\"`".to_owned()
            ));
        }
        if let Some(keyword) = pairs
            .iter()
            .find(|p| RUST_KEYWORDS.contains(&p.variant.as_str()))
        {
            return Err(at(format!(
                "`{}: {}` is a Rust keyword; use e.g. `{}Lang`",
                spec.key, keyword.variant, keyword.variant
            )));
        }
        if pairs.is_empty() && !spec.allow_empty {
            return Err(at(format!("`{}:` needs at least one entry", spec.key)));
        }
    }
    let display = &fields["language"][0].display;
    let expected = shard_for(display);
    if path.parent().and_then(Path::file_name) != Some(expected.as_ref()) {
        return Err(at(format!(
            "{display} must live in the `{expected}` folder"
        )));
    }
    if let Some(key) = fields
        .keys()
        .find(|key| catalog.enums.iter().all(|spec| spec.key != key.as_str()))
    {
        return Err(at(format!("`{MARKER}` has unknown key `{key}:`")));
    }
    Ok(LanguageFile {
        module,
        path: path.to_owned(),
        fields,
    })
}

/// The file's stem, checked to be a `snake_case` module name.
fn module_name(path: &Path) -> Result<String, String> {
    let module = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or("file name is not valid UTF-8")?
        .to_owned();
    if !module.starts_with(|c: char| c.is_ascii_lowercase())
        || !module
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    {
        return Err(format!("`{module}` must be a snake_case module name"));
    }
    if RUST_KEYWORDS.contains(&module.as_str()) {
        return Err(format!(
            "`{module}` is a Rust keyword; rename the file to `{module}_lang.rs`"
        ));
    }
    Ok(module)
}

/// The folder for a quoted display name: its first letter once folded (`Ölang` → `o`),
/// or `#` for names starting with anything else (`"0587"`, `"μλ"`, `"!"`).
fn shard_for(quoted: &str) -> String {
    match fold(quoted).chars().next() {
        Some(first) if first.is_ascii_alphabetic() => first.to_string(),
        _ => "#".to_owned(),
    }
}

/// The text between the braces of the `language_names! { ... }` block.
///
/// Takes the first `language_names!` anywhere in the file, comments included.
/// Comments inside the block are not skipped, so `tokenize` rejects them.
fn marker_block(source: &str) -> Option<&str> {
    let start = source.find(MARKER)?;
    let open = start + source[start..].find('{')?;
    let mut depth = 0;
    let mut in_string = false;
    let mut escaped = false;
    for (offset, c) in source[open..].char_indices() {
        if in_string {
            match (escaped, c) {
                (true, _) => escaped = false,
                (false, '\\') => escaped = true,
                (false, '"') => in_string = false,
                _ => {}
            }
            continue;
        }
        match c {
            '"' => in_string = true,
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(&source[open + 1..open + offset]);
                }
            }
            _ => {}
        }
    }
    None
}

/// One lexical token of a `language_names!` body.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Token {
    /// A key or variant name.
    Ident(String),
    /// A string literal, kept with its quotes and escapes.
    Str(String),
    /// `=>`.
    Arrow,
    /// One of `[ ] , : |`.
    Punct(char),
}

/// Splits a `language_names!` body into identifiers, string literals, `=>` and `[ ] , : |`.
fn tokenize(text: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = text.char_indices().peekable();
    while let Some((start, c)) = chars.next() {
        match c {
            c if c.is_whitespace() => {}
            '=' if chars.peek().is_some_and(|&(_, next)| next == '>') => {
                chars.next();
                tokens.push(Token::Arrow);
            }
            '[' | ']' | ',' | ':' | '|' => tokens.push(Token::Punct(c)),
            '"' => {
                let mut escaped = false;
                let end = chars
                    .by_ref()
                    .find(|&(_, c)| {
                        let closes = !escaped && c == '"';
                        escaped = !escaped && c == '\\';
                        closes
                    })
                    .map(|(i, _)| i)
                    .ok_or("unterminated string literal")?;
                tokens.push(Token::Str(text[start..=end].to_owned()));
            }
            c if c.is_ascii_alphabetic() || c == '_' => {
                let mut end = start + c.len_utf8();
                while let Some(&(i, next)) = chars.peek() {
                    if !(next.is_ascii_alphanumeric() || next == '_') {
                        break;
                    }
                    end = i + next.len_utf8();
                    chars.next();
                }
                tokens.push(Token::Ident(text[start..end].to_owned()));
            }
            other => return Err(format!("unexpected `{other}` in `{MARKER}`")),
        }
    }
    Ok(tokens)
}

/// `key: Variant => "Name"` or `key: [Variant => "Name", ...]`, comma-separated; any
/// pair may add `| "Endonym"`.
fn parse_fields(tokens: &[Token]) -> Result<HashMap<String, Vec<Named>>, String> {
    let mut fields = HashMap::new();
    let mut rest = tokens;
    while let [Token::Ident(key), Token::Punct(':'), after @ ..] = rest {
        let (values, after) = if let [Token::Punct('['), inner @ ..] = after {
            let close = inner
                .iter()
                .position(|t| *t == Token::Punct(']'))
                .ok_or_else(|| format!("`{key}:` is missing `]`"))?;
            (parse_pairs(&inner[..close])?, &inner[close + 1..])
        } else {
            // A single `Variant => "Name"` or `Variant => "Name" | "Endonym"`, up to the next `,`.
            let end = after
                .iter()
                .position(|t| *t == Token::Punct(','))
                .unwrap_or(after.len());
            (parse_pairs(&after[..end])?, &after[end..])
        };
        if fields.insert(key.clone(), values).is_some() {
            return Err(format!("`{key}:` appears twice"));
        }
        rest = after.strip_prefix(&[Token::Punct(',')]).unwrap_or(after);
    }
    if rest.is_empty() {
        Ok(fields)
    } else {
        Err(format!("unexpected {:?} in `{MARKER}`", rest[0]))
    }
}

/// Parses `A => "a", B => "b", C => "c" | "see"` (trailing comma allowed) into pairs.
fn parse_pairs(tokens: &[Token]) -> Result<Vec<Named>, String> {
    tokens
        .split(|t| *t == Token::Punct(','))
        .filter(|pair| !pair.is_empty())
        .map(|pair| match pair {
            [Token::Ident(variant), Token::Arrow, Token::Str(display)] => Ok(Named {
                variant: variant.clone(),
                display: display.clone(),
                endonym: None,
            }),
            [
                Token::Ident(variant),
                Token::Arrow,
                Token::Str(display),
                Token::Punct('|'),
                Token::Str(endonym),
            ] => Ok(Named {
                variant: variant.clone(),
                display: display.clone(),
                endonym: Some(endonym.clone()),
            }),
            other => Err(format!("expected `Variant => \"Name\"`, found {other:?}")),
        })
        .collect()
}

/// Collects one enum's variants across all files, rejecting conflicts:
/// - a variant declared twice, unless the enum is `shared` and both
///   declarations have the same display name and endonym;
/// - an endonym that folds to its own display name;
/// - two names (display names or endonyms) that fold to the same text, since
///   `FromStr` could not tell them apart.
fn merge_variants(spec: &EnumSpec, languages: &[LanguageFile]) -> Result<Vec<Named>, String> {
    let mut variants: Vec<Named> = Vec::new();
    let mut owners: HashMap<&str, &Path> = HashMap::new();
    // folded name → (variant, name as written)
    let mut taken: HashMap<String, (&str, &str)> = HashMap::new();
    for file in languages {
        for named in &file.fields[spec.key] {
            let declared =
                |owner: &Path| format!("`{}: {}` in {}", spec.key, named.variant, owner.display());
            if let Some(existing) = variants.iter().find(|v| v.variant == named.variant) {
                let same = existing.display == named.display && existing.endonym == named.endonym;
                if spec.shared && same {
                    continue;
                }
                let owner = owners[named.variant.as_str()];
                return Err(if spec.shared {
                    format!(
                        "{} and {} give it different names",
                        declared(owner),
                        file.path.display()
                    )
                } else {
                    format!(
                        "{} is declared again in {}",
                        declared(owner),
                        file.path.display()
                    )
                });
            }
            if let Some(endonym) = &named.endonym
                && fold(endonym) == fold(&named.display)
            {
                return Err(format!(
                    "{}: endonym {endonym} is just its display name {}",
                    declared(&file.path),
                    named.display
                ));
            }
            for name in std::iter::once(&named.display).chain(&named.endonym) {
                if let Some((other, other_name)) = taken.get(&fold(name)) {
                    return Err(format!(
                        "{}: {name} and `{other}`'s {other_name} are the same once case, \
                         accents and `-`/`_` are ignored, so parsing would be ambiguous",
                        declared(&file.path)
                    ));
                }
            }
            for name in std::iter::once(&named.display).chain(&named.endonym) {
                taken.insert(fold(name), (&named.variant, name));
            }
            owners.insert(&named.variant, &file.path);
            variants.push(named.clone());
        }
    }
    Ok(variants)
}

/// A quoted name from the source (`"Bokmål"`) folded like `FromStr` folds input.
///
/// Only the quotes are stripped; escapes such as `\u{e9}` are compared as written.
fn fold(quoted: &str) -> String {
    normalize_text(quoted.trim_matches('"'))
}

/// Builds `catalog`'s registry source: one `mod` per file, the enums, then the table.
fn generate(catalog: &Catalog, languages: &[LanguageFile]) -> Result<String, String> {
    let mut out = format!(
        "// @generated by hw-pp-core/build.rs from {}/ - do not edit.\n\n",
        catalog.dir
    );
    for language in languages {
        let path = language.path.to_str().ok_or("non-UTF-8 path")?;
        writeln!(out, "#[path = {path:?}]\nmod {};", language.module).map_err(|e| e.to_string())?;
    }
    for spec in &catalog.enums {
        write_enum(
            &mut out,
            spec.doc,
            spec.header,
            &merge_variants(spec, languages)?,
        )?;
    }
    writeln!(
        out,
        "\n{}\npub static {} = &[",
        catalog.table_doc, catalog.table
    )
    .map_err(|e| e.to_string())?;
    for language in languages {
        writeln!(out, "    {}::LANGUAGE,", language.module).map_err(|e| e.to_string())?;
    }
    out.push_str("];\n");
    Ok(out)
}

/// Appends a `name_enum!` call to `out`.
///
/// - `doc`: the enum's doc comment lines.
/// - `header`: `Name: ParseError("kind")`.
/// - `variants`: the `Variant => "Display Name"` pairs (with any endonyms), in order.
fn write_enum(out: &mut String, doc: &str, header: &str, variants: &[Named]) -> Result<(), String> {
    writeln!(
        out,
        "\nname_enum! {{\n    {doc}\n    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]\n    pub enum {header} {{"
    )
    .map_err(|e| e.to_string())?;
    for Named {
        variant,
        display,
        endonym,
    } in variants
    {
        let endonym = endonym
            .as_ref()
            .map(|endo| format!(" | {endo}"))
            .unwrap_or_default();
        writeln!(out, "        {variant} => {display}{endonym},").map_err(|e| e.to_string())?;
    }
    out.push_str("    }\n}\n");
    Ok(())
}
