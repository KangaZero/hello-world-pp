//! Hello-world snippet for `ProgrammingLanguageName::Ehtml`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#EHTML>
//! - <https://esolangs.org/wiki/EHTML>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ehtml => "EHTML",
    versions: [Ehtml => "EHTML"],
    extensions: [],
}

const EHTML: &str = concat!(
    r#"<set>
    <main>
    <function>
        <print>""#,
    output_for_hello_world!(),
    r#""</print>
    </function>
</set>"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ehtml,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ehtml,
            syntax: EHTML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EHTML,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"<set>
    <main>
    <function>
        <print>""#,
                            output_for_hello_world!(),
                            r#""</print>
    </function>
</set>"#
                        ),
                    )],
                )
            },
        }],
    };
