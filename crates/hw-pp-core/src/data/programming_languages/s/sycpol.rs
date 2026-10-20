//! Hello-world snippet for `ProgrammingLanguageName::Sycpol`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#SYCPOL>
//! - <https://esolangs.org/wiki/SYCPOL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sycpol => "SYCPOL",
    versions: [Sycpol => "SYCPOL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sycpol,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sycpol,
            syntax: concat!(
                r#"COMMENT           CARD
STANDARD SYSTEM CARD I

THESE  CARDS  PROVIDED
IN THIS  FILE  ARE THE
IMPLEMENTATION  OF THE
FAMOUS  ""#,
                output_for_hello_world!(),
                r#""
PROGRAM   IN   SYCPOL.

THE IMPLEMENTATION  OF
THE  "PRINT TO STREAM"
PROCEDURE     FROM THE
STANDARD LIBRARY    IS
INCLUDED FOR THOSE WHO
DON'T HAVE IT ALREADY.

THESE CARDS        ARE
RELEASED  IN SO CALLED
"PUBLIC DOMAIN"."#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"COMMENT           CARD
STANDARD SYSTEM CARD I

THESE  CARDS  PROVIDED
IN THIS  FILE  ARE THE
IMPLEMENTATION  OF THE
FAMOUS  ""#,
                    output_for_hello_world!(),
                    r#""
PROGRAM   IN   SYCPOL.

THE IMPLEMENTATION  OF
THE  "PRINT TO STREAM"
PROCEDURE     FROM THE
STANDARD LIBRARY    IS
INCLUDED FOR THOSE WHO
DON'T HAVE IT ALREADY.

THESE CARDS        ARE
RELEASED  IN SO CALLED
"PUBLIC DOMAIN"."#
                ),
            )],
        }],
    };
