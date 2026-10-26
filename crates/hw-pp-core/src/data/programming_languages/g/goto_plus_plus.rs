//! Hello-world snippet for `ProgrammingLanguageName::GotoPlusPlus`.
//!
//! Imported from leachim6/hello-world (`g/GOTO++.gpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GOTO%2B%2B.gpp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GotoPlusPlus => "GOTO++",
    versions: [GotoPlusPlus => "GOTO++"],
    extensions: [Gpp => "gpp"],
}

const GOTO_PLUS_PLUS: &str = concat!(r"GOTOPRINTDUTEXTE() ; «", output_for_hello_world!(), r"»");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GotoPlusPlus,
        extensions: &[FileExtension::Gpp],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GotoPlusPlus,
            syntax: GOTO_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOTO_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        concat!(r"GOTOPRINTDUTEXTE() ; «", output_for_hello_world!(), r"»"),
                    )],
                )
            },
        }],
    };
