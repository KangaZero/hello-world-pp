//! Hello-world snippet for `ProgrammingLanguageName::Fob`.
//!
//! Imported from leachim6/hello-world (`f/Fob.fob` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Fob.fob>
//! - <https://esolangs.org/wiki/Fob> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fob => "Fob",
    versions: [Fob => "Fob"],
    extensions: [Fob => "fob"],
}

const FOB: &str = concat!(r" $", output_for_hello_world!(), r"#<>");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fob,
        extensions: &[FileExtension::Fob],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fob,
            syntax: FOB,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FOB,
                    [(
                        TokenKind::Plain,
                        concat!(r" $", output_for_hello_world!(), r"#<>"),
                    )],
                )
            },
        }],
    };
