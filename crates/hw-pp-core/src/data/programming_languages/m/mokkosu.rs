//! Hello-world snippet for `ProgrammingLanguageName::Mokkosu`.
//!
//! Imported from leachim6/hello-world (`m/Mokkosu.mok` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mokkosu.mok>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mokkosu => "Mokkosu",
    versions: [Mokkosu => "Mokkosu"],
    extensions: [Mok => "mok"],
}

const MOKKOSU: &str = concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mokkosu,
        extensions: &[FileExtension::Mok],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mokkosu,
            syntax: MOKKOSU,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOKKOSU,
                    [(
                        TokenKind::Plain,
                        concat!(r#"do println ""#, output_for_hello_world!(), r#"";"#),
                    )],
                )
            },
        }],
    };
