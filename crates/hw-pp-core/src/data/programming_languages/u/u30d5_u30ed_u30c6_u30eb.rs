//! Hello-world snippet for `ProgrammingLanguageName::U30d5U30edU30c6U30eb`.
//!
//! Imported from leachim6/hello-world (`#/プロデル.rdr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

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
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U30d5U30edU30c6U30eb,
            syntax: concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"「", output_for_hello_world!(), r"」と出力する"),
            )],
        }],
    };
