//! Hello-world snippet for `ProgrammingLanguageName::AmazonStatesLanguage`.
//!
//! Imported from the Hello World Collection (`files/Amazon&nbsp;States&nbsp;Language.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Amazon%26nbsp%3BStates%26nbsp%3BLanguage.htm>
//! - <http://helloworldcollection.de/#Amazon%C2%A0States%C2%A0Language>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AmazonStatesLanguage => "Amazon States Language",
    versions: [AmazonStatesLanguage => "Amazon States Language"],
    extensions: [],
}

const AMAZON_STATES_LANGUAGE: &str = concat!(
    r#"{
    "Comment": ""#,
    output_for_hello_world!(),
    r#" in Amazon States Language",
    "StartAt": ""#,
    output_for_hello_world!(),
    r#"",
    "States": {
    ""#,
    output_for_hello_world!(),
    r#"": { 
      "Type": "Task",
      "Resource": "arn:aws:lambda:us-east-1:123456789012:function:HelloWorld",
      "End": true
    }
  }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AmazonStatesLanguage,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AmazonStatesLanguage,
            syntax: AMAZON_STATES_LANGUAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMAZON_STATES_LANGUAGE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"{
    "Comment": ""#,
                            output_for_hello_world!(),
                            r#" in Amazon States Language",
    "StartAt": ""#,
                            output_for_hello_world!(),
                            r#"",
    "States": {
    ""#,
                            output_for_hello_world!(),
                            r#"": { 
      "Type": "Task",
      "Resource": "arn:aws:lambda:us-east-1:123456789012:function:HelloWorld",
      "End": true
    }
  }
}"#
                        ),
                    )],
                )
            },
        }],
    };
