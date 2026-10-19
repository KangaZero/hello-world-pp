//! Hello-world snippet for `ProgrammingLanguageName::Kitanai`.
//!
//! Imported from leachim6/hello-world (`k/Kitanai.ktn` at `a152253`),
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
    language: Kitanai => "Kitanai",
    versions: [Kitanai => "Kitanai"],
    extensions: [Ktn => "ktn"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kitanai,
        extensions: &[FileExtension::Ktn],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kitanai,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
