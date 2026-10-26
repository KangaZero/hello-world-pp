//! Hello-world snippet for `ProgrammingLanguageName::Aardvark`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (nonalphabetic and A)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(nonalphabetic_and_A)#Aardvark>
//! - <https://esolangs.org/wiki/Aardvark>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Aardvark => "Aardvark",
    versions: [Aardvark => "Aardvark"],
    extensions: [],
}

const AARDVARK: &str = concat!(
    r#"async main() {
    println ""#,
    output_for_hello_world!(),
    r#""
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Aardvark,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Aardvark,
            syntax: AARDVARK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AARDVARK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"async main() {
    println ""#,
                            output_for_hello_world!(),
                            r#""
}"#
                        ),
                    )],
                )
            },
        }],
    };
