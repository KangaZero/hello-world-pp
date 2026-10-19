//! Hello-world snippet for `ProgrammingLanguageName::Sickbay`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SICKBAY>
//! - <https://esolangs.org/wiki/SICKBAY>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Sickbay => "SICKBAY",
    versions: [Sickbay => "SICKBAY"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sickbay,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sickbay,
            syntax: r#"20 PRINT "WORLD!"
10 PRINT "HELLO, ";
20 PRINT "SAILOR""#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"20 PRINT "WORLD!"
10 PRINT "HELLO, ";
20 PRINT "SAILOR""#,
            )],
        }],
    };
