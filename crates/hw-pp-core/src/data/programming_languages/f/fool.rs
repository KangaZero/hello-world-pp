//! Hello-world snippet for `ProgrammingLanguageName::Fool`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fool>
//! - <https://esolangs.org/wiki/Fool>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Fool => "Fool",
    versions: [Fool => "Fool"],
    extensions: [],
}

const FOOL: &str = r"H:>.>.>.>.*.>.>.>.*.>
e:>.*.>.>.*.>.>.>.*.>.*.>
l:>.>.>.*.>.*.>.>.*.>.*.>
o:>.*.>.*.>.*.>.*.>.>.*.>.*.>
,:>.>.>.*.>.*.>.>.*.>.>
 :>.>.>.>.>.>.*.>.>
w:>.*.>.*.>.*.>.>.*.>.*.>.*.>
r:>.>.*.>.>.>.*.>.*.>.*.>
d:>.>.>.*.>.>.>.*.>.*.>
!:>.*.>.>.>.>.>.*.>.>
main:!.d.l.r.o.w. .,.o.l.l.e.H";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fool,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fool,
            syntax: FOOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOOL,
                    [(
                        TokenKind::Plain,
                        r"H:>.>.>.>.*.>.>.>.*.>
e:>.*.>.>.*.>.>.>.*.>.*.>
l:>.>.>.*.>.*.>.>.*.>.*.>
o:>.*.>.*.>.*.>.*.>.>.*.>.*.>
,:>.>.>.*.>.*.>.>.*.>.>
 :>.>.>.>.>.>.*.>.>
w:>.*.>.*.>.*.>.>.*.>.*.>.*.>
r:>.>.*.>.>.>.*.>.*.>.*.>
d:>.>.>.*.>.>.>.*.>.*.>
!:>.*.>.>.>.>.>.*.>.>
main:!.d.l.r.o.w. .,.o.l.l.e.H",
                    )],
                )
            },
        }],
    };
