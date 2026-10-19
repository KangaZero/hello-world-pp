//! Hello-world snippet for `ProgrammingLanguageName::ApplesoftBasic`.
//!
//! Imported from leachim6/hello-world (`a/Applesoft BASIC` at `a152253`),
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
    language: ApplesoftBasic => "Applesoft BASIC",
    versions: [ApplesoftBasic => "Applesoft BASIC"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ApplesoftBasic,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ApplesoftBasic,
            syntax: concat!(r#"10 PRINT ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"10 PRINT ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
