//! Hello-world snippet for `ProgrammingLanguageName::Gofer`.
//!
//! Imported from the Hello World Collection (`files/Gofer` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Gofer>
//! - <http://helloworldcollection.de/#Gofer>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Gofer => "Gofer",
    versions: [Gofer => "Gofer"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gofer,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gofer,
            syntax: concat!(
                r"-- ",
                output_for_hello_world!(),
                r#" in Gofer
-- Simple version

helloWorld:: String
helloWorld = ""#,
                output_for_hello_world!(),
                r#"\n"


-- "#,
                output_for_hello_world!(),
                r#" in Gofer
-- Dialog version

helloWorld :: Dialogue
helloWorld resps = [AppendChan stdout ""#,
                output_for_hello_world!(),
                r#""]"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"-- ",
                    output_for_hello_world!(),
                    r#" in Gofer
-- Simple version

helloWorld:: String
helloWorld = ""#,
                    output_for_hello_world!(),
                    r#"\n"


-- "#,
                    output_for_hello_world!(),
                    r#" in Gofer
-- Dialog version

helloWorld :: Dialogue
helloWorld resps = [AppendChan stdout ""#,
                    output_for_hello_world!(),
                    r#""]"#
                ),
            )],
        }],
    };
