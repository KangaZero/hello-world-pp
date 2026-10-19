//! Hello-world snippet for `ProgrammingLanguageName::MircCommandline`.
//!
//! Imported from the Hello World Collection (`files/mIRC-Commandline.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/mIRC-Commandline.txt>
//! - <http://helloworldcollection.de/#mIRC-Commandline>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MircCommandline => "mIRC-Commandline",
    versions: [MircCommandline => "mIRC-Commandline"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MircCommandline,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MircCommandline,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r" for mIRC (command line version)

echo ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r" for mIRC (command line version)

echo ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
