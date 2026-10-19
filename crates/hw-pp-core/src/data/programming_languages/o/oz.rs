//! Hello-world snippet for `ProgrammingLanguageName::Oz`.
//!
//! Imported from the Hello World Collection (`files/Oz.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Oz.htm>
//! - <http://helloworldcollection.de/#Oz>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Oz => "Oz",
    versions: [Oz => "Oz"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Oz,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Oz,
            syntax: concat!(
                r"% ",
                output_for_hello_world!(),
                r#" in Oz

functor
import
  System
  Application
define
  {System.showInfo ""#,
                output_for_hello_world!(),
                r#""}
  {Application.exit 0}
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"% ",
                    output_for_hello_world!(),
                    r#" in Oz

functor
import
  System
  Application
define
  {System.showInfo ""#,
                    output_for_hello_world!(),
                    r#""}
  {Application.exit 0}
end"#
                ),
            )],
        }],
    };
