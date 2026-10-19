//! Hello-world snippet for `ProgrammingLanguageName::Spaghetti`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Spaghetti>
//! - <https://esolangs.org/wiki/Spaghetti>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Spaghetti => "Spaghetti",
    versions: [Spaghetti => "Spaghetti"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spaghetti,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spaghetti,
            syntax: r"10[?100]11
9[?108]10
13[?101]2
12[?10]0
4[?111]5
8[?114]9
2[?108]3
11[?33]12
6[?87]7
3[?108]4
1[?48]13
5[?32]6
7[?111]8",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"10[?100]11
9[?108]10
13[?101]2
12[?10]0
4[?111]5
8[?114]9
2[?108]3
11[?33]12
6[?87]7
3[?108]4
1[?48]13
5[?32]6
7[?111]8",
            )],
        }],
    };
