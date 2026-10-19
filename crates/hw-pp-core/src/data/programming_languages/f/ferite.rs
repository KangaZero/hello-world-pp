//! Hello-world snippet for `ProgrammingLanguageName::Ferite`.
//!
//! Imported from leachim6/hello-world (`f/Ferite.fe` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Ferite.fe>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
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
