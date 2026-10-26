//! Hello-world snippet for `ProgrammingLanguageName::Logos`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#logOS>
//! - <https://esolangs.org/wiki/LogOS>

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Logos => "logOS",
    versions: [Logos => "logOS"],
    extensions: [],
}

const LOGOS: &str = concat!(
    r#"Editor
write ""#,
    output_for_hello_world!(),
    r#""
copy
minimise
Email
paste
send"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logos,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logos,
            syntax: LOGOS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOGOS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Editor
write ""#,
                            output_for_hello_world!(),
                            r#""
copy
minimise
Email
paste
send"#
                        ),
                    )],
                )
            },
        }],
    };
