//! Hello-world snippet for `ProgrammingLanguageName::Bs2000`.
//!
//! Imported from the Hello World Collection (`files/BS2000` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BS2000>
//! - <http://helloworldcollection.de/#BS2000>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Bs2000 => "BS2000",
    versions: [Bs2000 => "BS2000"],
    extensions: [],
}

const BS2000: &str = concat!(
    r#"/BEGIN-PROCEDURE LOGGING=N
/REMARK ""#,
    output_for_hello_world!(),
    r#"" IN BS2000 (SDF)
/ASSIGN-SYSDTA TO-FILE=*SYSCMD
/WRITE-TEXT '"#,
    output_for_hello_world!(),
    r"'
/SET-JOB-STEP
/ASSIGN-SYSDTA TO-FILE=*PRIMARY
/END-PROCEDURE"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Bs2000,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Bs2000,
            syntax: BS2000,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BS2000,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"/BEGIN-PROCEDURE LOGGING=N
/REMARK ""#,
                            output_for_hello_world!(),
                            r#"" IN BS2000 (SDF)
/ASSIGN-SYSDTA TO-FILE=*SYSCMD
/WRITE-TEXT '"#,
                            output_for_hello_world!(),
                            r"'
/SET-JOB-STEP
/ASSIGN-SYSDTA TO-FILE=*PRIMARY
/END-PROCEDURE"
                        ),
                    )],
                )
            },
        }],
    };
