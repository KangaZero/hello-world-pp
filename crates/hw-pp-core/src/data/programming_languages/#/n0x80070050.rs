//! Hello-world snippet for `ProgrammingLanguageName::N0x80070050`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#0x80070050>
//! - <https://esolangs.org/wiki/0x80070050>

use crate::data::programming_languages::ProgrammingParadigm::StringRewriting;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: N0x80070050 => "0x80070050",
    versions: [N0x80070050 => "0x80070050"],
    extensions: [],
}

const N0X80070050: &str = concat!(
    r"❮",
    output_for_hello_world!(),
    r"❯
₫"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N0x80070050,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StringRewriting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N0x80070050,
            syntax: N0X80070050,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N0X80070050,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"❮",
                            output_for_hello_world!(),
                            r"❯
₫"
                        ),
                    )],
                )
            },
        }],
    };
