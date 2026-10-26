//! Hello-world snippet for `ProgrammingLanguageName::Kiml`.
//!
//! Imported from leachim6/hello-world (`k/KimL.kiml` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/KimL.kiml>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Kiml => "KimL",
    versions: [Kiml => "KimL"],
    extensions: [Kiml => "kiml"],
}

const KIML: &str = concat!(r#"io.out ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kiml,
        extensions: &[FileExtension::Kiml],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kiml,
            syntax: KIML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KIML,
                    [(
                        TokenKind::Plain,
                        concat!(r#"io.out ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
