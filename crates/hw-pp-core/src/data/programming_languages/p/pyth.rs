//! Hello-world snippet for `ProgrammingLanguageName::Pyth`.
//!
//! Imported from leachim6/hello-world (`p/Pyth.pyth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pyth.pyth>
//! - <https://esolangs.org/wiki/Pyth> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Pyth => "Pyth",
    versions: [Pyth => "Pyth"],
    extensions: [Pyth => "pyth"],
}

const PYTH: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Pyth,
        extensions: &[FileExtension::Pyth],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Pyth,
            syntax: PYTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PYTH,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
