//! Hello-world snippet for `ProgrammingLanguageName::Acl`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#ACL>
//! - <https://esolangs.org/wiki/ACL>

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Acl => "ACL",
    versions: [Acl => "ACL"],
    extensions: [],
}

const ACL: &str = r"34344343444C3443443434343C34434344344C34434344344C34434344443C34344444C3434343434443C34434344443C34443443434C34434344344C34434434344C3434444343CC";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acl,
            syntax: ACL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACL,
                    [(
                        TokenKind::Plain,
                        r"34344343444C3443443434343C34434344344C34434344344C34434344443C34344444C3434343434443C34434344443C34443443434C34434344344C34434434344C3434444343CC",
                    )],
                )
            },
        }],
    };
