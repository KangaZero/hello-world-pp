//! Hello-world snippet for `ProgrammingLanguageName::Gecho`.
//!
//! Imported from leachim6/hello-world (`g/Gecho.gecho` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Gecho => "Gecho",
    versions: [Gecho => "Gecho"],
    extensions: [Gecho => "gecho"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gecho,
        extensions: &[FileExtension::Gecho],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gecho,
            syntax: r"print 'hello '<> 'world",
            tokenized_syntax: &[(TokenKind::Plain, r"print 'hello '<> 'world")],
        }],
    };
