//! Hello-world snippet for `ProgrammingLanguageName::Kipple`.
//!
//! Imported from leachim6/hello-world (`k/Kipple.kipple` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kipple.kipple>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Kipple => "Kipple",
    versions: [Kipple => "Kipple"],
    extensions: [Kipple => "kipple"],
}

const KIPPLE: &str = r"33>o<100 108>o<114 111>o<87 32>o<111 108>o<108 101>o<72";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kipple,
        extensions: &[FileExtension::Kipple],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kipple,
            syntax: KIPPLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KIPPLE,
                    [(
                        TokenKind::Plain,
                        r"33>o<100 108>o<114 111>o<87 32>o<111 108>o<108 101>o<72",
                    )],
                )
            },
        }],
    };
