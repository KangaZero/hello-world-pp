//! Hello-world snippet for `ProgrammingLanguageName::Potigol`.
//!
//! Imported from leachim6/hello-world (`p/Potigol.poti` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Potigol.poti>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Potigol => "Potigol",
    versions: [Potigol => "Potigol"],
    extensions: [Poti => "poti"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Potigol,
        extensions: &[FileExtension::Poti],
        categories: &[Educational],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Potigol,
            syntax: concat!(r#"escreva ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"escreva ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
