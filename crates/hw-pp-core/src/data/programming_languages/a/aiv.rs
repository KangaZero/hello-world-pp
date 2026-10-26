//! Hello-world snippet for `ProgrammingLanguageName::Aiv`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#AIV>
//! - <https://esolangs.org/wiki/AIV>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aiv => "AIV",
    versions: [Aiv => "AIV"],
    extensions: [],
}

const AIV: &str = concat!(
    r"display«
",
    output_for_hello_world!(),
    r"
»
stop«undisp»"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aiv,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aiv,
            syntax: AIV,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AIV,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"display«
",
                            output_for_hello_world!(),
                            r"
»
stop«undisp»"
                        ),
                    )],
                )
            },
        }],
    };
