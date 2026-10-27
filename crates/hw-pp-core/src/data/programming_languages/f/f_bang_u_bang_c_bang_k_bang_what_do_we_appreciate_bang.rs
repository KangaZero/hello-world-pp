//! Hello-world snippet for `ProgrammingLanguageName::FBangUBangCBangKBangWhatDoWeAppreciateBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#F!_U!_C!_K!_What_do_we_appreciate!>
//! - <https://esolangs.org/wiki/F!_U!_C!_K!_What_do_we_appreciate!>
//! - <https://esolangs.org/wiki/F%21_U%21_C%21_K%21_What_do_we_appreciate%21> (paradigms)

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
    language: FBangUBangCBangKBangWhatDoWeAppreciateBang => "F! U! C! K! What do we appreciate!",
    versions: [FBangUBangCBangKBangWhatDoWeAppreciateBang => "F! U! C! K! What do we appreciate!"],
    extensions: [],
}

const F_BANG_U_BANG_C_BANG_K_BANG_WHAT_DO_WE_APPRECIATE_BANG: &str = r"F! F! F! F! F! F! F! F! Brain! What do we appreciate! F! F! F! F! Brain! 
What do we appreciate! F! F! What do we appreciate! F! F! F! What do we appreciate! 
F! F! F! What do we appreciate! F! Fuck! Fuck! Fuck! Fuck! U! Brainfuck! What do we appreciate! 
F! What do we appreciate! F! What do we appreciate! U! What do we appreciate! What do we appreciate!
 F! Brain! Fuck! Brainfuck! Fuck! U! Brainfuck! What do we appreciate! What do we appreciate! 
K! What do we appreciate! U! U! U! K! F! F! F! F! F! F! F! K! K! F! F! F! K! What do we appreciate! 
What do we appreciate! K! Fuck! U! K! Fuck! K! F! F! F! K! U! U! U! U! U! U! K! U! U! U! U! U! U! U! U! K! 
What do we appreciate! What do we appreciate! F! K! What do we appreciate! F! F! K!";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FBangUBangCBangKBangWhatDoWeAppreciateBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FBangUBangCBangKBangWhatDoWeAppreciateBang,
            syntax: F_BANG_U_BANG_C_BANG_K_BANG_WHAT_DO_WE_APPRECIATE_BANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    F_BANG_U_BANG_C_BANG_K_BANG_WHAT_DO_WE_APPRECIATE_BANG,
                    [(
                        TokenKind::Plain,
                        r"F! F! F! F! F! F! F! F! Brain! What do we appreciate! F! F! F! F! Brain! 
What do we appreciate! F! F! What do we appreciate! F! F! F! What do we appreciate! 
F! F! F! What do we appreciate! F! Fuck! Fuck! Fuck! Fuck! U! Brainfuck! What do we appreciate! 
F! What do we appreciate! F! What do we appreciate! U! What do we appreciate! What do we appreciate!
 F! Brain! Fuck! Brainfuck! Fuck! U! Brainfuck! What do we appreciate! What do we appreciate! 
K! What do we appreciate! U! U! U! K! F! F! F! F! F! F! F! K! K! F! F! F! K! What do we appreciate! 
What do we appreciate! K! Fuck! U! K! Fuck! K! F! F! F! K! U! U! U! U! U! U! K! U! U! U! U! U! U! U! U! K! 
What do we appreciate! What do we appreciate! F! K! What do we appreciate! F! F! K!",
                    )],
                )
            },
        }],
    };
