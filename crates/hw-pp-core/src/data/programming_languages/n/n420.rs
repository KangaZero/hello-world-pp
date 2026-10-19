//! Hello-world snippet for `ProgrammingLanguageName::N420`.
//!
//! Imported from leachim6/hello-world (`#/420.420` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: N420 => "420",
    versions: [N420 => "420"],
    extensions: [E420 => "420"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N420,
        extensions: &[FileExtension::E420],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N420,
            syntax: r"420
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it
blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"420
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it
blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it

blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it blaze it
blaze it blaze it blaze it blaze it blaze it blaze it",
            )],
        }],
    };
