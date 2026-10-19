//! Hello-world snippet for `ProgrammingLanguageName::Powerflex`.
//!
//! Imported from the Hello World Collection (`files/Powerflex.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Powerflex.txt>
//! - <http://helloworldcollection.de/#Powerflex>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Powerflex => "Powerflex",
    versions: [Powerflex => "Powerflex"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Powerflex,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Powerflex,
            syntax: concat!(
                r#"// ""#,
                output_for_hello_world!(),
                r#"" in PowerFlex

showln ""#,
                output_for_hello_world!(),
                r#""
system"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"// ""#,
                    output_for_hello_world!(),
                    r#"" in PowerFlex

showln ""#,
                    output_for_hello_world!(),
                    r#""
system"#
                ),
            )],
        }],
    };
