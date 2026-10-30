//! Hello-world snippet for `ProgrammingLanguageName::Yap`.
//!
//! Imported from leachim6/hello-world (PR #1618 `y/YAP.yap`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1618>
//! - <https://en.wikipedia.org/wiki/Prolog> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Logic;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Prolog, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Yap => "YAP",
    versions: [Yap => "YAP"],
    extensions: [Yap => "yap"],
}

const YAP: &str = concat!(
    r":- initialization(main).

main :-
    write('",
    output_for_hello_world!(),
    r"'), nl."
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yap,
        extensions: &[FileExtension::Yap],
        categories: &[Scripting],
        families: &[Prolog],
        paradigms: &[Logic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yap,
            syntax: YAP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    YAP,
                    [
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
                )
            },
        }],
    };
