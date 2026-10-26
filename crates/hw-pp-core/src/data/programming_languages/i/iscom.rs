//! Hello-world snippet for `ProgrammingLanguageName::Iscom`.
//!
//! Imported from leachim6/hello-world (`i/Iscom.iscom` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Iscom.iscom>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Iscom => "Iscom",
    versions: [Iscom => "Iscom"],
    extensions: [Iscom => "iscom"],
}

const ISCOM: &str = concat!(r#"""#, output_for_hello_world!(), r#"" @=10"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Iscom,
        extensions: &[FileExtension::Iscom],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Iscom,
            syntax: ISCOM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ISCOM,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"" @=10"#),
                    )],
                )
            },
        }],
    };
