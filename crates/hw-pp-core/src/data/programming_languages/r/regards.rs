//! Hello-world snippet for `ProgrammingLanguageName::Regards`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Regards,>
//! - <https://esolangs.org/wiki/Regards,>

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Regards => "Regards,",
    versions: [Regards => "Regards,"],
    extensions: [],
}

const REGARDS: &str = concat!(
    r"Subject: ",
    output_for_hello_world!(),
    r#"

Hi world,

Circling back on ""#,
    output_for_hello_world!(),
    r#"".

Best,
Ihor"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Regards,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Regards,
            syntax: REGARDS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REGARDS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"Subject: ",
                            output_for_hello_world!(),
                            r#"

Hi world,

Circling back on ""#,
                            output_for_hello_world!(),
                            r#"".

Best,
Ihor"#
                        ),
                    )],
                )
            },
        }],
    };
