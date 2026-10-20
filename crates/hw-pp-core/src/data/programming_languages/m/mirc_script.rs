//! Hello-world snippet for `ProgrammingLanguageName::MircScript`.
//!
//! Imported from the Hello World Collection (`files/mIRC-Script.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/mIRC-Script.txt>
//! - <http://helloworldcollection.de/#mIRC-Script>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MircScript => "mIRC-Script",
    versions: [MircScript => "mIRC-Script"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MircScript,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MircScript,
            syntax: concat!(
                r";",
                output_for_hello_world!(),
                r" for mIRC script

alias helloworld {
  /echo -a ",
                output_for_hello_world!(),
                r"
}"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";",
                    output_for_hello_world!(),
                    r" for mIRC script

alias helloworld {
  /echo -a ",
                    output_for_hello_world!(),
                    r"
}"
                ),
            )],
        }],
    };
