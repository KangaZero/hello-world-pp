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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: IbmExec => "IBM-Exec",
    versions: [IbmExec => "IBM-Exec"],
    extensions: [],
}

const IBM_EXEC: &str = concat!(
    output_for_hello_world!(),
    r" for IBM EXEC (under VM/CMS)

&CONTROL
*
&TYPE ",
    output_for_hello_world!(),
    r"
*
&EXIT 0"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::IbmExec,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::IbmExec,
            syntax: IBM_EXEC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    IBM_EXEC,
                    [(
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
                )
            },
        }],
    };
