//! Hello-world snippet for `ProgrammingLanguageName::Lil`.
//!
//! Imported from leachim6/hello-world (PR #1602 `l/Lil.lil`),
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
    language: Lil => "Lil",
    versions: [Lil => "Lil"],
    extensions: [Lil => "lil"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lil,
        extensions: &[FileExtension::Lil],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lil,
            syntax: concat!(r#"print[""#, output_for_hello_world!(), r#""]"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print[""#, output_for_hello_world!(), r#""]"#),
            )],
        }],
    };
