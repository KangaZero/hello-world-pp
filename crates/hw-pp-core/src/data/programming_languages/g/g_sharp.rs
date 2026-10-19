//! Hello-world snippet for `ProgrammingLanguageName::GSharp`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#G%23>
//! - <https://esolangs.org/wiki/G_Sharp>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: GSharp => "G#",
    versions: [GSharp => "G#"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GSharp,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GSharp,
            syntax: concat!(
                r#"make main[print[""#,
                output_for_hello_world!(),
                r#"\n"]]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"make main[print[""#,
                    output_for_hello_world!(),
                    r#"\n"]]"#
                ),
            )],
        }],
    };
