//! Hello-world snippet for `ProgrammingLanguageName::Grin`.
//!
//! Imported from leachim6/hello-world (`g/Grin.grin` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Grin.grin>
//! - <https://esolangs.org/wiki/Grin> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Grin => "Grin",
    versions: [Grin => "Grin"],
    extensions: [Grin => "grin"],
}

const GRIN: &str = concat!(r"(", output_for_hello_world!(), r")");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Grin,
        extensions: &[FileExtension::Grin],
        categories: &[Esolang],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Grin,
            syntax: GRIN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GRIN,
                    [(
                        TokenKind::Plain,
                        concat!(r"(", output_for_hello_world!(), r")"),
                    )],
                )
            },
        }],
    };
