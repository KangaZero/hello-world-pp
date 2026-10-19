//! Hello-world snippet for `ProgrammingLanguageName::Acl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#ACL>
//! - <https://esolangs.org/wiki/ACL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Acl => "ACL",
    versions: [Acl => "ACL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acl,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acl,
            syntax: r"34344343444C3443443434343C34434344344C34434344344C34434344443C34344444C3434343434443C34434344443C34443443434C34434344344C34434434344C3434444343CC",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"34344343444C3443443434343C34434344344C34434344344C34434344443C34344444C3434343434443C34434344443C34443443434C34434344344C34434434344C3434444343CC",
            )],
        }],
    };
