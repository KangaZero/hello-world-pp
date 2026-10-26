//! Hello-world snippet for `ProgrammingLanguageName::Anguish`.
//!
//! Imported from the Hello World Collection (`files/Anguish.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Anguish.htm>
//! - <http://helloworldcollection.de/#Anguish>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Anguish => "Anguish",
    versions: [Anguish => "Anguish"],
    extensions: [],
}

const ANGUISH: &str = "\u{200c}\u{feff}\u{2063}\u{200c}\u{2063}\u{200d}\u{feff}\u{2063}\u{2063}\u{feff}\u{feff}\u{feff}\u{2061}\u{feff}\u{2062}\u{feff}\u{200b}\u{2060}\u{feff}\u{200c}\u{200d}\u{2063}\u{2063}\u{200d}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{200c}\u{2060}\u{2061}\u{2061}\u{2061}\u{2061}\u{200c}\u{2060}\u{2061}\u{2061}\u{2060}\u{2061}\u{2061}\u{2061}\u{2060}\u{2061}\u{2061}\u{2061}\u{2060}\u{2061}\u{200b}\u{200b}\u{200b}\u{200b}\u{2062}\u{200d}\u{2060}\u{2061}\u{2060}\u{2061}\u{2060}\u{2062}\u{2060}\u{2060}\u{2061}\u{200c}\u{200b}\u{200d}\u{200b}\u{2062}\u{200d}\u{2060}\u{2060}\u{2063}\u{2060}\u{2062}\u{2062}\u{2062}\u{2063}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2063}\u{2063}\u{2061}\u{2061}\u{2061}\u{2063}\u{2060}\u{2060}\u{2063}\u{200b}\u{2062}\u{2063}\u{200b}\u{2063}\u{2061}\u{2061}\u{2061}\u{2063}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2063}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2063}\u{2060}\u{2060}\u{2061}\u{2063}\u{2060}\u{2061}\u{2061}\u{2063}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Anguish,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Anguish,
            syntax: ANGUISH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ANGUISH,
                    [(
                        TokenKind::Plain,
                        "\u{200c}\u{feff}\u{2063}\u{200c}\u{2063}\u{200d}\u{feff}\u{2063}\u{2063}\u{feff}\u{feff}\u{feff}\u{2061}\u{feff}\u{2062}\u{feff}\u{200b}\u{2060}\u{feff}\u{200c}\u{200d}\u{2063}\u{2063}\u{200d}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{200c}\u{2060}\u{2061}\u{2061}\u{2061}\u{2061}\u{200c}\u{2060}\u{2061}\u{2061}\u{2060}\u{2061}\u{2061}\u{2061}\u{2060}\u{2061}\u{2061}\u{2061}\u{2060}\u{2061}\u{200b}\u{200b}\u{200b}\u{200b}\u{2062}\u{200d}\u{2060}\u{2061}\u{2060}\u{2061}\u{2060}\u{2062}\u{2060}\u{2060}\u{2061}\u{200c}\u{200b}\u{200d}\u{200b}\u{2062}\u{200d}\u{2060}\u{2060}\u{2063}\u{2060}\u{2062}\u{2062}\u{2062}\u{2063}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2061}\u{2063}\u{2063}\u{2061}\u{2061}\u{2061}\u{2063}\u{2060}\u{2060}\u{2063}\u{200b}\u{2062}\u{2063}\u{200b}\u{2063}\u{2061}\u{2061}\u{2061}\u{2063}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2063}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2062}\u{2063}\u{2060}\u{2060}\u{2061}\u{2063}\u{2060}\u{2061}\u{2061}\u{2063}",
                    )],
                )
            },
        }],
    };
