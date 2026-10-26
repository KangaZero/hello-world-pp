//! Hello-world snippet for `ProgrammingLanguageName::Vbscript`.
//!
//! Imported from the Hello World Collection (`files/VBScript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/VBScript.txt>
//! - <http://helloworldcollection.de/#VBScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Vbscript => "VBScript",
    versions: [Vbscript => "VBScript"],
    extensions: [],
}

const VBSCRIPT: &str = concat!(
    r"' ",
    output_for_hello_world!(),
    r#" in VBScript (Windows Scripting Host)
msgbox ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Vbscript,
        extensions: &[],
        categories: &[],
        families: &[Basic],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Vbscript,
            syntax: VBSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    VBSCRIPT,
                    [
                        (
                            TokenKind::Comment,
                            concat!(
                                r"' ",
                                output_for_hello_world!(),
                                r" in VBScript (Windows Scripting Host)"
                            ),
                        ),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"msgbox"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
