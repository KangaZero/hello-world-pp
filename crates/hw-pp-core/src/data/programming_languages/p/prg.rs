//! Hello-world snippet for `ProgrammingLanguageName::Prg`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#PRG>
//! - <https://esolangs.org/wiki/PRG>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Prg => "PRG",
    versions: [Prg => "PRG"],
    extensions: [],
}

const PRG: &str = r"VAR CHR CHL
VAR CHR CHO
SET CHL SUB POW TEN TWO TEN
SET CHO SUB POW ADD TEN ONE TWO TEN
PUT ARR 
    MUL ADD TEN TWO SIX
    ADD POW TEN TWO ONE
    CHL
    CHL
    CHO
    ADD MUL ADD SIX ONE SIX TWO
    POW TWO SUB SIX ONE
    ADD ADD CHL TEN ONE
    CHO
    SUB MUL ADD TEN TWO TEN SIX
    CHL
    POW TEN TWO
    ADD POW TWO SUB SIX ONE ONE
END";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Prg,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Prg,
            syntax: PRG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PRG,
                    [(
                        TokenKind::Plain,
                        r"VAR CHR CHL
VAR CHR CHO
SET CHL SUB POW TEN TWO TEN
SET CHO SUB POW ADD TEN ONE TWO TEN
PUT ARR 
    MUL ADD TEN TWO SIX
    ADD POW TEN TWO ONE
    CHL
    CHL
    CHO
    ADD MUL ADD SIX ONE SIX TWO
    POW TWO SUB SIX ONE
    ADD ADD CHL TEN ONE
    CHO
    SUB MUL ADD TEN TWO TEN SIX
    CHL
    POW TEN TWO
    ADD POW TWO SUB SIX ONE ONE
END",
                    )],
                )
            },
        }],
    };
