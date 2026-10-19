//! Hello-world snippet for `ProgrammingLanguageName::Dafny`.
//!
//! Imported from the Hello World Collection (`files/Dafny.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Dafny.htm>
//! - <http://helloworldcollection.de/#Dafny>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Dafny => "Dafny",
    versions: [Dafny => "Dafny"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dafny,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dafny,
            syntax: concat!(
                r"// ",
                output_for_hello_world!(),
                r#" in Dafny

method Main() {
  print ""#,
                output_for_hello_world!(),
                r#"\n";
}"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Dafny

method Main() {
  print ""#,
                    output_for_hello_world!(),
                    r#"\n";
}"#
                ),
            )],
        }],
    };
