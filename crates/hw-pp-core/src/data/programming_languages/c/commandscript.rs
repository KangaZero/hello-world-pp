//! Hello-world snippet for `ProgrammingLanguageName::Commandscript`.
//!
//! Imported from the Hello World Collection (`files/CommandScript.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/CommandScript.txt>
//! - <http://helloworldcollection.de/#CommandScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Commandscript => "CommandScript",
    versions: [Commandscript => "CommandScript"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Commandscript,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Commandscript,
            syntax: concat!(
                r"#",
                output_for_hello_world!(),
                r#" in Command Script 3.1
#Meta.Name: ""#,
                output_for_hello_world!(),
                r#""

#Block(Main).Start
    echo ""#,
                output_for_hello_world!(),
                r#""
#Block(Main).End"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"#",
                    output_for_hello_world!(),
                    r#" in Command Script 3.1
#Meta.Name: ""#,
                    output_for_hello_world!(),
                    r#""

#Block(Main).Start
    echo ""#,
                    output_for_hello_world!(),
                    r#""
#Block(Main).End"#
                ),
            )],
        }],
    };
