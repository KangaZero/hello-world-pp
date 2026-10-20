//! Hello-world snippet for `ProgrammingLanguageName::Mokkosu`.
//!
//! Imported from leachim6/hello-world (`m/Mokkosu.mok` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mokkosu.mok>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mokkosu => "Mokkosu",
    versions: [Mokkosu => "Mokkosu"],
    extensions: [Mok => "mok"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mokkosu,
        extensions: &[FileExtension::Mok],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mokkosu,
            syntax: concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
