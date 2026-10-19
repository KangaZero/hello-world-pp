//! Hello-world snippet for `ProgrammingLanguageName::Packlang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Packlang>
//! - <https://esolangs.org/wiki/Packlang>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Packlang => "Packlang",
    versions: [Packlang => "Packlang"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Packlang,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Packlang,
            syntax: r"Package : IO {
  Integer main {
    charPut(72);
    charPut(101);
    charPut(108);
    charPut(108);
    charPut(111);
    charPut(44);
    charPut(32);
    charPut(87);
    charPut(111);
    charPut(114);
    charPut(108);
    charPut(100);
    charPut(33);
    charPut(13);
    charPut(10);
    0;
  }
} helloWorld;",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"Package : IO {
  Integer main {
    charPut(72);
    charPut(101);
    charPut(108);
    charPut(108);
    charPut(111);
    charPut(44);
    charPut(32);
    charPut(87);
    charPut(111);
    charPut(114);
    charPut(108);
    charPut(100);
    charPut(33);
    charPut(13);
    charPut(10);
    0;
  }
} helloWorld;",
            )],
        }],
    };
