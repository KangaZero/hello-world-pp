//! Hello-world snippet for `ProgrammingLanguageName::IbmExec2`.
//!
//! Imported from the Hello World Collection (`files/IBM-Exec2.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/IBM-Exec2.txt>
//! - <http://helloworldcollection.de/#IBM-Exec2>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: IbmExec2 => "IBM-Exec2",
    versions: [IbmExec2 => "IBM-Exec2"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IbmExec2,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IbmExec2,
            syntax: concat!(
                output_for_hello_world!(),
                r" for IBM EXEC2 (under VM/CMS)

&TRACE OFF
*
&TYPE ",
                output_for_hello_world!(),
                r"
*
&EXIT 0"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    output_for_hello_world!(),
                    r" for IBM EXEC2 (under VM/CMS)

&TRACE OFF
*
&TYPE ",
                    output_for_hello_world!(),
                    r"
*
&EXIT 0"
                ),
            )],
        }],
    };
