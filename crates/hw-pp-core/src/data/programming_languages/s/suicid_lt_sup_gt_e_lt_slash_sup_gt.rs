//! Hello-world snippet for `ProgrammingLanguageName::SuicidLtSupGtELtSlashSupGt`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#suicid%3Csup%3Ee%3C%2Fsup%3E>
//! - <https://esolangs.org/wiki/Suicide>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SuicidLtSupGtELtSlashSupGt => "suicid<sup>e</sup>",
    versions: [SuicidLtSupGtELtSlashSupGt => "suicid<sup>e</sup>"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SuicidLtSupGtELtSlashSupGt,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SuicidLtSupGtELtSlashSupGt,
            syntax: concat!(r#"<*vAA:""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"<*vAA:""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
