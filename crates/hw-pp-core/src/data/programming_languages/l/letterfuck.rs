//! Hello-world snippet for `ProgrammingLanguageName::Letterfuck`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#Letterfuck>
//! - <https://esolangs.org/wiki/Letterfuck>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Letterfuck => "Letterfuck",
    versions: [Letterfuck => "Letterfuck"],
    extensions: [],
}

const LETTERFUCK: &str = concat!(
    r#"out(char), ""#,
    output_for_hello_world!(),
    r#""
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Letterfuck,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Letterfuck,
            syntax: LETTERFUCK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LETTERFUCK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"out(char), ""#,
                            output_for_hello_world!(),
                            r#""
end"#
                        ),
                    )],
                )
            },
        }],
    };
