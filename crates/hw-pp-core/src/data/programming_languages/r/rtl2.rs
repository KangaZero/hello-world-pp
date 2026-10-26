//! Hello-world snippet for `ProgrammingLanguageName::Rtl2`.
//!
//! Imported from leachim6/hello-world (`r/RTL2.rtl2` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RTL2.rtl2>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Rtl2 => "RTL2",
    versions: [Rtl2 => "RTL2"],
    extensions: [Rtl2 => "rtl2"],
}

const RTL2: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rtl2,
        extensions: &[FileExtension::Rtl2],
        categories: &[Historical],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rtl2,
            syntax: RTL2,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RTL2,
                    [(
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
                )
            },
        }],
    };
