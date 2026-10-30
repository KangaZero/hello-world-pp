//! Hello-world snippet for `ProgrammingLanguageName::Cat`.
//!
//! Imported from leachim6/hello-world (`c/cat.cat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/cat.cat>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cat => "cat",
    versions: [Cat => "cat"],
    extensions: [Cat => "cat"],
}

const CAT: &str = output_for_hello_world!();

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cat,
        extensions: &[FileExtension::Cat],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cat,
            syntax: CAT,
            tokenized_syntax: &const {
                checked_token_lengths(CAT, [(TokenKind::Plain, output_for_hello_world!())])
            },
        }],
    };
