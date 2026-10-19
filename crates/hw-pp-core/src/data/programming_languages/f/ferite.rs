//! Hello-world snippet for `ProgrammingLanguageName::Ferite`.
//!
//! Imported from leachim6/hello-world (`f/Ferite.fe` at `a152253`),
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
    language: Ferite => "Ferite",
    versions: [Ferite => "Ferite"],
    extensions: [Fe => "fe"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ferite,
        extensions: &[FileExtension::Fe],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ferite,
            syntax: concat!(
                r#"uses "console";
Console.println( ""#,
                output_for_hello_world!(),
                r#"" );"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"uses "console";
Console.println( ""#,
                    output_for_hello_world!(),
                    r#"" );"#
                ),
            )],
        }],
    };
