//! Hello-world snippet for `ProgrammingLanguageName::RickRoll`.
//!
//! Imported from leachim6/hello-world (`r/Rick Roll.rickroll` at `a152253`),
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
    language: RickRoll => "Rick Roll",
    versions: [RickRoll => "Rick Roll"],
    extensions: [Rickroll => "rickroll"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RickRoll,
        extensions: &[FileExtension::Rickroll],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RickRoll,
            syntax: concat!(
                r#"take me to ur heart
    give msg up ""#,
                output_for_hello_world!(),
                r#""
    i just wanna tell u how im feeling msg
say goodbye"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"take me to ur heart
    give msg up ""#,
                    output_for_hello_world!(),
                    r#""
    i just wanna tell u how im feeling msg
say goodbye"#
                ),
            )],
        }],
    };
