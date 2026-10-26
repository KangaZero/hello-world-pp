//! Hello-world snippet for `ProgrammingLanguageName::ScratchBlocks`.
//!
//! Imported from leachim6/hello-world (`s/Scratch Blocks` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Scratch%20Blocks>
//! - <https://en.wikipedia.org/wiki/Scratch_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Visual};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Educational, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ScratchBlocks => "Scratch Blocks",
    versions: [ScratchBlocks => "Scratch Blocks"],
    extensions: [],
}

const SCRATCH_BLOCKS: &str = concat!(
    r"when flag clicked
say [",
    output_for_hello_world!(),
    r"]"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ScratchBlocks,
        extensions: &[],
        categories: &[Educational],
        families: &[],
        paradigms: &[EventDriven, Visual],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ScratchBlocks,
            syntax: SCRATCH_BLOCKS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SCRATCH_BLOCKS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"when flag clicked
say [",
                            output_for_hello_world!(),
                            r"]"
                        ),
                    )],
                )
            },
        }],
    };
