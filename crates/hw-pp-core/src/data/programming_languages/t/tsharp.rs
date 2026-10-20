//! Hello-world snippet for `ProgrammingLanguageName::Tsharp`.
//!
//! Imported from leachim6/hello-world (`t/TSharp.tsp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TSharp.tsp>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Tsharp => "TSharp",
    versions: [Tsharp => "TSharp"],
    extensions: [Tsp => "tsp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Tsharp,
        extensions: &[FileExtension::Tsp],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Tsharp,
            syntax: concat!(r#"push ""#, output_for_hello_world!(), r#"" print"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"push ""#, output_for_hello_world!(), r#"" print"#),
            )],
        }],
    };
