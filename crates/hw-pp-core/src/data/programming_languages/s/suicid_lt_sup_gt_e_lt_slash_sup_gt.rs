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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SuicidLtSupGtELtSlashSupGt => "suicid<sup>e</sup>",
    versions: [SuicidLtSupGtELtSlashSupGt => "suicid<sup>e</sup>"],
    extensions: [],
}

const SUICID_LT_SUP_GT_E_LT_SLASH_SUP_GT: &str =
    concat!(r#"<*vAA:""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SuicidLtSupGtELtSlashSupGt,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SuicidLtSupGtELtSlashSupGt,
            syntax: SUICID_LT_SUP_GT_E_LT_SLASH_SUP_GT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SUICID_LT_SUP_GT_E_LT_SLASH_SUP_GT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"<*vAA:""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
