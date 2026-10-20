//! Hello-world snippet for `ProgrammingLanguageName::IntroduceYourself`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Introduce_yourself>
//! - <https://esolangs.org/wiki/Introduce_yourself>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: IntroduceYourself => "Introduce yourself",
    versions: [IntroduceYourself => "Introduce yourself"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IntroduceYourself,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IntroduceYourself,
            syntax: r"Hi, I am H, I am 72 years old.
How old are you in character, H?
Hi, I am e, I am 101 years old.
How old are you in character, e?
Hi, I am l, I am 108 years old.
How old are you in character, l?
How old are you in character, l?
Hi, I am o, I am 111 years old.
How old are you in character, o?
Hi, I am c, I am 32 years old.
How old are you in character, c?
Hi, I am W, I am 87 years old.
How old are you in character, W?
How old are you in character, o?
Hi, I am r, I am 114 years old.
How old are you in character, r?
How old are you in character, l?
Hi, I am d, I am 100 years old.
How old are you in character, d?
Hi, I am z, I am 33 years old.
How old are you in character, z?",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Hi, I am H, I am 72 years old.
How old are you in character, H?
Hi, I am e, I am 101 years old.
How old are you in character, e?
Hi, I am l, I am 108 years old.
How old are you in character, l?
How old are you in character, l?
Hi, I am o, I am 111 years old.
How old are you in character, o?
Hi, I am c, I am 32 years old.
How old are you in character, c?
Hi, I am W, I am 87 years old.
How old are you in character, W?
How old are you in character, o?
Hi, I am r, I am 114 years old.
How old are you in character, r?
How old are you in character, l?
Hi, I am d, I am 100 years old.
How old are you in character, d?
Hi, I am z, I am 33 years old.
How old are you in character, z?",
            )],
        }],
    };
