//! Hello-world snippet for `ProgrammingLanguageName::Yap`.
//!
//! Imported from leachim6/hello-world (PR #1618 `y/YAP.yap`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1618>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Prolog, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Yap => "YAP",
    versions: [Yap => "YAP"],
    extensions: [Yap => "yap"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yap,
        extensions: &[FileExtension::Yap],
        categories: &[Scripting],
        families: &[Prolog],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yap,
            syntax: concat!(
                r":- initialization(main).

main :-
    write('",
                output_for_hello_world!(),
                r"'), nl."
            ),
            tokenized_syntax: &[
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r"- "),
                (TokenKind::Keyword, r"initialization"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Keyword, r"main"),
                (TokenKind::Punctuation, r")."),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"main"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r":"),
                (
                    TokenKind::Plain,
                    r"-
    ",
                ),
                (TokenKind::Keyword, r"write"),
                (TokenKind::Punctuation, r"("),
                (
                    TokenKind::String,
                    concat!(r"'", output_for_hello_world!(), r"'"),
                ),
                (TokenKind::Punctuation, r"),"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"nl"),
                (TokenKind::Punctuation, r"."),
            ],
        }],
    };
