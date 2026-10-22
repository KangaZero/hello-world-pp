//! The programming-language catalog: languages, versions, extensions and their snippets.

use std::collections::BTreeSet;

use crate::data::{common::DEFAULT_PROGRAMMING_LANGUAGE, programming_languages_token::TokenKind};

use super::lookup::LookupError;
use super::macros::name_enum;
use thiserror::Error;

name_enum! {
    /// A classification of a programming language; a language may have several.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ProgrammingLanguageCategory: ParseProgrammingLanguageCategoryError("programming language category") {
        /// Esoteric: built as a joke, puzzle or experiment (e.g. Brainfuck).
        Esolang => "Esolang",
        /// Usually interpreted (e.g. Python).
        Scripting => "Scripting",
        /// Usually compiled ahead of time (e.g. C).
        Compiled => "Compiled",
        /// Describes documents or styles rather than computation (e.g. Markdown, SCSS).
        Markup => "Markup",
        /// Mainly of historical interest.
        Historical => "Historical",
        /// Low-level code close to a machine or VM (e.g. MIPS, LLVM IR).
        Assembly => "Assembly",
        /// A library, runtime or framework rather than a language (e.g. Node.js).
        Framework => "Framework",
        /// Retrieves or manipulates data (e.g. GraphQL, MySQL).
        Query => "Query",
        /// Describes settings, builds or infrastructure (e.g. TOML, Dockerfile).
        Configuration => "Configuration",
        /// Describes digital circuits (e.g. VHDL).
        HardwareDescription => "Hardware Description",
        /// Designed for teaching programming.
        Educational => "Educational",
        /// Designed for numerical or scientific work.
        Scientific => "Scientific",
        /// Writes machine-checked proofs.
        ProofAssistant => "Proof Assistant",
        /// Composes or synthesizes music.
        Music => "Music",
    }
}

name_enum! {
    /// A family of related programming languages, as grouped by Wikipedia's
    /// [Programming language families](https://en.wikipedia.org/wiki/Category:Programming_language_families).
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ProgrammingLanguageFamily: ParseProgrammingLanguageFamilyError("programming language family") {
        Ada => "Ada",
        Algol => "ALGOL",
        Apl => "APL",
        Basic => "BASIC",
        C => "C",
        Cpp => "C++",
        /// Languages targeting the .NET runtime (C#, F#, Visual Basic .NET), a platform rather
        /// than a lineage; listed because Wikipedia groups them here.
        DotNet => ".NET",
        Forth => "Forth",
        Fortran => "Fortran",
        Haskell => "Haskell",
        /// Languages from International Computers Limited (e.g. SCL), grouped by maker.
        Icl => "ICL",
        Icon => "Icon",
        Java => "Java",
        JavaScript => "JavaScript",
        Joss => "JOSS",
        Lisp => "Lisp",
        Logo => "Logo",
        Ml => "ML",
        Modula => "Modula",
        Oberon => "Oberon",
        OCaml => "OCaml",
        Pascal => "Pascal",
        Perl => "Perl",
        PlI => "PL/I",
        Prolog => "Prolog",
        Python => "Python",
        R => "R",
        Rexx => "Rexx",
        Ruby => "Ruby",
        Smalltalk => "Smalltalk",
        Snobol => "SNOBOL",
        Sr => "SR",
        Tcl => "Tcl",
        XBase => "xBase",
    }
}

/// Language/version resolution errors for the programming-language catalog.
pub type ProgrammingLookupError = LookupError<ProgrammingLanguageName, ProgrammingLanguageDialect>;

/// Errors that can occur while looking up hello-world source snippets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum ProgrammingLanguageError {
    /// The language or version could not be resolved.
    #[error(transparent)]
    Lookup(#[from] ProgrammingLookupError),

    /// No recorded language uses the extension.
    #[error("no recorded language has extension {0}")]
    UnknownExtension(FileExtension),

    /// The language is recorded but lists no file extensions.
    #[error("{0} has no recorded file extensions")]
    NoExtensions(ProgrammingLanguageName),

    /// The extension was given alongside a language that does not use it.
    #[error("{extension} is not an extension of {language}")]
    ExtensionMismatch {
        /// The resolved language.
        language: ProgrammingLanguageName,
        /// The extension it does not use.
        extension: FileExtension,
    },

    /// Should never happen, but just in case a language has no extensions.
    #[error("no recorded extension for language: {0}")]
    MissingExtension(ProgrammingLanguageName),
}

/// One version of a [`ProgrammingLanguage`] and its hello-world snippet.
#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguageVersion {
    /// The version's name.
    pub label: ProgrammingLanguageDialect,
    /// The hello-world source code.
    pub syntax: &'static str,
    /// `syntax` split into highlighted tokens, which concatenate back to it.
    pub tokenized_syntax: &'static [(TokenKind, &'static str)],
}

/// One recorded programming language, as declared by its language file.
#[derive(Debug, Clone, Copy)]
pub struct ProgrammingLanguage {
    /// The language's name.
    pub name: ProgrammingLanguageName,
    /// Its file extensions; may be empty, and the first is the default.
    pub extensions: &'static [FileExtension],
    /// Every category it belongs to.
    pub categories: &'static [ProgrammingLanguageCategory],
    /// Every family it belongs to.
    pub families: &'static [ProgrammingLanguageFamily],
    /// Its versions; the first is the default.
    pub versions: &'static [ProgrammingLanguageVersion],
}

// Generated by `build.rs` from `programming_languages/<shard>/<name>.rs`:
// one `mod` per language file, the `ProgrammingLanguageName`,
// `ProgrammingLanguageDialect` and `FileExtension` enums, and
// `PROGRAMMING_LANGUAGES`. To add a language, add a file; nothing here changes.
include!(concat!(
    env!("OUT_DIR"),
    "/programming_languages_registry.rs"
));

/// The hello-world snippet for a programming language; each `None` falls back to a default.
///
/// - `language`: `None` → inferred from `dialect`, then `extension`, else
///   [`DEFAULT_PROGRAMMING_LANGUAGE`].
/// - `dialect`: `None` → the language's first version.
/// - `extension`: only identifies or checks the language; a shared one (`m`) picks
///   the first language alphabetically by file name.
///
/// ```
/// use hw_pp_core::data::programming_languages::{FileExtension, get_hello_world_syntax_for};
/// let python = get_hello_world_syntax_for(None, None, Some(FileExtension::Py));
/// assert_eq!(python, Ok(r#"print "Hello, World!""#));
/// ```
///
/// # Errors
///
/// - [`LookupError::NoOwner`] if `dialect` belongs to no recorded language.
/// - [`ProgrammingLanguageError::UnknownExtension`] if `extension` belongs to no recorded language.
/// - [`LookupError::UnknownLanguage`] if the language is not recorded.
/// - [`ProgrammingLanguageError::ExtensionMismatch`] if the language does not use `extension`.
/// - [`LookupError::UnknownVariant`] if the language does not record `dialect`.
/// - [`LookupError::NoVariants`] if the language has no versions to default to.
///
/// The [`LookupError`]s are wrapped in [`ProgrammingLanguageError::Lookup`].
pub fn get_hello_world_syntax_for(
    language: Option<ProgrammingLanguageName>,
    dialect: Option<ProgrammingLanguageDialect>,
    extension: Option<FileExtension>,
) -> Result<&'static str, ProgrammingLanguageError> {
    get_hello_world_version_for(language, dialect, extension).map(|version| version.syntax)
}

/// Like [`get_hello_world_syntax_for`], but returns the whole version (label, syntax and tokens).
///
/// # Errors
///
/// Same as [`get_hello_world_syntax_for`].
pub fn get_hello_world_version_for(
    language: Option<ProgrammingLanguageName>,
    dialect: Option<ProgrammingLanguageDialect>,
    extension: Option<FileExtension>,
) -> Result<&'static ProgrammingLanguageVersion, ProgrammingLanguageError> {
    let resolved_language = match (language, dialect, extension) {
        (Some(existing_language), _, _) => existing_language,
        (None, Some(existing_dialect), _) => {
            get_programming_language_name_from_dialect(existing_dialect)?
        }
        (None, None, Some(existing_extension)) => {
            get_programming_language_name_from_extension(existing_extension)?
        }
        (None, None, None) => DEFAULT_PROGRAMMING_LANGUAGE,
    };
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == resolved_language)
        .ok_or(LookupError::UnknownLanguage(resolved_language))?;

    if let Some(mismatched) = extension.filter(|ext| !lang.extensions.contains(ext)) {
        return Err(ProgrammingLanguageError::ExtensionMismatch {
            language: resolved_language,
            extension: mismatched,
        });
    }

    let version = match dialect {
        Some(wanted) => lang
            .versions
            .iter()
            .find(|version| version.label == wanted)
            .ok_or(LookupError::UnknownVariant {
                language: resolved_language,
                variant: wanted,
            })?,
        None => lang
            .versions
            .first()
            .ok_or(LookupError::NoVariants(resolved_language))?,
    };
    Ok(version)
}

/// Finds the first language that uses `extension`.
///
/// # Errors
///
/// Returns [`ProgrammingLanguageError::UnknownExtension`] if no recorded
/// language uses the extension.
pub fn get_programming_language_name_from_extension(
    extension: FileExtension,
) -> Result<ProgrammingLanguageName, ProgrammingLanguageError> {
    PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.extensions.contains(&extension))
        .map(|lang| lang.name)
        .ok_or(ProgrammingLanguageError::UnknownExtension(extension))
}

/// Finds the language that records the version `dialect`.
///
/// # Errors
///
/// Returns [`LookupError::NoOwner`] if no recorded language has the version.
pub fn get_programming_language_name_from_dialect(
    dialect: ProgrammingLanguageDialect,
) -> Result<ProgrammingLanguageName, ProgrammingLookupError> {
    PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.versions.iter().any(|d| d.label == dialect))
        .map(|lang| lang.name)
        .ok_or(LookupError::NoOwner(dialect))
}

/// The first recorded extension of `language`.
///
/// # Errors
///
/// Returns [`LookupError::UnknownLanguage`] (wrapped) if the language is not
/// recorded, or [`ProgrammingLanguageError::NoExtensions`] if it has none.
pub fn get_default_extension_from_programming_language_name(
    language: ProgrammingLanguageName,
) -> Result<FileExtension, ProgrammingLanguageError> {
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(LookupError::UnknownLanguage(language))?;
    lang.extensions
        .first()
        .copied()
        .ok_or(ProgrammingLanguageError::NoExtensions(language))
}

/// The first recorded version of `language`.
///
/// # Errors
///
/// Returns [`LookupError::UnknownLanguage`] if the language is not recorded,
/// or [`LookupError::NoVariants`] if it has no versions.
pub fn get_default_programming_language_dialect_from_language_name(
    language: ProgrammingLanguageName,
) -> Result<ProgrammingLanguageDialect, ProgrammingLookupError> {
    let lang = PROGRAMMING_LANGUAGES
        .iter()
        .find(|lang| lang.name == language)
        .ok_or(LookupError::UnknownLanguage(language))?;
    lang.versions
        .first()
        .map(|version| version.label)
        .ok_or(LookupError::NoVariants(language))
}

/// Display names of the programming languages matching every given filter; `None` matches all.
///
/// - `extension`: languages using it.
/// - `family`: languages in it (e.g. [`ProgrammingLanguageFamily::OCaml`]).
/// - `category`: languages in it (e.g. [`ProgrammingLanguageCategory::Esolang`]).
///
/// ```
/// use hw_pp_core::data::programming_languages::{
///     FileExtension, ProgrammingLanguageFamily, get_all_programming_languages_as_str_by,
/// };
/// let users_of_m = get_all_programming_languages_as_str_by(Some(FileExtension::M), None, None);
/// assert_eq!(users_of_m, ["MATLAB", "Objective-C"]);
/// let ocaml_family =
///     get_all_programming_languages_as_str_by(None, Some(ProgrammingLanguageFamily::OCaml), None);
/// assert!(ocaml_family.contains(&"F#") && ocaml_family.contains(&"OCaml"));
/// ```
#[must_use]
pub fn get_all_programming_languages_as_str_by(
    extension: Option<FileExtension>,
    family: Option<ProgrammingLanguageFamily>,
    category: Option<ProgrammingLanguageCategory>,
) -> Vec<&'static str> {
    PROGRAMMING_LANGUAGES
        .iter()
        .filter(|lang| {
            extension.is_none_or(|e| lang.extensions.contains(&e))
                && family.is_none_or(|f| lang.families.contains(&f))
                && category.is_none_or(|c| lang.categories.contains(&c))
        })
        .map(|lang| lang.name.as_str())
        .collect()
}

/// Display names of the versions of `language`, or of every language if `None`.
///
/// ```
/// use hw_pp_core::data::programming_languages::{ProgrammingLanguageName, get_all_programming_versions_as_str_by};
/// let versions = get_all_programming_versions_as_str_by(Some(ProgrammingLanguageName::Python));
/// assert_eq!(versions, ["Python 2", "Python 3"]);
/// ```
#[must_use]
pub fn get_all_programming_versions_as_str_by(
    language: Option<ProgrammingLanguageName>,
) -> Vec<&'static str> {
    PROGRAMMING_LANGUAGES
        .iter()
        .filter(|lang| language.is_none_or(|l| lang.name == l))
        .flat_map(|lang| lang.versions.iter().map(|v| v.label.as_str()))
        .collect()
}

/// Display names of the categories of `language` (or of all languages if `None`), sorted, without duplicates.
///
/// ```
/// use hw_pp_core::data::programming_languages::{
///     ProgrammingLanguageName, get_all_programming_language_categories_as_str_by,
/// };
/// let categories =
///     get_all_programming_language_categories_as_str_by(Some(ProgrammingLanguageName::TypeScript));
/// assert_eq!(categories, ["Compiled", "Scripting"]);
/// ```
#[must_use]
pub fn get_all_programming_language_categories_as_str_by(
    language: Option<ProgrammingLanguageName>,
) -> Vec<&'static str> {
    PROGRAMMING_LANGUAGES
        .iter()
        .filter(|lang| language.is_none_or(|l| lang.name == l))
        .flat_map(|lang| lang.categories.iter().map(|c| c.as_str()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

/// Display names of the families of `language` (or of all languages if `None`), sorted, without duplicates.
#[must_use]
pub fn get_all_programming_language_families_as_str_by(
    language: Option<ProgrammingLanguageName>,
) -> Vec<&'static str> {
    PROGRAMMING_LANGUAGES
        .iter()
        .filter(|lang| language.is_none_or(|l| lang.name == l))
        .flat_map(|lang| lang.families.iter().map(|f| f.as_str()))
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}
