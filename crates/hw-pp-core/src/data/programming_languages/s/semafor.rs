//! Hello-world snippet for `ProgrammingLanguageName::Semafor`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Semafor>
//! - <https://esolangs.org/wiki/Semafor>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Semafor => "Semafor",
    versions: [Semafor => "Semafor"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Semafor,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Semafor,
            syntax: r"++++%!!!%7%+%!%8+++%!!!%7%+%!%8+++++%
!!!%7%+%!%8+++++%!!!%7%+%!%8++++++%!!
!%7%+%!%8+%!!!%7%+%!%8++++++++%!!!%7%
+%!%8++++++%!!!%7%+%!%8+++++++%!!!%7%
+%!%8+++++%!!!%7%+%!%8++%!!!%7%+%!%8%",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"++++%!!!%7%+%!%8+++%!!!%7%+%!%8+++++%
!!!%7%+%!%8+++++%!!!%7%+%!%8++++++%!!
!%7%+%!%8+%!!!%7%+%!%8++++++++%!!!%7%
+%!%8++++++%!!!%7%+%!%8+++++++%!!!%7%
+%!%8+++++%!!!%7%+%!%8++%!!!%7%+%!%8%",
            )],
        }],
    };
