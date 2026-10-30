//! Hello-world snippet for `ProgrammingLanguageName::Xlisp`.
//!
//! Imported from leachim6/hello-world (`x/XLisp.xlisp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XLisp.xlisp>
//! - <https://en.wikipedia.org/wiki/Lisp_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Procedural, Reflective,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Xlisp => "XLisp",
    versions: [Xlisp => "XLisp"],
    extensions: [Xlisp => "xlisp"],
}

const XLISP: &str = concat!(
    r#"(DISPLAY ""#,
    output_for_hello_world!(),
    r#"")
(NEWLINE)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xlisp,
        extensions: &[FileExtension::Xlisp],
        categories: &[Scripting],
        families: &[Lisp],
        paradigms: &[Procedural, Functional, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xlisp,
            syntax: XLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"DISPLAY"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"NEWLINE"),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
