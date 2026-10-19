//! Hello-world snippet for `ProgrammingLanguageName::Cobolscript`.
//!
//! Imported from leachim6/hello-world (`c/CobolScript.cbl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CobolScript.cbl>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
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
