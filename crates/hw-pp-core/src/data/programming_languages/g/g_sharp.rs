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
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: GSharp => "G#",
    versions: [GSharp => "G#"],
    extensions: [],
}

const G_SHARP: &str = concat!(
    r#"make main[print[""#,
    output_for_hello_world!(),
    r#"\n"]]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::GSharp,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::GSharp,
            syntax: G_SHARP,
            tokenized_syntax: &const {
                checked_token_lengths(
                    G_SHARP,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"make main[print[""#,
                            output_for_hello_world!(),
                            r#"\n"]]"#
                        ),
                    )],
                )
            },
        }],
    };
