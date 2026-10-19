//! Hello-world snippet for `ProgrammingLanguageName::And`.
//!
//! Imported from leachim6/hello-world (`a/And` at `a152253`),
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
    language: And => "And",
    versions: [And => "And"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::And,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::And,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"" && exit"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"" && exit"#),
            )],
        }],
    };
