//! Hello-world snippet for `ProgrammingLanguageName::Islisp`.
//!
//! Imported from leachim6/hello-world (`i/ISLISP.lisp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/ISLISP.lisp>
//! - <https://www.wikidata.org/wiki/Q5974027> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, ObjectOriented, Procedural, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Islisp => "ISLISP",
    versions: [Islisp => "ISLISP"],
    extensions: [],
}

const ISLISP: &str = concat!(
    r#"(format (standard-output) ""#,
    output_for_hello_world!(),
    r#"")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Islisp,
        extensions: &[],
        categories: &[Scripting],
        families: &[Lisp],
        paradigms: &[
            Procedural,
            ObjectOriented,
            Functional,
            Reflective,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Islisp,
            syntax: ISLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ISLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"format"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"standard"),
                        (TokenKind::Keyword, r"-"),
                        (TokenKind::Plain, r"output"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
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
