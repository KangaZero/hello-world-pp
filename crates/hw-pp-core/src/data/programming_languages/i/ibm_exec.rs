//! Hello-world snippet for `ProgrammingLanguageName::IbmExec`.
//!
//! Imported from the Hello World Collection (`files/IBM-Exec.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/IBM-Exec.txt>
//! - <http://helloworldcollection.de/#IBM-Exec>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: IbmExec => "IBM-Exec",
    versions: [IbmExec => "IBM-Exec"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IbmExec,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IbmExec,
            syntax: concat!(
                output_for_hello_world!(),
                r" for IBM EXEC (under VM/CMS)

&CONTROL
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
                    r" for IBM EXEC (under VM/CMS)

&CONTROL
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
