//! Hello-world snippet for `ProgrammingLanguageName::Subskin`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Subskin>
//! - <https://esolangs.org/wiki/Subskin>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Subskin => "Subskin",
    versions: [Subskin => "Subskin"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Subskin,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Subskin,
            syntax: r"2
48
18
a
1
2
4
2
a
6
0
100
a
21
64
6c
72
6f
77
20
2c
6f
6c
6c
65",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"2
48
18
a
1
2
4
2
a
6
0
100
a
21
64
6c
72
6f
77
20
2c
6f
6c
6c
65",
            )],
        }],
    };
