//! Hello-world snippet for `ProgrammingLanguageName::Recorder`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Recorder>
//! - <https://esolangs.org/wiki/Recorder>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Recorder => "Recorder",
    versions: [Recorder => "Recorder"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Recorder,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Recorder,
            syntax: concat!(
                output_for_hello_world!(),
                r#"
#alternitave
""#,
                output_for_hello_world!(),
                r#""
#more alternative
"some text ok ("#,
                output_for_hello_world!(),
                r#") yay wow" : () = a
a"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r#"
#alternitave
""#,
                    output_for_hello_world!(),
                    r#""
#more alternative
"some text ok ("#,
                    output_for_hello_world!(),
                    r#") yay wow" : () = a
a"#
                ),
            )],
        }],
    };
