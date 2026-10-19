//! Hello-world snippet for `ProgrammingLanguageName::Retro`.
//!
//! Imported from leachim6/hello-world (`r/Retro.retro` at `a152253`),
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
    language: Retro => "Retro",
    versions: [Retro => "Retro"],
    extensions: [Retro => "retro"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Retro,
        extensions: &[FileExtension::Retro],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Retro,
            syntax: concat!(r#"""#, output_for_hello_world!(), r#"" puts"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"""#, output_for_hello_world!(), r#"" puts"#),
            )],
        }],
    };
