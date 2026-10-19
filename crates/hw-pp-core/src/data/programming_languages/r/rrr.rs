//! Hello-world snippet for `ProgrammingLanguageName::Rrr`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Rrr>
//! - <https://esolangs.org/wiki/Rrr>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Rrr => "Rrr",
    versions: [Rrr => "Rrr"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rrr,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rrr,
            syntax: r"rRrRrrRrrRrrRrrRrRrrrRrrRrrrRrrr
RrRrrrrRrrRrrrrRrrrrRrRrrrrRrrRr
rrrrRrrrrrRrRrrrrrRrrRrrrrrrRrrr
rrrRrRrRrrRrrrrrrrRrrrrrrrRrRrrr
rrrRrrRrrrrrrrrRrrrrrrrrRrRrrrrr
RrrRrrrrrrrrrRrrrrrrrrrRrRrrrrrr
rRrrRrrrrrrrrrrRrrrrrrrrrrRrRrrr
rRrrRrrrrrrrrrrrRrrrrrrrrrrrRrRr
rrrrrrrRrrRrrrrrrrrrrrr",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"rRrRrrRrrRrrRrrRrRrrrRrrRrrrRrrr
RrRrrrrRrrRrrrrRrrrrRrRrrrrRrrRr
rrrrRrrrrrRrRrrrrrRrrRrrrrrrRrrr
rrrRrRrRrrRrrrrrrrRrrrrrrrRrRrrr
rrrRrrRrrrrrrrrRrrrrrrrrRrRrrrrr
RrrRrrrrrrrrrRrrrrrrrrrRrRrrrrrr
rRrrRrrrrrrrrrrRrrrrrrrrrrRrRrrr
rRrrRrrrrrrrrrrrRrrrrrrrrrrrRrRr
rrrrrrrRrrRrrrrrrrrrrrr",
            )],
        }],
    };
