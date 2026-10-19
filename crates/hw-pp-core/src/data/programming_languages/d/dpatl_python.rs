//! Hello-world snippet for `ProgrammingLanguageName::DpatlPython`.
//!
//! Imported from leachim6/hello-world (`d/DPATL Python.dpatl` at `a152253`),
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
    language: DpatlPython => "DPATL Python",
    versions: [DpatlPython => "DPATL Python"],
    extensions: [Dpatl => "dpatl"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::DpatlPython,
        extensions: &[FileExtension::Dpatl],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::DpatlPython,
            syntax: concat!(
                r#"{start python}
print(""#,
                output_for_hello_world!(),
                r#"")
{end python}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"{start python}
print(""#,
                    output_for_hello_world!(),
                    r#"")
{end python}"#
                ),
            )],
        }],
    };
