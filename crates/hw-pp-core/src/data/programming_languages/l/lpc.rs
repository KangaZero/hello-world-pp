//! Hello-world snippet for `ProgrammingLanguageName::Lpc`.
//!
//! Imported from the Hello World Collection (`files/LPC.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/LPC.htm>
//! - <http://helloworldcollection.de/#LPC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lpc => "LPC",
    versions: [Lpc => "LPC"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lpc,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lpc,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in LPC

void create()
{
       message("info",""#,
                output_for_hello_world!(),
                r#"",this_user());
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in LPC

void create()
{
       message("info",""#,
                    output_for_hello_world!(),
                    r#"",this_user());
}"#
                ),
            )],
        }],
    };
