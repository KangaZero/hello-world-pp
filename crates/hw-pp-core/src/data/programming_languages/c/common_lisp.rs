//! Hello-world snippet for `ProgrammingLanguageName::CommonLisp`.
//!
//! Sources:
//! - <https://lisp-lang.org/learn/getting-started/>
//! - <https://www.lispworks.com/documentation/HyperSpec/Body/f_format.htm>
//! - <https://www.wikidata.org/wiki/Q849146> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Generic, Metaprogramming, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: CommonLisp => "Common Lisp",
    versions: [AnsiCommonLisp => "ANSI Common Lisp"],
    extensions: [Lisp => "lisp", Lsp => "lsp", Cl => "cl"],
}

const ANSI_COMMON_LISP: &str = concat!(r#"(format t ""#, output_for_hello_world!(), r#"~%")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CommonLisp,
        extensions: &[FileExtension::Lisp, FileExtension::Lsp, FileExtension::Cl],
        categories: &[Compiled, Historical],
        families: &[Lisp],
        paradigms: &[
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
            Generic,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AnsiCommonLisp,
            syntax: ANSI_COMMON_LISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANSI_COMMON_LISP,
                    [
                        (TokenKind::Punctuation, "("),
                        (TokenKind::Function, "format"),
                        (TokenKind::Plain, " t "),
                        (
                            TokenKind::String,
                            concat!("\"", output_for_hello_world!(), "~%\""),
                        ),
                        (TokenKind::Punctuation, ")"),
                    ],
                )
            },
        }],
    };
