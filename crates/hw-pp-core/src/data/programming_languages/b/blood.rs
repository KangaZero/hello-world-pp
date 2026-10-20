//! Hello-world snippet for `ProgrammingLanguageName::Blood`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Blood>
//! - <https://esolangs.org/wiki/Blood32>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Blood => "Blood",
    versions: [Blood => "Blood"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Blood,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Blood,
            syntax: concat!(
                r"<a grid of 0 cells>
[0,0]
<set tape to '",
                output_for_hello_world!(),
                r"' with ASCII values in binary (reversed)>
T: 00100010B00110010B01001010B11110010B11101010B000001B11110010B00110010B00110010B10100010B0001001
<note that if the tape input is interrupted by a newline in this source, the newline should be ignored>
{
C()
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<a grid of 0 cells>
[0,0]
<set tape to '",
                    output_for_hello_world!(),
                    r"' with ASCII values in binary (reversed)>
T: 00100010B00110010B01001010B11110010B11101010B000001B11110010B00110010B00110010B10100010B0001001
<note that if the tape input is interrupted by a newline in this source, the newline should be ignored>
{
C()
}"
                ),
            )],
        }],
    };
