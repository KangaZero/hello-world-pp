//! Hello-world snippet for `ProgrammingLanguageName::TrollOnline`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Troll_Online>
//! - <https://esolangs.org/wiki/Troll_Online>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TrollOnline => "Troll Online",
    versions: [TrollOnline => "Troll Online"],
    extensions: [],
}

const TROLL_ONLINE: &str = concat!(
    output_for_hello_world!(),
    r"!¨ç]a
↓ùp👌a"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TrollOnline,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TrollOnline,
            syntax: TROLL_ONLINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TROLL_ONLINE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r"!¨ç]a
↓ùp👌a"
                        ),
                    )],
                )
            },
        }],
    };
