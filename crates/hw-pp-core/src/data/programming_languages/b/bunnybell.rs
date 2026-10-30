//! Hello-world snippet for `ProgrammingLanguageName::Bunnybell`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#BunnyBell>
//! - <https://esolangs.org/wiki/BunnyBell>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Bunnybell => "BunnyBell",
    versions: [Bunnybell => "BunnyBell"],
    extensions: [],
}

const BUNNYBELL: &str = concat!(
    r"func main
 print ",
    output_for_hello_world!(),
    r"
 return"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bunnybell,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bunnybell,
            syntax: BUNNYBELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUNNYBELL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"func main
 print ",
                            output_for_hello_world!(),
                            r"
 return"
                        ),
                    )],
                )
            },
        }],
    };
