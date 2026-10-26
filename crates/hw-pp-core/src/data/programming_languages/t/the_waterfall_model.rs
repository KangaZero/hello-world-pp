//! Hello-world snippet for `ProgrammingLanguageName::TheWaterfallModel`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#The_Waterfall_Model>
//! - <https://esolangs.org/wiki/The_Waterfall_Model>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TheWaterfallModel => "The Waterfall Model",
    versions: [TheWaterfallModel => "The Waterfall Model"],
    extensions: [],
}

const THE_WATERFALL_MODEL: &str = r"[[27425257720895005854917070679642,
      17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17],
 [  2, 3, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
 [  3, 1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 4, 2, 2],
 [  3, 2, 2, 3, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 0, 2, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 4, 0, 2, 2],
 [  3, 1, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2],
 [  3, 2, 2, 2, 2, 2, 3, 1, 2, 2, 0, 2, 0, 4, 2, 0, 2, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 4, 7],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 2, 2, 2, 2, 2],
 [ 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
 [27425257720895005854917070679641,
       1, 2, 1, 1, 1, 0, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1],
 [  3, 1, 1, 0, 1, 1, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1, 1, 9],
 [  3, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1],
 [239, 2, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1],
 [  5, 1, 1, 1, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 3, 1, 1],
 [  3, 1, 1, 1, 1, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 3, 1],
 [  3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]]";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TheWaterfallModel,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TheWaterfallModel,
            syntax: THE_WATERFALL_MODEL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    THE_WATERFALL_MODEL,
                    [(
                        TokenKind::Plain,
                        r"[[27425257720895005854917070679642,
      17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17,17],
 [  2, 3, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2, 2, 2, 2],
 [  3, 1, 3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 4, 2, 2],
 [  3, 2, 2, 3, 1, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 0, 2, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 4, 0, 2, 2],
 [  3, 1, 2, 2, 2, 3, 2, 2, 2, 2, 2, 2, 2, 2, 0, 2, 2, 2],
 [  3, 2, 2, 2, 2, 2, 3, 1, 2, 2, 0, 2, 0, 4, 2, 0, 2, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 0, 4, 7],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 2],
 [  3, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 0, 2, 2, 2, 2, 2],
 [ 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
 [27425257720895005854917070679641,
       1, 2, 1, 1, 1, 0, 1, 1, 1, 1, 3, 1, 1, 1, 1, 1, 1],
 [  3, 1, 1, 0, 1, 1, 1, 1, 1, 2, 1, 1, 3, 1, 1, 1, 1, 9],
 [  3, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1],
 [239, 2, 1, 0, 1, 1, 1, 1, 1, 1, 1, 1, 3, 1, 3, 1, 1, 1],
 [  5, 1, 1, 1, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 3, 1, 1],
 [  3, 1, 1, 1, 1, 1, 1, 1, 2, 0, 1, 1, 1, 1, 1, 1, 3, 1],
 [  3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 3]]",
                    )],
                )
            },
        }],
    };
