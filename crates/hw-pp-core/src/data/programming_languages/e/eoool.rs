//! Hello-world snippet for `ProgrammingLanguageName::Eoool`.
//!
//! Imported from leachim6/hello-world (`e/EOOOL.eoool` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EOOOL.eoool>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Eoool => "EOOOL",
    versions: [Eoool => "EOOOL"],
    extensions: [Eoool => "eoool"],
}

const EOOOL: &str = r",{,{89+(59+~1&(7~+1&2&2&((3~+1&(4~(32_((6~+((8+(38_(},}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eoool,
        extensions: &[FileExtension::Eoool],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eoool,
            syntax: EOOOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EOOOL,
                    [(
                        TokenKind::Plain,
                        r",{,{89+(59+~1&(7~+1&2&2&((3~+1&(4~(32_((6~+((8+(38_(},}",
                    )],
                )
            },
        }],
    };
