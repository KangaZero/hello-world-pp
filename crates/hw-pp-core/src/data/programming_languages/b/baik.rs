//! Hello-world snippet for `ProgrammingLanguageName::Baik`.
//!
//! Imported from leachim6/hello-world (`b/BAIK.ina` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BAIK.ina>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Baik => "BAIK",
    versions: [Baik => "BAIK"],
    extensions: [Ina => "ina"],
}

const BAIK: &str = concat!(r#"tulis ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Baik,
        extensions: &[FileExtension::Ina],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Baik,
            syntax: BAIK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BAIK,
                    [(
                        TokenKind::Plain,
                        concat!(r#"tulis ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
