//! Hello-world snippet for `ProgrammingLanguageName::Casl2`.
//!
//! Imported from leachim6/hello-world (`c/CASL2.cas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CASL2.cas>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Casl2 => "CASL2",
    versions: [Casl2 => "CASL2"],
    extensions: [Cas => "cas"],
}

const CASL2: &str = concat!(
    r"MAIN    START
        OUT     MSG,LEN
        RET
MSG     DC      '",
    output_for_hello_world!(),
    r"'
LEN     DC      18
        END"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Casl2,
        extensions: &[FileExtension::Cas],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Casl2,
            syntax: CASL2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CASL2,
                    [
                        (TokenKind::Keyword, r"MAIN"),
                        (
                            TokenKind::Plain,
                            r"    START
        ",
                        ),
                        (TokenKind::Keyword, r"OUT"),
                        (TokenKind::Plain, r"     MSG"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"LEN
        ",
                        ),
                        (TokenKind::Keyword, r"RET"),
                        (
                            TokenKind::Plain,
                            r"
MSG     ",
                        ),
                        (TokenKind::Keyword, r"DC"),
                        (TokenKind::Plain, r"      "),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (
                            TokenKind::Plain,
                            r"
LEN     ",
                        ),
                        (TokenKind::Keyword, r"DC"),
                        (TokenKind::Plain, r"      "),
                        (TokenKind::Number, r"18"),
                        (
                            TokenKind::Plain,
                            r"
        ",
                        ),
                        (TokenKind::Keyword, r"END"),
                    ],
                )
            },
        }],
    };
