//! Hello-world snippet for `ProgrammingLanguageName::Alore`.
//!
//! Imported from leachim6/hello-world (`a/Alore.alore` at `a152253`),
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
    language: Alore => "Alore",
    versions: [Alore => "Alore"],
    extensions: [Alore => "alore"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alore,
        extensions: &[FileExtension::Alore],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alore,
            syntax: concat!(r"Print('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"Print('", output_for_hello_world!(), r"')"),
            )],
        }],
    };
