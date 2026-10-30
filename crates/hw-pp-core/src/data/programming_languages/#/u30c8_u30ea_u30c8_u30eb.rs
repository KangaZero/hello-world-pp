//! Hello-world snippet for `ProgrammingLanguageName::U30c8U30eaU30c8U30eb`.
//!
//! Imported from leachim6/hello-world (`#/ドリトル.dtl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/%E3%83%89%E3%83%AA%E3%83%88%E3%83%AB.dtl>
//! - <https://en.wikipedia.org/wiki/Dolittle_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: U30c8U30eaU30c8U30eb => "ドリトル",
    versions: [U30c8U30eaU30c8U30eb => "ドリトル"],
    extensions: [Dtl => "dtl"],
}

const U30C8_U30EA_U30C8_U30EB: &str =
    concat!(r#"ラベル！（""#, output_for_hello_world!(), r#""）作る。"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::U30c8U30eaU30c8U30eb,
        extensions: &[FileExtension::Dtl],
        categories: &[Educational],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::U30c8U30eaU30c8U30eb,
            syntax: U30C8_U30EA_U30C8_U30EB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    U30C8_U30EA_U30C8_U30EB,
                    [(
                        TokenKind::Plain,
                        concat!(r#"ラベル！（""#, output_for_hello_world!(), r#""）作る。"#),
                    )],
                )
            },
        }],
    };
