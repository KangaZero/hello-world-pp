//! Hello-world snippet for `ProgrammingLanguageName::Borth`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#borth>
//! - <https://esolangs.org/wiki/Borth>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Borth => "borth",
    versions: [Borth => "borth"],
    extensions: [],
}

const BORTH: &str = concat!(
    r"S:
",
    output_for_hello_world!(),
    r"
print
E:"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Borth,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Borth,
            syntax: BORTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BORTH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"S:
",
                            output_for_hello_world!(),
                            r"
print
E:"
                        ),
                    )],
                )
            },
        }],
    };
