//! Hello-world snippet for `ProgrammingLanguageName::Acidic`.
//!
//! Imported from leachim6/hello-world (`a/ACIDIC.acidic` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ACIDIC.acidic>
//! - <https://esolangs.org/wiki/ACIDIC> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Acidic => "ACIDIC",
    versions: [Acidic => "ACIDIC"],
    extensions: [Acidic => "acidic"],
}

const ACIDIC: &str = concat!(
    output_for_hello_world!(),
    r"
wP"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Acidic,
        extensions: &[FileExtension::Acidic],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Acidic,
            syntax: ACIDIC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ACIDIC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            output_for_hello_world!(),
                            r"
wP"
                        ),
                    )],
                )
            },
        }],
    };
