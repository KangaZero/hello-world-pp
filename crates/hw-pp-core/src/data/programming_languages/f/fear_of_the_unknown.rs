//! Hello-world snippet for `ProgrammingLanguageName::FearOfTheUnknown`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Fear_of_the_Unknown>
//! - <https://esolangs.org/wiki/Fear_of_the_Unknown>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: FearOfTheUnknown => "Fear of the Unknown",
    versions: [FearOfTheUnknown => "Fear of the Unknown"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FearOfTheUnknown,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FearOfTheUnknown,
            syntax: r"$IO=$IO; $IO+72; $IO=$IO; $IO+101; $IO=$IO; $IO+108;
$IO=$IO; $IO+108; $IO=$IO; $IO+111; $IO=$IO; $IO+44;
$IO=$IO; $IO+32; $IO=$IO; $IO+87; $IO=$IO; $IO+111;
$IO=$IO; $IO+114; $IO=$IO; $IO+108; $IO=$IO; $IO+100;
$IO=$IO; $IO+33; HALT-99;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"$IO=$IO; $IO+72; $IO=$IO; $IO+101; $IO=$IO; $IO+108;
$IO=$IO; $IO+108; $IO=$IO; $IO+111; $IO=$IO; $IO+44;
$IO=$IO; $IO+32; $IO=$IO; $IO+87; $IO=$IO; $IO+111;
$IO=$IO; $IO+114; $IO=$IO; $IO+108; $IO=$IO; $IO+100;
$IO=$IO; $IO+33; HALT-99;",
            )],
        }],
    };
