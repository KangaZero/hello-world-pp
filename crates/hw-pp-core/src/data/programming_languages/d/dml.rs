//! Hello-world snippet for `ProgrammingLanguageName::Dml`.
//!
//! Imported from the Hello World Collection (`files/DML.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/DML.txt>
//! - <http://helloworldcollection.de/#DML>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dml => "DML",
    versions: [Dml => "DML"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dml,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dml,
            syntax: concat!(
                r"! ",
                output_for_hello_world!(),
                r#" in DML (Gembase database language)

PROCEDURE_FORM MAIN
        PRINT/NOWAIT (""#,
                output_for_hello_world!(),
                r#"")
END_FORM"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"! ",
                    output_for_hello_world!(),
                    r#" in DML (Gembase database language)

PROCEDURE_FORM MAIN
        PRINT/NOWAIT (""#,
                    output_for_hello_world!(),
                    r#"")
END_FORM"#
                ),
            )],
        }],
    };
