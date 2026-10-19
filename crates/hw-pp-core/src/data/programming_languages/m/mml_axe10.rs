//! Hello-world snippet for `ProgrammingLanguageName::MmlAxe10`.
//!
//! Imported from the Hello World Collection (`files/MML-AXE10.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MML-AXE10.txt>
//! - <http://helloworldcollection.de/#MML-AXE10>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MmlAxe10 => "MML-AXE10",
    versions: [MmlAxe10 => "MML-AXE10"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MmlAxe10,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MmlAxe10,
            syntax: concat!(
                r"! ",
                output_for_hello_world!(),
                r" program in MML for Ericsson's AXE10 telephone exchange
IOTXP:",
                output_for_hello_world!(),
                r";"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"! ",
                    output_for_hello_world!(),
                    r" program in MML for Ericsson's AXE10 telephone exchange
IOTXP:",
                    output_for_hello_world!(),
                    r";"
                ),
            )],
        }],
    };
