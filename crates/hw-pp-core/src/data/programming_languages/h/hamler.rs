//! Hello-world snippet for `ProgrammingLanguageName::Hamler`.
//!
//! Imported from leachim6/hello-world (`h/Hamler.hm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hamler.hm>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Hamler => "Hamler",
    versions: [Hamler => "Hamler"],
    extensions: [Hm => "hm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hamler,
        extensions: &[FileExtension::Hm],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hamler,
            syntax: concat!(
                r#"module Main where
import Prelude
main = print ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"module Main where
import Prelude
main = print ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
