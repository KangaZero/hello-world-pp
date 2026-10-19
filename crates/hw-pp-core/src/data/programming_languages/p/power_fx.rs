//! Hello-world snippet for `ProgrammingLanguageName::PowerFx`.
//!
//! Imported from leachim6/hello-world (`p/Power Fx.pfx` at `a152253`),
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
    language: PowerFx => "Power Fx",
    versions: [PowerFx => "Power Fx"],
    extensions: [Pfx => "pfx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PowerFx,
        extensions: &[FileExtension::Pfx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PowerFx,
            syntax: concat!(r#"Text(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Text(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
