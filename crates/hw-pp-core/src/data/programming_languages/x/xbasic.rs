//! Hello-world snippet for `ProgrammingLanguageName::Xbasic`.
//!
//! Imported from leachim6/hello-world (`x/XBasic.x` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Xbasic => "XBasic",
    versions: [Xbasic => "XBasic"],
    extensions: [X => "x"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Xbasic,
        extensions: &[FileExtension::X],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Xbasic,
            syntax: concat!(
                r#"IMPORT "xst"
DECLARE FUNCTION  Hello ()

FUNCTION  Hello ()
 XstDisplayConsole ()
 PRINT ""#,
                output_for_hello_world!(),
                r#""
END FUNCTION
END PROGRAM"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"IMPORT "xst"
DECLARE FUNCTION  Hello ()

FUNCTION  Hello ()
 XstDisplayConsole ()
 PRINT ""#,
                    output_for_hello_world!(),
                    r#""
END FUNCTION
END PROGRAM"#
                ),
            )],
        }],
    };
