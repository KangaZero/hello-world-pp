//! Hello-world snippet for `ProgrammingLanguageName::Tibasic`.
//!
//! Imported from leachim6/hello-world (`t/TIBasic.ti` at `a152253`),
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
    language: Tibasic => "TIBasic",
    versions: [Tibasic => "TIBasic"],
    extensions: [Ti => "ti"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tibasic,
        extensions: &[FileExtension::Ti],
        categories: &[],
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
