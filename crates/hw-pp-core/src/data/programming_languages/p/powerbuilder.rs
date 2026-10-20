//! Hello-world snippet for `ProgrammingLanguageName::Powerbuilder`.
//!
//! Imported from leachim6/hello-world (`p/PowerBuilder.psr` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/PowerBuilder.psr>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Powerbuilder => "PowerBuilder",
    versions: [Powerbuilder => "PowerBuilder"],
    extensions: [Psr => "psr"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powerbuilder,
        extensions: &[FileExtension::Psr],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powerbuilder,
            syntax: concat!(r#"MessageBox(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"MessageBox(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
