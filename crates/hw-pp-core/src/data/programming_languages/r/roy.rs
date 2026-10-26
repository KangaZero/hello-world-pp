//! Hello-world snippet for `ProgrammingLanguageName::Roy`.
//!
//! Imported from leachim6/hello-world (`r/Roy.roy` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Roy.roy>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Roy => "Roy",
    versions: [Roy => "Roy"],
    extensions: [Roy => "roy"],
}

const ROY: &str = concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Roy,
        extensions: &[FileExtension::Roy],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Roy,
            syntax: ROY,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ROY,
                    [(
                        TokenKind::Plain,
                        concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
