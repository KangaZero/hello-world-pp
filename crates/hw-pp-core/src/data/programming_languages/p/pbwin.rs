//! Hello-world snippet for `ProgrammingLanguageName::Pbwin`.
//!
//! Imported from leachim6/hello-world (`p/PBWin.bas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PBWin.bas>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pbwin => "PBWin",
    versions: [Pbwin => "PBWin"],
    extensions: [],
}

const PBWIN: &str = concat!(
    r#"#COMPILE EXE
#DIM ALL

FUNCTION PBMAIN () AS LONG

    MSGBOX ""#,
    output_for_hello_world!(),
    r#""

END FUNCTION"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pbwin,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pbwin,
            syntax: PBWIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PBWIN,
                    [
                        (
                            TokenKind::Plain,
                            r"#COMPILE EXE
#",
                        ),
                        (TokenKind::Keyword, r"DIM"),
                        (
                            TokenKind::Plain,
                            r" ALL

",
                        ),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"PBMAIN"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"AS"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"LONG"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"MSGBOX"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"FUNCTION"),
                    ],
                )
            },
        }],
    };
