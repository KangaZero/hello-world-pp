//! Hello-world snippets for `ProgrammingLanguageName::Gosu`.
//!
//! Imported from leachim6/hello-world (`g/Gosu.gosu`, `g/Gosu.gs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gosu.gosu>
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gosu.gs>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gosu => "Gosu",
    versions: [GosuGosu => "Gosu (.gosu)", GosuGs => "Gosu (.gs)"],
    extensions: [Gosu => "gosu"],
}

const GOSU_GOSU: &str = concat!(r#"print(""#, output_for_hello_world!(), r#" from Gosu!!")"#);
const GOSU_GS: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gosu,
        extensions: &[FileExtension::Gosu],
        categories: &[Scripting],
        families: &[Java],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::GosuGosu,
                syntax: GOSU_GOSU,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        GOSU_GOSU,
                        [
                            (TokenKind::Function, r"print"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#" from Gosu!!""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::GosuGs,
                syntax: GOSU_GS,
                tokenized_syntax: &const {
                    checked_token_lengths(
                        GOSU_GS,
                        [
                            (TokenKind::Function, r"print"),
                            (TokenKind::Punctuation, r"("),
                            (
                                TokenKind::String,
                                concat!(r#"""#, output_for_hello_world!(), r#"""#),
                            ),
                            (TokenKind::Punctuation, r")"),
                        ],
                    )
                },
            },
        ],
    };
