//! Hello-world snippet for `ProgrammingLanguageName::Hollow`.
//!
//! Imported from leachim6/hello-world (`h/Hollow.hollow` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Hollow.hollow>
//! - <https://esolangs.org/wiki/Hollow> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Hollow => "Hollow",
    versions: [Hollow => "Hollow"],
    extensions: [Hollow => "hollow"],
}

const HOLLOW: &str = concat!(r"{", output_for_hello_world!(), r".9}:?");

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Hollow,
        extensions: &[FileExtension::Hollow],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Hollow,
            syntax: HOLLOW,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HOLLOW,
                    [(
                        TokenKind::Plain,
                        concat!(r"{", output_for_hello_world!(), r".9}:?"),
                    )],
                )
            },
        }],
    };
