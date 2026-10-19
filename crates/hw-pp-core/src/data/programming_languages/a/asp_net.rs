//! Hello-world snippet for `ProgrammingLanguageName::AspNet`.
//!
//! Imported from the Hello World Collection (`files/ASP.NET.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ASP.NET.htm>
//! - <http://helloworldcollection.de/#ASP.NET>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AspNet => "ASP.NET",
    versions: [AspNet => "ASP.NET"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AspNet,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AspNet,
            syntax: concat!(
                r"<!-- ",
                output_for_hello_world!(),
                r#" in ASP.NET -->
<%= ""#,
                output_for_hello_world!(),
                r#"" %>"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"<!-- ",
                    output_for_hello_world!(),
                    r#" in ASP.NET -->
<%= ""#,
                    output_for_hello_world!(),
                    r#"" %>"#
                ),
            )],
        }],
    };
