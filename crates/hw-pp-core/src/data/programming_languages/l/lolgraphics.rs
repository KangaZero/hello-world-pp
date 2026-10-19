//! Hello-world snippet for `ProgrammingLanguageName::Lolgraphics`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#LOLGraphics>
//! - <https://esolangs.org/wiki/LOLGraphics>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Lolgraphics => "LOLGraphics",
    versions: [Lolgraphics => "LOLGraphics"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolgraphics,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lolgraphics,
            syntax: r"HAI 0.1
IM IN UR CODE EXECUTIN UR KOMANDZ
PLZ PRINT TEXT HAI WORLD!
IM OUTTA UR CODE",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"HAI 0.1
IM IN UR CODE EXECUTIN UR KOMANDZ
PLZ PRINT TEXT HAI WORLD!
IM OUTTA UR CODE",
            )],
        }],
    };
