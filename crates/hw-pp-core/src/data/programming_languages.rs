//! The programming-language catalog: languages, versions, extensions and their snippets.

use crate::data::{
    common::{DEFAULT_PROGRAMMING_LANGUAGE, unique},
    programming_languages_token::TokenKind,
};

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

name_enum! {
    /// A [programming paradigm](https://en.wikipedia.org/wiki/Programming_paradigm): a style a
    /// language supports. Most languages support several. Recorded from Wikidata's
    /// "programming paradigm" (P3966), Wikipedia's infobox or esolangs.org, with finer
    /// paradigms folded into these.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum ProgrammingParadigm: ParseProgrammingParadigmError("programming paradigm") {
        /// Statements that change program state, step by step (C, Pascal).
        Imperative => "Imperative",
        /// Imperative code organised into procedures (C, Fortran, Go).
        Procedural => "Procedural",
        /// Imperative code built from blocks, loops and conditionals instead of `goto`.
        Structured => "Structured",
        /// Data and behaviour bundled into objects, usually via classes (Java, C#, Smalltalk).
        ObjectOriented => "Object-Oriented",
        /// Object-oriented without classes: objects inherit from other objects (JavaScript, Self, Lua).
        PrototypeBased => "Prototype-Based",
        /// Programs built from (ideally pure) functions and immutable data (Haskell, OCaml, Clojure).
        Functional => "Functional",
        /// Describes what result is wanted rather than how to compute it (SQL, HTML).
        Declarative => "Declarative",
        /// Facts and rules, queried by a solver (Prolog, Datalog).
        Logic => "Logic",
        /// Relations between variables, solved by a constraint solver (`MiniZinc`, `ECLiPSe`).
        Constraint => "Constraint",
        /// Built around simultaneous computations: processes, actors, parallelism (Erlang, Go).
        Concurrent => "Concurrent",
        /// Operations apply to whole arrays at once (APL, J, MATLAB).
        Array => "Array",
        /// Programs inspect and modify their own structure at run time (Smalltalk, Ruby).
        Reflective => "Reflective",
        /// Code written once for many types via type parameters (C++ templates, Ada generics).
        Generic => "Generic",
        /// Control flow driven by events and their handlers (Visual Basic, JavaScript).
        EventDriven => "Event-Driven",
        /// Computations that update as their input streams change (Elm, functional reactive programming).
        Reactive => "Reactive",
        /// Code that writes or transforms code, e.g. macros (Lisp, Rust).
        Metaprogramming => "Metaprogramming",
        /// Programs built from diagrams or blocks instead of text (Scratch, `LabVIEW`).
        Visual => "Visual",
        /// Programs as graphs of operations that fire when their inputs arrive (`LabVIEW`, Lucid).
        Dataflow => "Dataflow",
        /// Operations work on an implicit data stack (Forth, PostScript).
        StackBased => "Stack-Based",
        /// Programs are compositions of functions written one after another (Joy, Factor).
        Concatenative => "Concatenative",
        /// Cross-cutting concerns (logging, security) added separately as aspects (`AspectJ`).
        AspectOriented => "Aspect-Oriented",
        // Esoteric-language styles, as categorised by esolangs.org.
        /// Memory is a tape or grid of cells that a pointer moves over (Brainfuck).
        CellBased => "Cell-Based",
        /// The program is a 2D grid that the instruction pointer travels across (Befunge, ><>).
        TwoDimensional => "Two-Dimensional",
        /// Programs repeatedly rewrite a string with substitution rules (Thue, ///).
        StringRewriting => "String-Rewriting",
        /// Data lives in a queue rather than on a stack (Fueue).
        QueueBased => "Queue-Based",
    }
}

/// One version of a [`ProgrammingLanguage`] and its hello-world snippet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgrammingLanguageVersion {
    /// The version's name.
    pub label: ProgrammingLanguageDialect,
    /// The hello-world source code.
    pub syntax: &'static str,
    /// `syntax` split into highlighted tokens, as each token's kind and byte length; see
    /// [`tokens`](Self::tokens). Lengths, not `&str`s, so the table needs no load-time
    /// relocations; built with the crate-internal `checked_token_lengths`, which checks the
    /// tokens against `syntax` at compile time.
    pub tokenized_syntax: &'static [(TokenKind, usize)],
}

/// One recorded programming language, as declared by its language file.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ProgrammingLanguage {
    /// The language's name.
    pub name: ProgrammingLanguageName,
    /// Its file extensions; may be empty, and the first is the default.
    pub extensions: &'static [FileExtension],
    /// Every category it belongs to.
    pub categories: &'static [ProgrammingLanguageCategory],
    /// Every family it belongs to.
    pub families: &'static [ProgrammingLanguageFamily],
    /// Every paradigm it supports, as recorded by Wikidata, Wikipedia or esolangs.org; empty
    /// when none is recorded.
    pub paradigms: &'static [ProgrammingParadigm],
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

impl ProgrammingLanguageVersion {
    /// Hello-world snippet for the default programming language and its first version.
    ///
    /// # Panics
    ///
    /// Panics if the default programming language has no recorded snippet.
    #[must_use]
    pub fn default_syntax() -> &'static str {
        ProgrammingLanguageVersion::syntax_for(Some(DEFAULT_PROGRAMMING_LANGUAGE), None, None)
            .expect("default programming language must resolve to a snippet")
    }

    /// The hello-world snippet for a programming language; each `None` falls back to a default.
    ///
    /// - `language`: `None` → inferred from `dialect`, then `extension`, else
    ///   [`DEFAULT_PROGRAMMING_LANGUAGE`].
    /// - `dialect`: `None` → the language's first version.
    /// - `extension`: only identifies or checks the language; a shared one (`m`) picks
    ///   the first language alphabetically by file name.
    ///
    /// ```
    /// use hw_pp_core::data::programming_languages::{FileExtension, ProgrammingLanguageVersion};
    /// let python = ProgrammingLanguageVersion::syntax_for(None, None, Some(FileExtension::Py));
    /// assert_eq!(python, Ok(r#"print "Hello, World!""#));
    /// ```
    ///
    /// # Errors
    ///
    /// - [`LookupError::NoOwner`] if `dialect` belongs to no recorded language.
    /// - [`ProgrammingLanguageError::UnknownExtension`] if `extension` belongs to no recorded
    ///   language.
    /// - [`LookupError::UnknownLanguage`] if the language is not recorded.
    /// - [`ProgrammingLanguageError::ExtensionMismatch`] if the language does not use `extension`.
    /// - [`LookupError::UnknownVariant`] if the language does not record `dialect`.
    /// - [`LookupError::NoVariants`] if the language has no versions to default to.
    ///
    /// The [`LookupError`]s are wrapped in [`ProgrammingLanguageError::Lookup`].
    pub fn syntax_for(
        language: Option<ProgrammingLanguageName>,
        dialect: Option<ProgrammingLanguageDialect>,
        extension: Option<FileExtension>,
    ) -> Result<&'static str, ProgrammingLanguageError> {
        ProgrammingLanguageVersion::find(language, dialect, extension).map(|version| version.syntax)
    }

    /// Like [`Self::syntax_for`], but returns the whole version (label, syntax and tokens).
    ///
    /// # Errors
    ///
    /// Same as [`ProgrammingLanguageVersion::syntax_for`].
    pub fn find(
        language: Option<ProgrammingLanguageName>,
        dialect: Option<ProgrammingLanguageDialect>,
        extension: Option<FileExtension>,
    ) -> Result<&'static ProgrammingLanguageVersion, ProgrammingLanguageError> {
        let resolved_language = match (language, dialect, extension) {
            (Some(existing_language), _, _) => existing_language,
            (None, Some(existing_dialect), _) => existing_dialect.language()?,
            (None, None, Some(existing_extension)) => existing_extension.language()?,
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
}

impl FileExtension {
    /// Finds the first language that uses this extension.
    ///
    /// # Errors
    ///
    /// Returns [`ProgrammingLanguageError::UnknownExtension`] if no recorded
    /// language uses the extension.
    pub fn language(self) -> Result<ProgrammingLanguageName, ProgrammingLanguageError> {
        PROGRAMMING_LANGUAGES
            .iter()
            .find(|lang| lang.extensions.contains(&self))
            .map(|lang| lang.name)
            .ok_or(ProgrammingLanguageError::UnknownExtension(self))
    }
}

impl ProgrammingLanguageDialect {
    /// Finds the language that records this version.
    ///
    /// # Errors
    ///
    /// Returns [`LookupError::NoOwner`] if no recorded language has the version.
    pub fn language(self) -> Result<ProgrammingLanguageName, ProgrammingLookupError> {
        PROGRAMMING_LANGUAGES
            .iter()
            .find(|lang| lang.versions.iter().any(|d| d.label == self))
            .map(|lang| lang.name)
            .ok_or(LookupError::NoOwner(self))
    }

    /// The versions of `language` (or of every language if `None`), in catalog order.
    ///
    /// ```
    /// use hw_pp_core::data::programming_languages::{
    ///     ProgrammingLanguageDialect, ProgrammingLanguageName,
    /// };
    /// let versions: Vec<_> =
    ///     ProgrammingLanguageDialect::all_by(Some(ProgrammingLanguageName::Python)).collect();
    /// assert_eq!(
    ///     versions,
    ///     [ProgrammingLanguageDialect::Python2, ProgrammingLanguageDialect::Python3]
    /// );
    /// ```
    pub fn all_by(language: Option<ProgrammingLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            PROGRAMMING_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.versions.iter().map(|v| v.label)),
        )
    }
}

impl ProgrammingLanguageName {
    /// The first recorded extension of this language.
    ///
    /// # Errors
    ///
    /// Returns [`LookupError::UnknownLanguage`] (wrapped) if the language is not
    /// recorded, or [`ProgrammingLanguageError::NoExtensions`] if it has none.
    pub fn default_extension(self) -> Result<FileExtension, ProgrammingLanguageError> {
        let lang = PROGRAMMING_LANGUAGES
            .iter()
            .find(|lang| lang.name == self)
            .ok_or(LookupError::UnknownLanguage(self))?;
        lang.extensions
            .first()
            .copied()
            .ok_or(ProgrammingLanguageError::NoExtensions(self))
    }

    /// The first recorded version of this language.
    ///
    /// # Errors
    ///
    /// Returns [`LookupError::UnknownLanguage`] if the language is not recorded,
    /// or [`LookupError::NoVariants`] if it has no versions.
    pub fn default_version(self) -> Result<ProgrammingLanguageDialect, ProgrammingLookupError> {
        let lang = PROGRAMMING_LANGUAGES
            .iter()
            .find(|lang| lang.name == self)
            .ok_or(LookupError::UnknownLanguage(self))?;
        lang.versions
            .first()
            .map(|version| version.label)
            .ok_or(LookupError::NoVariants(self))
    }

    /// The programming languages matching every given filter, in catalog order; `None` matches
    /// all.
    ///
    /// - `extension`: languages using it.
    /// - `family`: languages in it (e.g. [`ProgrammingLanguageFamily::OCaml`]).
    /// - `category`: languages in it (e.g. [`ProgrammingLanguageCategory::Esolang`]).
    /// - `paradigm`: languages supporting it (e.g. [`ProgrammingParadigm::Functional`]).
    ///
    /// ```
    /// use hw_pp_core::data::programming_languages::{
    ///     FileExtension, ProgrammingLanguageFamily, ProgrammingLanguageName, ProgrammingParadigm,
    /// };
    /// let users_of_m: Vec<_> =
    ///     ProgrammingLanguageName::all_by(Some(FileExtension::M), None, None, None).collect();
    /// assert_eq!(users_of_m, [ProgrammingLanguageName::Matlab, ProgrammingLanguageName::ObjectiveC]);
    /// let ocaml_family: Vec<_> =
    ///     ProgrammingLanguageName::all_by(None, Some(ProgrammingLanguageFamily::OCaml), None, None)
    ///         .collect();
    /// assert!(
    ///     ocaml_family.contains(&ProgrammingLanguageName::FSharp)
    ///         && ocaml_family.contains(&ProgrammingLanguageName::OCaml)
    /// );
    /// assert!(
    ///     ProgrammingLanguageName::all_by(None, None, None, Some(ProgrammingParadigm::Logic))
    ///         .any(|name| name == ProgrammingLanguageName::Prolog)
    /// );
    /// ```
    pub fn all_by(
        extension: Option<FileExtension>,
        family: Option<ProgrammingLanguageFamily>,
        category: Option<ProgrammingLanguageCategory>,
        paradigm: Option<ProgrammingParadigm>,
    ) -> impl Iterator<Item = Self> {
        unique(
            PROGRAMMING_LANGUAGES
                .iter()
                .filter(move |lang| {
                    extension.is_none_or(|e| lang.extensions.contains(&e))
                        && family.is_none_or(|f| lang.families.contains(&f))
                        && category.is_none_or(|c| lang.categories.contains(&c))
                        && paradigm.is_none_or(|p| lang.paradigms.contains(&p))
                })
                .map(|lang| lang.name),
        )
    }
}

impl ProgrammingLanguageCategory {
    /// The categories of `language` (or of every language if `None`), each once, in the order
    /// first recorded.
    ///
    /// ```
    /// use hw_pp_core::data::programming_languages::{
    ///     ProgrammingLanguageCategory, ProgrammingLanguageName,
    /// };
    /// let categories: Vec<_> =
    ///     ProgrammingLanguageCategory::all_by(Some(ProgrammingLanguageName::TypeScript)).collect();
    /// assert_eq!(
    ///     categories,
    ///     [ProgrammingLanguageCategory::Scripting, ProgrammingLanguageCategory::Compiled]
    /// );
    /// ```
    pub fn all_by(language: Option<ProgrammingLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            PROGRAMMING_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.categories.iter().copied()),
        )
    }
}

impl ProgrammingLanguageFamily {
    /// The families of `language` (or of every language if `None`), each once, in the order first
    /// recorded.
    pub fn all_by(language: Option<ProgrammingLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            PROGRAMMING_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.families.iter().copied()),
        )
    }
}

impl ProgrammingParadigm {
    /// The paradigms of `language` (or of every language if `None`), each once, in the order
    /// first recorded.
    ///
    /// ```
    /// use hw_pp_core::data::programming_languages::{ProgrammingLanguageName, ProgrammingParadigm};
    /// assert!(
    ///     ProgrammingParadigm::all_by(Some(ProgrammingLanguageName::Prolog))
    ///         .any(|paradigm| paradigm == ProgrammingParadigm::Logic)
    /// );
    /// ```
    pub fn all_by(language: Option<ProgrammingLanguageName>) -> impl Iterator<Item = Self> {
        unique(
            PROGRAMMING_LANGUAGES
                .iter()
                .filter(move |lang| language.is_none_or(|l| lang.name == l))
                .flat_map(|lang| lang.paradigms.iter().copied()),
        )
    }
}
