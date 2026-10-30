//! Hello-world snippet for `ProgrammingLanguageName::Eve`.
//!
//! Imported from the Hello World Collection (`files/Eve.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Eve.htm>
//! - <http://helloworldcollection.de/#Eve>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Eve => "Eve",
    versions: [Eve => "Eve"],
    extensions: [],
}

const EVE: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Eve

bind @browser
  [tag: "div", text: ""#,
    output_for_hello_world!(),
    r#""]"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eve,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eve,
            syntax: EVE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EVE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in Eve

bind @browser
  [tag: "div", text: ""#,
                            output_for_hello_world!(),
                            r#""]"#
                        ),
                    )],
                )
            },
        }],
    };
