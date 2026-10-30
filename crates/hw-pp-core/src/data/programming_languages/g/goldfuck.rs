//! Hello-world snippet for `ProgrammingLanguageName::Goldfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Goldfuck>
//! - <https://esolangs.org/wiki/Goldfuck>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Goldfuck => "Goldfuck",
    versions: [Goldfuck => "Goldfuck"],
    extensions: [],
}

const GOLDFUCK: &str = r"1.6180339787498948482645868343756381177203391798027628625354486
227057604628179024497072052041893961374847560880753838917501266
338622233369317934800607667269544333090865989395820056383226613
599282902678876752087668922017115962076322212432162691486262963
136244381197587012303408058870544547498461856959648644491410443
207723449440495658467185098743399422126448770664760915888607499
987124037652177575179785341662562424075893697040042812104276517
711077780431531314101970466059914669792731762356006708758071018
179523489427521942435305478300228985699782971";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Goldfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Goldfuck,
            syntax: GOLDFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOLDFUCK,
                    [(
                        TokenKind::Plain,
                        r"1.6180339787498948482645868343756381177203391798027628625354486
227057604628179024497072052041893961374847560880753838917501266
338622233369317934800607667269544333090865989395820056383226613
599282902678876752087668922017115962076322212432162691486262963
136244381197587012303408058870544547498461856959648644491410443
207723449440495658467185098743399422126448770664760915888607499
987124037652177575179785341662562424075893697040042812104276517
711077780431531314101970466059914669792731762356006708758071018
179523489427521942435305478300228985699782971",
                    )],
                )
            },
        }],
    };
