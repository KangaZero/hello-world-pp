//! Hello-world snippet for `ProgrammingLanguageName::Babalang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#Babalang>
//! - <https://esolangs.org/wiki/Babalang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::language_names;

language_names! {
    language: Babalang => "Babalang",
    versions: [Babalang => "Babalang"],
    extensions: [],
}

const BABALANG: &str = r"// Constants
1 is you and move 2 is 1 and 1 4 is 2 and 2 8 is 4 and 4 
16 is 8 and 8 32 is 16 and 16 64 is 32 and 32
 
// Stack
hello_world is group

// Chars
H is 64 and 8 e is 64 and 32 and 4 and 1 l is 64 and 32 and 8 and 4 
o is 64 and 32 and 16 and not 1 comma is 32 and 8 and 4 sp is 32 
w is 64 and 32 and 16 and 8 and not 1 r is 64 and 32 and 16 and 2 
d is 64 and 32 and 4 excl is 32 and 1 newline is 8 and 2

// Push chars to stack
hello_world has H and e and l and l and o and comma and sp 
and w and o and r and l and d and excl and newline

// Print
hello_world is text";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Babalang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Babalang,
            syntax: BABALANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BABALANG,
                    [(
                        TokenKind::Plain,
                        r"// Constants
1 is you and move 2 is 1 and 1 4 is 2 and 2 8 is 4 and 4 
16 is 8 and 8 32 is 16 and 16 64 is 32 and 32
 
// Stack
hello_world is group

// Chars
H is 64 and 8 e is 64 and 32 and 4 and 1 l is 64 and 32 and 8 and 4 
o is 64 and 32 and 16 and not 1 comma is 32 and 8 and 4 sp is 32 
w is 64 and 32 and 16 and 8 and not 1 r is 64 and 32 and 16 and 2 
d is 64 and 32 and 4 excl is 32 and 1 newline is 8 and 2

// Push chars to stack
hello_world has H and e and l and l and o and comma and sp 
and w and o and r and l and d and excl and newline

// Print
hello_world is text",
                    )],
                )
            },
        }],
    };
