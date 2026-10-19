//! Hello-world snippet for `ProgrammingLanguageName::Nial`.
//!
//! Imported from leachim6/hello-world (`n/Nial.ndf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nial.ndf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Nial => "Nial",
    versions: [Nial => "Nial"],
    extensions: [Ndf => "ndf"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nial,
        extensions: &[FileExtension::Ndf],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nial,
            syntax: concat!(
                r"write '",
                output_for_hello_world!(),
                r"';
bye"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"write '",
                    output_for_hello_world!(),
                    r"';
bye"
                ),
            )],
        }],
    };
