//! Hello-world snippet for `ProgrammingLanguageName::Groot`.
//!
//! Imported from leachim6/hello-world (`g/groot.groot` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/groot.groot>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Groot => "groot",
    versions: [Groot => "groot"],
    extensions: [Groot => "groot"],
}

const GROOT: &str = r"I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I AM GROOOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
i am groot
I AM GROOOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOOT
I AM GROOOT
i am groot
i am groot
i am groot
I AM GROOOT
I AM GROOT
I AM GROOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I AM GROOOT
I AM GROOT
I AM GROOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I am groot
I am groot
I am groot
I am groot
I AM GROOOT
i am groot
i am groot
i am groot
I AM GROOOT
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I AM GROOT
I AM GROOT
i am groot
I AM GROOOT";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groot,
        extensions: &[FileExtension::Groot],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Groot,
            syntax: GROOT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GROOT,
                    [(
                        TokenKind::Plain,
                        r"I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I AM GROOOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
i am groot
I AM GROOOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOOT
I AM GROOOT
i am groot
i am groot
i am groot
I AM GROOOT
I AM GROOT
I AM GROOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I AM GROOOT
I AM GROOT
I AM GROOT
I AM GROOT
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I'm Groot
I am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
i am groot
I AM GROOT
I am Groot
We are Groot
I am groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I am groot
I am groot
I am groot
I am groot
I AM GROOOT
i am groot
i am groot
i am groot
I AM GROOOT
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I am Groot
I AM GROOOT
I AM GROOT
I AM GROOT
i am groot
I AM GROOOT",
                    )],
                )
            },
        }],
    };
