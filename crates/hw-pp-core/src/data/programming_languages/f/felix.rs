//! Hello-world snippet for `ProgrammingLanguageName::Felix`.
//!
//! Imported from leachim6/hello-world (`f/Felix.flx` at `a152253`),
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
    language: Felix => "Felix",
    versions: [Felix => "Felix"],
    extensions: [Flx => "flx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Felix,
        extensions: &[FileExtension::Flx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Felix,
            syntax: concat!(r#"println$ ""#, output_for_hello_world!(), r#"";"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println$ ""#, output_for_hello_world!(), r#"";"#),
            )],
        }],
    };
