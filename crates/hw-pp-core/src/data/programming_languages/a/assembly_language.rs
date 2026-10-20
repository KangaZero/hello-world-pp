//! Hello-world snippet for `ProgrammingLanguageName::AssemblyLanguage`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Assembly_language>
//! - <https://esolangs.org/wiki/Assembly_language>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: AssemblyLanguage => "Assembly language",
    versions: [AssemblyLanguage => "Assembly language"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblyLanguage,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblyLanguage,
            syntax: r"DAT 01 48
MOV 00 01
DAT 01 65
MOV 00 01
DAT 01 6C
MOV 00 01
MOV 00 01
DAT 02 6F
MOV 00 02
DAT 01 20
MOV 00 01
DAT 01 77
MOV 00 01
MOV 00 02
DAT 01 72
MOV 00 01
DAT 01 6C
MOV 00 01
DAT 01 64
MOV 00 01",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"DAT 01 48
MOV 00 01
DAT 01 65
MOV 00 01
DAT 01 6C
MOV 00 01
MOV 00 01
DAT 02 6F
MOV 00 02
DAT 01 20
MOV 00 01
DAT 01 77
MOV 00 01
MOV 00 02
DAT 01 72
MOV 00 01
DAT 01 6C
MOV 00 01
DAT 01 64
MOV 00 01",
            )],
        }],
    };
