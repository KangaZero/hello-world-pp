//! Hello-world snippet for `ProgrammingLanguageName::Gridscript`.
//!
//! Imported from leachim6/hello-world (`g/GridScript.gridscript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/GridScript.gridscript>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Gridscript => "GridScript",
    versions: [Gridscript => "GridScript"],
    extensions: [Gridscript => "gridscript"],
}

const GRIDSCRIPT: &str = concat!(
    r"#",
    output_for_hello_world!(),
    r".

@width 3
@height 1

(1,1):START
(3,1):PRINT '",
    output_for_hello_world!(),
    r"' "
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gridscript,
        extensions: &[FileExtension::Gridscript],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gridscript,
            syntax: GRIDSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GRIDSCRIPT,
                    [(
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
                )
            },
        }],
    };
