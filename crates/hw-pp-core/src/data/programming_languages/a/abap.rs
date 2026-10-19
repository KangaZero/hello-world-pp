//! Hello-world snippet for `ProgrammingLanguageName::Abap`.
//!
//! Imported from leachim6/hello-world (`a/ABAP.abap` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ABAP4.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ABAP.abap>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ABAP4.txt>
//! - <http://helloworldcollection.de/#ABAP4>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Abap => "ABAP",
    versions: [Abap => "ABAP", Abap4 => "ABAP4"],
    extensions: [Abap => "abap"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Abap,
        extensions: &[FileExtension::Abap],
        categories: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Abap,
                syntax: concat!(
                    r"REPORT ZHELLO_WORLD.

START-OF-SELECTION.
    WRITE: '",
                    output_for_hello_world!(),
                    r"'.

"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"REPORT ZHELLO_WORLD.

START-OF-SELECTION.
    WRITE: '",
                        output_for_hello_world!(),
                        r"'.

"
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Abap4,
                syntax: concat!(
                    r"REPORT ZHB00001.
*",
                    output_for_hello_world!(),
                    r" in ABAP/4 *
WRITE: '",
                    output_for_hello_world!(),
                    r"'."
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"REPORT ZHB00001.
*",
                        output_for_hello_world!(),
                        r" in ABAP/4 *
WRITE: '",
                        output_for_hello_world!(),
                        r"'."
                    ),
                )],
            },
        ],
    };
