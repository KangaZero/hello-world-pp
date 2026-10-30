//! Hello-world snippet for `ProgrammingLanguageName::Ecstatic`.
//!
//! Imported from the Hello World Collection (`files/Ecstatic.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Ecstatic.htm>
//! - <http://helloworldcollection.de/#Ecstatic>
//! - <https://esolangs.org/wiki/Ecstatic> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Ecstatic => "Ecstatic",
    versions: [Ecstatic => "Ecstatic"],
    extensions: [],
}

const ECSTATIC: &str = r"!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ecstatic,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ecstatic,
            syntax: ECSTATIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ECSTATIC,
                    [(
                        TokenKind::Plain,
                        r"!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!",
                    )],
                )
            },
        }],
    };
