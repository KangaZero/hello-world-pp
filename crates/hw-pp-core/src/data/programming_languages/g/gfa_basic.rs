//! Hello-world snippet for `ProgrammingLanguageName::GfaBasic`.
//!
//! Imported from leachim6/hello-world (`g/GFA Basic` at `a152253`),
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
    language: GfaBasic => "GFA Basic",
    versions: [GfaBasic => "GFA Basic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GfaBasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GfaBasic,
            syntax: concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"PRINT ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
