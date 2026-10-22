//! Hello-world snippets for `ProgrammingLanguageName::K`.
//!
//! Imported from leachim6/hello-world (`k/K.k` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/K3.htm`, `files/K4.htm`, `files/K5.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/K.k>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/K3.htm>
//! - <http://helloworldcollection.de/#K3>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/K4.htm>
//! - <http://helloworldcollection.de/#K4>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/K5.htm>
//! - <http://helloworldcollection.de/#K5>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Apl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: K => "K",
    versions: [K => "K", K3 => "K3", K4 => "K4", K5 => "K5"],
    extensions: [K => "k"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::K,
        extensions: &[FileExtension::K],
        categories: &[Scripting],
        families: &[Apl],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::K,
                syntax: concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(r#"""#, output_for_hello_world!(), r#"\n""#),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::K3,
                syntax: concat!(
                    r"/ ",
                    output_for_hello_world!(),
                    r#" in K3

`0:""#,
                    output_for_hello_world!(),
                    r"\n"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"/ ",
                        output_for_hello_world!(),
                        r#" in K3

`0:""#,
                        output_for_hello_world!(),
                        r"\n"
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::K4,
                syntax: concat!(
                    r"/ ",
                    output_for_hello_world!(),
                    r#" in K4/Q/KDB

1""#,
                    output_for_hello_world!(),
                    r#"\n""#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"/ ",
                        output_for_hello_world!(),
                        r#" in K4/Q/KDB

1""#,
                        output_for_hello_world!(),
                        r#"\n""#
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::K5,
                syntax: concat!(
                    r"/ ",
                    output_for_hello_world!(),
                    r#" in K5

1'""#,
                    output_for_hello_world!(),
                    r#"\n""#
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"/ ",
                        output_for_hello_world!(),
                        r#" in K5

1'""#,
                        output_for_hello_world!(),
                        r#"\n""#
                    ),
                )],
            },
        ],
    };
