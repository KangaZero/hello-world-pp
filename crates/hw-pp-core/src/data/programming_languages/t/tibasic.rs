//! Hello-world snippet for `ProgrammingLanguageName::Tibasic`.
//!
//! Imported from leachim6/hello-world (`t/TIBasic.ti` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TIBasic.ti>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Basic, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tibasic => "TIBasic",
    versions: [Tibasic => "TIBasic"],
    extensions: [Ti => "ti"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tibasic,
        extensions: &[FileExtension::Ti],
        categories: &[Scripting],
        families: &[Basic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tibasic,
            syntax: concat!(
                r#"PROGRAM:HELLOW
:ClrHome
:Disp ""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"PROGRAM:HELLOW
:ClrHome
:Disp ""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
