//! Hello-world snippet for `ProgrammingLanguageName::Rtl2`.
//!
//! Imported from leachim6/hello-world (`r/RTL2.rtl2` at `a152253`),
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
    language: Rtl2 => "RTL2",
    versions: [Rtl2 => "RTL2"],
    extensions: [Rtl2 => "rtl2"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rtl2,
        extensions: &[FileExtension::Rtl2],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rtl2,
            syntax: concat!(
                r"TITLE ",
                output_for_hello_world!(),
                r#";

LET NL=10;

EXT PROC(REF ARRAY BYTE) TWRT;

ENT PROC INT RRJOB();

    TWRT(""#,
                output_for_hello_world!(),
                r#"#NL#");
    RETURN(1);

ENDPROC;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"TITLE ",
                    output_for_hello_world!(),
                    r#";

LET NL=10;

EXT PROC(REF ARRAY BYTE) TWRT;

ENT PROC INT RRJOB();

    TWRT(""#,
                    output_for_hello_world!(),
                    r#"#NL#");
    RETURN(1);

ENDPROC;"#
                ),
            )],
        }],
    };
