//! Hello-world snippet for `ProgrammingLanguageName::Autolisp`.
//!
//! Imported from leachim6/hello-world (`a/AutoLISP.lsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/AutoLISP.lsp>
//! - <https://en.wikipedia.org/wiki/AutoLISP> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Metaprogramming, Procedural, Reflective,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Lisp, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Autolisp => "AutoLISP",
    versions: [Autolisp => "AutoLISP"],
    extensions: [],
}

const AUTOLISP: &str = concat!(r#"(alert ""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Autolisp,
        extensions: &[],
        categories: &[Scripting],
        families: &[Lisp],
        paradigms: &[Procedural, Functional, Reflective, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Autolisp,
            syntax: AUTOLISP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AUTOLISP,
                    [
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"alert "),
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
