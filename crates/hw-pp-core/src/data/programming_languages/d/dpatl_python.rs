//! Hello-world snippet for `ProgrammingLanguageName::DpatlPython`.
//!
//! Imported from leachim6/hello-world (`d/DPATL Python.dpatl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/DPATL%20Python.dpatl>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: DpatlPython => "DPATL Python",
    versions: [DpatlPython => "DPATL Python"],
    extensions: [Dpatl => "dpatl"],
}

const DPATL_PYTHON: &str = concat!(
    r#"{start python}
print(""#,
    output_for_hello_world!(),
    r#"")
{end python}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DpatlPython,
        extensions: &[FileExtension::Dpatl],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DpatlPython,
            syntax: DPATL_PYTHON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DPATL_PYTHON,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"{start python}
print(""#,
                            output_for_hello_world!(),
                            r#"")
{end python}"#
                        ),
                    )],
                )
            },
        }],
    };
