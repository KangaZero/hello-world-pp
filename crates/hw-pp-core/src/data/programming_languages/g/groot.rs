//! Hello-world snippet for `ProgrammingLanguageName::Groot`.
//!
//! Imported from leachim6/hello-world (`g/groot.groot` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Groot => "groot",
    versions: [Groot => "groot"],
    extensions: [Groot => "groot"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Groot,
        extensions: &[FileExtension::Groot],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Groot,
            syntax: r"I AM GROOT
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
            tokenized_syntax: &[(
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
        }],
    };
