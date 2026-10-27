//! Hello-world snippet for `ProgrammingLanguageName::MushyBang3`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Mushy!_:3>
//! - <https://esolangs.org/wiki/Mushy!_:3>

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
    language: MushyBang3 => "Mushy! :3",
    versions: [MushyBang3 => "Mushy! :3"],
    extensions: [],
}

const MUSHY_BANG3: &str = r" tsspssspt
sssppwwwpqtt
 wwwpepwww
     p
     q
     q
     t
     w
     p
     q
     w
     w
     w
     w";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MushyBang3,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MushyBang3,
            syntax: MUSHY_BANG3,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MUSHY_BANG3,
                    [(
                        TokenKind::Plain,
                        r" tsspssspt
sssppwwwpqtt
 wwwpepwww
     p
     q
     q
     t
     w
     p
     q
     w
     w
     w
     w",
                    )],
                )
            },
        }],
    };
