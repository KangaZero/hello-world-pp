//! Hello-world snippet for `ProgrammingLanguageName::Something`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Something>
//! - <https://esolangs.org/wiki/Something>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Something => "Something",
    versions: [Something => "Something"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Something,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Something,
            syntax: r"ADD 72
CHR 
ZER
ADD 69
CHR 
ZER
ADD 76
CHR 
ZER
ADD 76
CHR 
ZER
ADD 79
CHR 
ZER
ADD 32
CHR 
ZER
ADD 87
CHR 
ZER
ADD 79
CHR 
ZER
ADD 82
CHR 
ZER
ADD 76
CHR 
ZER
ADD 68
CHR 
ZER",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"ADD 72
CHR 
ZER
ADD 69
CHR 
ZER
ADD 76
CHR 
ZER
ADD 76
CHR 
ZER
ADD 79
CHR 
ZER
ADD 32
CHR 
ZER
ADD 87
CHR 
ZER
ADD 79
CHR 
ZER
ADD 82
CHR 
ZER
ADD 76
CHR 
ZER
ADD 68
CHR 
ZER",
            )],
        }],
    };
