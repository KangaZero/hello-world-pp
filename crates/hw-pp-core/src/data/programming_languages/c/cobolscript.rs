//! Hello-world snippet for `ProgrammingLanguageName::Cobolscript`.
//!
//! Imported from leachim6/hello-world (`c/CobolScript.cbl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cobolscript => "CobolScript",
    versions: [Cobolscript => "CobolScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cobolscript,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cobolscript,
            syntax: concat!(
                r"DISPLAY `Content-type: text/html `.
DISPLAY LINEFEED.
DISPLAY `<HTML><BODY>`.
DISPLAY `<CENTER>",
                output_for_hello_world!(),
                r"</CENTER>`.
DISPLAY `</BODY></HTML>`.
GOBACK."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"DISPLAY `Content-type: text/html `.
DISPLAY LINEFEED.
DISPLAY `<HTML><BODY>`.
DISPLAY `<CENTER>",
                    output_for_hello_world!(),
                    r"</CENTER>`.
DISPLAY `</BODY></HTML>`.
GOBACK."
                ),
            )],
        }],
    };
