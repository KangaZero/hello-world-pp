//! Hello-world snippet for `ProgrammingLanguageName::ActionBang`.
//!
//! Imported from the Hello World Collection (`files/Action!.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Action%21.txt>
//! - <http://helloworldcollection.de/#Action!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ActionBang => "Action!",
    versions: [ActionBang => "Action!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ActionBang,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ActionBang,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in Action! programming language for the Atari 8-Bit computers

PROC Main()
 PrintE(""#,
                output_for_hello_world!(),
                r#"")
RETURN"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in Action! programming language for the Atari 8-Bit computers

PROC Main()
 PrintE(""#,
                    output_for_hello_world!(),
                    r#"")
RETURN"#
                ),
            )],
        }],
    };
