//! Hello-world snippet for `ProgrammingLanguageName::TsoClist`.
//!
//! Imported from the Hello World Collection (`files/TSO&nbsp;CLIST` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/TSO%26nbsp%3BCLIST>
//! - <http://helloworldcollection.de/#TSO%C2%A0CLIST>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: TsoClist => "TSO CLIST",
    versions: [TsoClist => "TSO CLIST"],
    extensions: [],
}

const TSO_CLIST: &str = concat!(
    r"PROC 0
/* ",
    output_for_hello_world!(),
    r" in TSO CLIST */
write ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TsoClist,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TsoClist,
            syntax: TSO_CLIST,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TSO_CLIST,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"PROC 0
/* ",
                            output_for_hello_world!(),
                            r" in TSO CLIST */
write ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
