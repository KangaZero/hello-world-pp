//! Hello-world snippet for `ProgrammingLanguageName::Rubic`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rubic>
//! - <https://esolangs.org/wiki/Rubic>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Rubic => "Rubic",
    versions: [Rubic => "Rubic"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rubic,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rubic,
            syntax: r".ww ( +ww ) ! +ww +ww +ww +ww r -wg ! +oo b -og ! ! +wo' ! b' .wg *oo +oo ! *wg -oo b -og ! b' +oo +oo +oo ! +wg ! l' -wg ! -oo ! .wg *wg r -wg ! .oo +ww !",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r".ww ( +ww ) ! +ww +ww +ww +ww r -wg ! +oo b -og ! ! +wo' ! b' .wg *oo +oo ! *wg -oo b -og ! b' +oo +oo +oo ! +wg ! l' -wg ! -oo ! .wg *wg r -wg ! .oo +ww !",
            )],
        }],
    };
