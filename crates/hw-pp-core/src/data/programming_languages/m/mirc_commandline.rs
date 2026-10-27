//! Hello-world snippet for `ProgrammingLanguageName::MircCommandline`.
//!
//! Imported from the Hello World Collection (`files/mIRC-Commandline.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/mIRC-Commandline.txt>
//! - <http://helloworldcollection.de/#mIRC-Commandline>
//! - <https://en.wikipedia.org/wiki/MIRC_scripting_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: MircCommandline => "mIRC-Commandline",
    versions: [MircCommandline => "mIRC-Commandline"],
    extensions: [],
}

const MIRC_COMMANDLINE: &str = concat!(
    r"; ",
    output_for_hello_world!(),
    r" for mIRC (command line version)

echo ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MircCommandline,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MircCommandline,
            syntax: MIRC_COMMANDLINE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIRC_COMMANDLINE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"; ",
                            output_for_hello_world!(),
                            r" for mIRC (command line version)

echo ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
