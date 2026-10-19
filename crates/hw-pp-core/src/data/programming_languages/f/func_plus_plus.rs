//! Hello-world snippet for `ProgrammingLanguageName::FuncPlusPlus`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#Func%2B%2B>
//! - <https://esolangs.org/wiki/Func%2B%2B>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: FuncPlusPlus => "Func++",
    versions: [FuncPlusPlus => "Func++"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FuncPlusPlus,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FuncPlusPlus,
            syntax: r"do(do(do(do(do(do(do(do(do(do(do(H(),e()),l()),l()),o()), ()),W()),o()),r()),l()),d()),!())",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"do(do(do(do(do(do(do(do(do(do(do(H(),e()),l()),l()),o()), ()),W()),o()),r()),l()),d()),!())",
            )],
        }],
    };
