//! Hello-world snippet for `ProgrammingLanguageName::E62qpodb593`.
//!
//! Imported from leachim6/hello-world (`e/e62qpodb593` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: E62qpodb593 => "e62qpodb593",
    versions: [E62qpodb593 => "e62qpodb593"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::E62qpodb593,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::E62qpodb593,
            syntax: r"tqqqyoyyyyyypoyqqooqqqoztkkkkkkppotyyyyyppo2oqqqokpokkqqo",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"tqqqyoyyyyyypoyqqooqqqoztkkkkkkppotyyyyyppo2oqqqokpokkqqo",
            )],
        }],
    };
