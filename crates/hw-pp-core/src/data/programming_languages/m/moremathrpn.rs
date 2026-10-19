//! Hello-world snippet for `ProgrammingLanguageName::Moremathrpn`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MoreMathRPN>
//! - <https://esolangs.org/wiki/MoreMathRPN>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Moremathrpn => "MoreMathRPN",
    versions: [Moremathrpn => "MoreMathRPN"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Moremathrpn,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Moremathrpn,
            syntax: concat!(r#"outputS ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"outputS ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
