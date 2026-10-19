//! Hello-world snippet for `ProgrammingLanguageName::FromHereToThere`.
//!
//! Imported from leachim6/hello-world (`f/FROM HERE TO THERE` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/FROM%20HERE%20TO%20THERE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: FromHereToThere => "FROM HERE TO THERE",
    versions: [FromHereToThere => "FROM HERE TO THERE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FromHereToThere,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FromHereToThere,
            syntax: concat!(r#"FROM ""#, output_for_hello_world!(), r#"" TO OUT"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"FROM ""#, output_for_hello_world!(), r#"" TO OUT"#),
            )],
        }],
    };
