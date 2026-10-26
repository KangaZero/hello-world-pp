//! Hello-world snippet for `ProgrammingLanguageName::XSo`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#X.so>
//! - <https://esolangs.org/wiki/X.so>

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: XSo => "X.so",
    versions: [XSo => "X.so"],
    extensions: [],
}

const X_SO: &str = concat!(
    r"/**
 * ",
    output_for_hello_world!(),
    r#" in X.so.
 */
$HelloWorld (
    $Main(
        !Include("X")
        !X.Show(""#,
    output_for_hello_world!(),
    r#"\n")
    )
)"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::XSo,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::XSo,
            syntax: X_SO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    X_SO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/**
 * ",
                            output_for_hello_world!(),
                            r#" in X.so.
 */
$HelloWorld (
    $Main(
        !Include("X")
        !X.Show(""#,
                            output_for_hello_world!(),
                            r#"\n")
    )
)"#
                        ),
                    )],
                )
            },
        }],
    };
