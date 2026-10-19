//! Hello-world snippet for `ProgrammingLanguageName::Nasal`.
//!
//! Imported from leachim6/hello-world (PR #1615 `n/Nasal.nas`),
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
    language: Nasal => "Nasal",
    versions: [Nasal => "Nasal"],
    extensions: [Nas => "nas"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nasal,
        extensions: &[FileExtension::Nas],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nasal,
            syntax: concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
