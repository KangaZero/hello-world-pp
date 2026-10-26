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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: FromHereToThere => "FROM HERE TO THERE",
    versions: [FromHereToThere => "FROM HERE TO THERE"],
    extensions: [],
}

const FROM_HERE_TO_THERE: &str = concat!(r#"FROM ""#, output_for_hello_world!(), r#"" TO OUT"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FromHereToThere,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FromHereToThere,
            syntax: FROM_HERE_TO_THERE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FROM_HERE_TO_THERE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"FROM ""#, output_for_hello_world!(), r#"" TO OUT"#),
                    )],
                )
            },
        }],
    };
