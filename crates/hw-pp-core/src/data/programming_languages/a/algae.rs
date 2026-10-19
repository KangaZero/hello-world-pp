//! Hello-world snippet for `ProgrammingLanguageName::Algae`.
//!
//! Imported from leachim6/hello-world (`a/Algae.algae` at `a152253`),
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
    language: Algae => "Algae",
    versions: [Algae => "Algae"],
    extensions: [Algae => "algae"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Algae,
        extensions: &[FileExtension::Algae],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Algae,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#),
            )],
        }],
    };
