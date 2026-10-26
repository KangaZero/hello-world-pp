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
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: FuncPlusPlus => "Func++",
    versions: [FuncPlusPlus => "Func++"],
    extensions: [],
}

const FUNC_PLUS_PLUS: &str =
    r"do(do(do(do(do(do(do(do(do(do(do(H(),e()),l()),l()),o()), ()),W()),o()),r()),l()),d()),!())";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FuncPlusPlus,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FuncPlusPlus,
            syntax: FUNC_PLUS_PLUS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FUNC_PLUS_PLUS,
                    [(
                        TokenKind::Plain,
                        r"do(do(do(do(do(do(do(do(do(do(do(H(),e()),l()),l()),o()), ()),W()),o()),r()),l()),d()),!())",
                    )],
                )
            },
        }],
    };
