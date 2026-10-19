//! Hello-world snippet for `ProgrammingLanguageName::TranslatedU4e2dU6587`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Translated_%E4%B8%AD%E6%96%87>
//! - <https://esolangs.org/wiki/Translated_%E4%B8%AD%E6%96%87>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: TranslatedU4e2dU6587 => "Translated 中文",
    versions: [TranslatedU4e2dU6587 => "Translated 中文"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TranslatedU4e2dU6587,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TranslatedU4e2dU6587,
            syntax: r"T[UNK]kikik сиңa һ UNK]r character[UNK]chenbarder.1(0,0,72)1(0,0,101)1(0,0,108)1(0,0,108)1(0,0,111)1(0,0,44)1(0,0,32)1(0,0,119)1(0,0,111)1(0,0,114)1(0,0,108)1(0,0,100)*
nothing here
nothing here
nothing here-",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"T[UNK]kikik сиңa һ UNK]r character[UNK]chenbarder.1(0,0,72)1(0,0,101)1(0,0,108)1(0,0,108)1(0,0,111)1(0,0,44)1(0,0,32)1(0,0,119)1(0,0,111)1(0,0,114)1(0,0,108)1(0,0,100)*
nothing here
nothing here
nothing here-",
            )],
        }],
    };
