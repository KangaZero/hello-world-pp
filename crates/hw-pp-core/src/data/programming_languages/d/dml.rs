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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Dml => "DML",
    versions: [Dml => "DML"],
    extensions: [],
}

const DML: &str = concat!(
    r"! ",
    output_for_hello_world!(),
    r#" in DML (Gembase database language)

PROCEDURE_FORM MAIN
        PRINT/NOWAIT (""#,
    output_for_hello_world!(),
    r#"")
END_FORM"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dml,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dml,
            syntax: DML,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DML,
                    [(
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
                )
            },
        }],
    };
