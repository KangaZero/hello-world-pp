//! Hello-world snippet for `ProgrammingLanguageName::Brainflop`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#brainflop>
//! - <https://esolangs.org/wiki/Brainflop>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Brainflop => "brainflop",
    versions: [Brainflop => "brainflop"],
    extensions: [],
}

const BRAINFLOP: &str = r"++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*
+++++++++++++++++++++++++++++*
+++++++**
+++*#
++++++++++++++++++++++++++++++++*
+++++++++++++++++++++++++++++++++++++++++++++++++++++++*
++++++++++++++++++++++++*
+++*
------*
--------*";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainflop,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainflop,
            syntax: BRAINFLOP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINFLOP,
                    [(
                        TokenKind::Plain,
                        r"++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++++*
+++++++++++++++++++++++++++++*
+++++++**
+++*#
++++++++++++++++++++++++++++++++*
+++++++++++++++++++++++++++++++++++++++++++++++++++++++*
++++++++++++++++++++++++*
+++*
------*
--------*",
                    )],
                )
            },
        }],
    };
