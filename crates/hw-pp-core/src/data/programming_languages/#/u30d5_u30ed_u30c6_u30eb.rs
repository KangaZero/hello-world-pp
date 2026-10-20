//! Hello-world snippet for `ProgrammingLanguageName::U30d5U30edU30c6U30eb`.
//!
//! Imported from leachim6/hello-world (`#/プロデル.rdr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E3%83%97%E3%83%AD%E3%83%87%E3%83%AB.rdr>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: U30d5U30edU30c6U30eb => "プロデル",
    versions: [U30d5U30edU30c6U30eb => "プロデル"],
    extensions: [Rdr => "rdr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U30d5U30edU30c6U30eb,
        extensions: &[FileExtension::Rdr],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U30d5U30edU30c6U30eb,
            syntax: concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            )],
        }],
    };
