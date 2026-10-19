//! Hello-world snippet for `ProgrammingLanguageName::Hy`.
//!
//! Imported from leachim6/hello-world (`h/Hy.hy` at `a152253`),
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
    language: Hy => "Hy",
    versions: [Hy => "Hy"],
    extensions: [Hy => "hy"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hy,
        extensions: &[FileExtension::Hy],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hy,
            syntax: concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"(print ""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
