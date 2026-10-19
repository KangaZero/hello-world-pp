//! Hello-world snippet for `ProgrammingLanguageName::KotlinScript`.
//!
//! Imported from leachim6/hello-world (`k/Kotlin Script.kts` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: KotlinScript => "Kotlin Script",
    versions: [KotlinScript => "Kotlin Script"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::KotlinScript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::KotlinScript,
            syntax: concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"println(""#, output_for_hello_world!(), r#"")"#),
            )],
        }],
    };
