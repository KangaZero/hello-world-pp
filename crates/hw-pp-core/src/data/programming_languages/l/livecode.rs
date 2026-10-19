//! Hello-world snippet for `ProgrammingLanguageName::Livecode`.
//!
//! Imported from leachim6/hello-world (`l/LiveCode` at `a152253`),
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
    language: Livecode => "LiveCode",
    versions: [Livecode => "LiveCode"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Livecode,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Livecode,
            syntax: concat!(r#"answer ""#, output_for_hello_world!(), r#"" "#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"answer ""#, output_for_hello_world!(), r#"" "#),
            )],
        }],
    };
