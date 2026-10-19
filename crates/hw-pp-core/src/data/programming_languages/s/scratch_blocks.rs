//! Hello-world snippet for `ProgrammingLanguageName::ScratchBlocks`.
//!
//! Imported from leachim6/hello-world (`s/Scratch Blocks` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: ScratchBlocks => "Scratch Blocks",
    versions: [ScratchBlocks => "Scratch Blocks"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ScratchBlocks,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ScratchBlocks,
            syntax: concat!(
                r"when flag clicked
say [",
                output_for_hello_world!(),
                r"]"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"when flag clicked
say [",
                    output_for_hello_world!(),
                    r"]"
                ),
            )],
        }],
    };
