//! Hello-world snippet for `ProgrammingLanguageName::Gridscript`.
//!
//! Imported from leachim6/hello-world (`g/GridScript.gridscript` at `a152253`),
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
    language: Gridscript => "GridScript",
    versions: [Gridscript => "GridScript"],
    extensions: [Gridscript => "gridscript"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gridscript,
        extensions: &[FileExtension::Gridscript],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gridscript,
            syntax: concat!(
                r"#",
                output_for_hello_world!(),
                r".

@width 3
@height 1

(1,1):START
(3,1):PRINT '",
                output_for_hello_world!(),
                r"' "
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#",
                    output_for_hello_world!(),
                    r".

@width 3
@height 1

(1,1):START
(3,1):PRINT '",
                    output_for_hello_world!(),
                    r"' "
                ),
            )],
        }],
    };
