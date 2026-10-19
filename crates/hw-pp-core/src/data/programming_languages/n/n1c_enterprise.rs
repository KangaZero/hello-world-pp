//! Hello-world snippet for `ProgrammingLanguageName::N1cEnterprise`.
//!
//! Imported from leachim6/hello-world (`#/1C Enterprise` at `a152253`),
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
    language: N1cEnterprise => "1C Enterprise",
    versions: [N1cEnterprise => "1C Enterprise"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N1cEnterprise,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N1cEnterprise,
            syntax: concat!(r#"Message(""#, output_for_hello_world!(), r#"");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"Message(""#, output_for_hello_world!(), r#"");"#),
            )],
        }],
    };
