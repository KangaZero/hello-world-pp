//! Hello-world snippet for `ProgrammingLanguageName::Xbasic`.
//!
//! Imported from leachim6/hello-world (`x/XBasic.x` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/XBasic.x>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Xbasic => "XBasic",
    versions: [Xbasic => "XBasic"],
    extensions: [X => "x"],
}

const XBASIC: &str = concat!(
    r#"IMPORT "xst"
DECLARE FUNCTION  Hello ()

FUNCTION  Hello ()
 XstDisplayConsole ()
 PRINT ""#,
    output_for_hello_world!(),
    r#""
END FUNCTION
END PROGRAM"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xbasic,
        extensions: &[FileExtension::X],
        categories: &[Compiled],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xbasic,
            syntax: XBASIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    XBASIC,
                    [
                        (TokenKind::Plain, r"IMPORT "),
                        (TokenKind::String, r#""xst""#),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"DECLARE"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r"  "),
                        (TokenKind::Function, r"Hello"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"FUNCTION"),
                        (TokenKind::Plain, r"  "),
                        (TokenKind::Function, r"Hello"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Function, r"XstDisplayConsole"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
 ",
                        ),
                        (TokenKind::Keyword, r"PRINT"),
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
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"END"),
                        (TokenKind::Plain, r" PROGRAM"),
                    ],
                )
            },
        }],
    };
