//! Hello-world snippet for `ProgrammingLanguageName::Gammaplex`.
//!
//! Imported from leachim6/hello-world (`g/Gammaplex.gammaplex` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Gammaplex.gammaplex>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gammaplex => "Gammaplex",
    versions: [Gammaplex => "Gammaplex"],
    extensions: [Gammaplex => "gammaplex"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gammaplex,
        extensions: &[FileExtension::Gammaplex],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gammaplex,
            syntax: concat!(r#"X""#, output_for_hello_world!(), r#""XXSXrRE"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"X""#, output_for_hello_world!(), r#""XXSXrRE"#),
            )],
        }],
    };
