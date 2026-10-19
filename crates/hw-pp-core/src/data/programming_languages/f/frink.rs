//! Hello-world snippet for `ProgrammingLanguageName::Frink`.
//!
//! Imported from leachim6/hello-world (`f/Frink.frink` at `a152253`),
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
    language: Frink => "Frink",
    versions: [Frink => "Frink"],
    extensions: [Frink => "frink"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Frink,
        extensions: &[FileExtension::Frink],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Frink,
            syntax: concat!(r#"println[""#, output_for_hello_world!(), r#""]"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println[""#, output_for_hello_world!(), r#""]"#),
            )],
        }],
    };
