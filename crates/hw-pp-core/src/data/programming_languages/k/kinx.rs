//! Hello-world snippet for `ProgrammingLanguageName::Kinx`.
//!
//! Imported from leachim6/hello-world (`k/Kinx.kx` at `a152253`),
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
    language: Kinx => "Kinx",
    versions: [Kinx => "Kinx"],
    extensions: [Kx => "kx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kinx,
        extensions: &[FileExtension::Kx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kinx,
            syntax: concat!(r#"System.println(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"System.println(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
