//! Hello-world snippet for `ProgrammingLanguageName::Pluvo`.
//!
//! Imported from leachim6/hello-world (`p/Pluvo.pvo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pluvo.pvo>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Pluvo => "Pluvo",
    versions: [Pluvo => "Pluvo"],
    extensions: [Pvo => "pvo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pluvo,
        extensions: &[FileExtension::Pvo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pluvo,
            syntax: concat!(r#"say ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"say ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
