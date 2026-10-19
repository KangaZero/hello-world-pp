//! Hello-world snippet for `ProgrammingLanguageName::Perfume`.
//!
//! Imported from leachim6/hello-world (`p/Perfume.prfm` at `a152253`),
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
    language: Perfume => "Perfume",
    versions: [Perfume => "Perfume"],
    extensions: [Prfm => "prfm"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Perfume,
        extensions: &[FileExtension::Prfm],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Perfume,
            syntax: concat!(r#"println ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
