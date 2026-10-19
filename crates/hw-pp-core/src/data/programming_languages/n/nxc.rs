//! Hello-world snippet for `ProgrammingLanguageName::Nxc`.
//!
//! Imported from leachim6/hello-world (`n/NXC.nxc` at `a152253`),
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
    language: Nxc => "NXC",
    versions: [Nxc => "NXC"],
    extensions: [Nxc => "nxc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nxc,
        extensions: &[FileExtension::Nxc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nxc,
            syntax: concat!(
                r#"task main()
{
    TextOut(0, LCD_LINE1, ""#,
                output_for_hello_world!(),
                r#"");
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"task main()
{
    TextOut(0, LCD_LINE1, ""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
            )],
        }],
    };
