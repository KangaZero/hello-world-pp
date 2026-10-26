//! Hello-world snippet for `ProgrammingLanguageName::Modulous`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Modulous>
//! - <https://esolangs.org/wiki/Modulous>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Modulous => "Modulous",
    versions: [Modulous => "Modulous"],
    extensions: [],
}

const MODULOUS: &str = concat!(
    r"[PSH STR “",
    output_for_hello_world!(),
    r"”][PRT STR][JMP B 1 IF NOT 0]"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Modulous,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Modulous,
            syntax: MODULOUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MODULOUS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"[PSH STR “",
                            output_for_hello_world!(),
                            r"”][PRT STR][JMP B 1 IF NOT 0]"
                        ),
                    )],
                )
            },
        }],
    };
