//! Hello-world snippet for `ProgrammingLanguageName::Clisp`.
//!
//! Imported from leachim6/hello-world (`c/CLISP.lisp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CLISP.lisp>
//! - <https://en.wikipedia.org/wiki/Common_Lisp> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Generic, Metaprogramming, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Clisp => "CLISP",
    versions: [Clisp => "CLISP"],
    extensions: [],
}

const CLISP: &str = concat!(r#"(write-line ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Clisp,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
            Generic,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Clisp,
            syntax: CLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"write"),
                        (TokenKind::Keyword, r"-"),
                        (TokenKind::Plain, r"line "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
