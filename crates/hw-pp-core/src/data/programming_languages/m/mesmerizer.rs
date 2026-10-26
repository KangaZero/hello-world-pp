//! Hello-world snippet for `ProgrammingLanguageName::Mesmerizer`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Mesmerizer>
//! - <https://esolangs.org/wiki/Mesmerizer>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Mesmerizer => "Mesmerizer",
    versions: [Mesmerizer => "Mesmerizer"],
    extensions: [],
}

const MESMERIZER: &str = concat!(
    r"Ready for Performance!
Send Message “",
    output_for_hello_world!(),
    r"”
Shut down…"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mesmerizer,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mesmerizer,
            syntax: MESMERIZER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MESMERIZER,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Ready for Performance!
Send Message “",
                            output_for_hello_world!(),
                            r"”
Shut down…"
                        ),
                    )],
                )
            },
        }],
    };
