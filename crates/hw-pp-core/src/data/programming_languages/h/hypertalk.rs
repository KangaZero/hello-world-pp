//! Hello-world snippet for `ProgrammingLanguageName::Hypertalk`.
//!
//! Imported from leachim6/hello-world (`h/HyperTalk.ht` at `a152253`),
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
    language: Hypertalk => "HyperTalk",
    versions: [Hypertalk => "HyperTalk"],
    extensions: [Ht => "ht"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hypertalk,
        extensions: &[FileExtension::Ht],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hypertalk,
            syntax: concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"put ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
