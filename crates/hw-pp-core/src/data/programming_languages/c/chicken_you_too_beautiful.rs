//! Hello-world snippet for `ProgrammingLanguageName::ChickenYouTooBeautiful`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Chicken_you_too_beautiful>
//! - <https://esolangs.org/wiki/Chicken_you_too_beautiful>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ChickenYouTooBeautiful => "Chicken you too beautiful",
    versions: [ChickenYouTooBeautiful => "Chicken you too beautiful"],
    extensions: [],
}

const CHICKEN_YOU_TOO_BEAUTIFUL: &str = r"chicken chicken chicken chicken chicken chicken chicken chicken rap beautiful chicken chicken chicken chicken rap beautiful chicken chicken beautiful chicken chicken chicken beautiful chicken chicken chicken beautiful chicken too too too too you basketball beautiful chicken beautiful chicken beautiful you beautiful beautiful chicken rap too basketball too you basketball beautiful beautiful dance beautiful you you you dance chicken chicken chicken chicken chicken chicken chicken dance dance chicken chicken chicken dance beautiful beautiful dance too you dance too dance chicken chicken chicken dance you you you you you you dance you you you you you you you you dance beautiful beautiful chicken dance beautiful chicken chicken dance";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ChickenYouTooBeautiful,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ChickenYouTooBeautiful,
            syntax: CHICKEN_YOU_TOO_BEAUTIFUL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHICKEN_YOU_TOO_BEAUTIFUL,
                    [(
                        TokenKind::Plain,
                        r"chicken chicken chicken chicken chicken chicken chicken chicken rap beautiful chicken chicken chicken chicken rap beautiful chicken chicken beautiful chicken chicken chicken beautiful chicken chicken chicken beautiful chicken too too too too you basketball beautiful chicken beautiful chicken beautiful you beautiful beautiful chicken rap too basketball too you basketball beautiful beautiful dance beautiful you you you dance chicken chicken chicken chicken chicken chicken chicken dance dance chicken chicken chicken dance beautiful beautiful dance too you dance too dance chicken chicken chicken dance you you you you you you dance you you you you you you you you dance beautiful beautiful chicken dance beautiful chicken chicken dance",
                    )],
                )
            },
        }],
    };
