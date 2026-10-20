//! Hello-world snippet for `ProgrammingLanguageName::Elena`.
//!
//! Imported from leachim6/hello-world (`e/Elena.elena` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ELENA&nbsp;3.0.htm`, `files/ELENA&nbsp;4.0.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/Elena.elena>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ELENA%26nbsp%3B3.0.htm>
//! - <http://helloworldcollection.de/#ELENA%C2%A03.0>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ELENA%26nbsp%3B4.0.htm>
//! - <http://helloworldcollection.de/#ELENA%C2%A04.0>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Elena => "Elena",
    versions: [Elena => "Elena", Elena30 => "ELENA 3.0", Elena40 => "ELENA 4.0"],
    extensions: [Elena => "elena"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Elena,
        extensions: &[FileExtension::Elena],
        categories: &[Compiled],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Elena,
                syntax: concat!(
                    r#"public program
[
    console writeLine(""#,
                    output_for_hello_world!(),
                    r#"").
]"#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r#"public program
[
    console writeLine(""#,
                        output_for_hello_world!(),
                        r#"").
]"#
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Elena30,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ELENA 3.0

program =
[
    console writeLine:""#,
                    output_for_hello_world!(),
                    r#"".
]."#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"// ",
                        output_for_hello_world!(),
                        r#" in ELENA 3.0

program =
[
    console writeLine:""#,
                        output_for_hello_world!(),
                        r#"".
]."#
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Elena40,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ELENA 4.0

public program()
{
    console.writeLine(""#,
                    output_for_hello_world!(),
                    r#"")
}"#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"// ",
                        output_for_hello_world!(),
                        r#" in ELENA 4.0

public program()
{
    console.writeLine(""#,
                        output_for_hello_world!(),
                        r#"")
}"#
                    ),
                )],
            },
        ],
    };
