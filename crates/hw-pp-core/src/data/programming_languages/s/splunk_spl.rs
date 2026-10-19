//! Hello-world snippet for `ProgrammingLanguageName::SplunkSpl`.
//!
//! Imported from the Hello World Collection (`files/Splunk&nbsp;SPL.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Splunk%26nbsp%3BSPL.htm>
//! - <http://helloworldcollection.de/#Splunk%C2%A0SPL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SplunkSpl => "Splunk SPL",
    versions: [SplunkSpl => "Splunk SPL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SplunkSpl,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SplunkSpl,
            syntax: concat!(
                r#"| makeresults `comment(""#,
                output_for_hello_world!(),
                r#" in Splunk SPL")` | eval mystring=""#,
                output_for_hello_world!(),
                r#"""#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"| makeresults `comment(""#,
                    output_for_hello_world!(),
                    r#" in Splunk SPL")` | eval mystring=""#,
                    output_for_hello_world!(),
                    r#"""#
                ),
            )],
        }],
    };
