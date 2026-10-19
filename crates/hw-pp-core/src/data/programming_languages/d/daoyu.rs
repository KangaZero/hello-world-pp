//! Hello-world snippet for `ProgrammingLanguageName::Daoyu`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Daoyu>
//! - <https://esolangs.org/wiki/Daoyu>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Daoyu => "Daoyu",
    versions: [Daoyu => "Daoyu"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Daoyu,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Daoyu,
            syntax: r"$$$
(([]!)/([])):
((/[])/([]!/[]!)):
(/[])::
[/([]!/[])]!:
[[[]]!]:
[([]!)/[/[]!]!]:
[/([]!/[])]!:
[([]!)/(/[])]:
((/[])/[]):
(/([]!)):
([[]]!/[[]!]!):
[[[]/[]]]!:",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"$$$
(([]!)/([])):
((/[])/([]!/[]!)):
(/[])::
[/([]!/[])]!:
[[[]]!]:
[([]!)/[/[]!]!]:
[/([]!/[])]!:
[([]!)/(/[])]:
((/[])/[]):
(/([]!)):
([[]]!/[[]!]!):
[[[]/[]]]!:",
            )],
        }],
    };
