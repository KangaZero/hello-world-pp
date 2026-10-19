//! Hello-world snippet for `ProgrammingLanguageName::Incal`.
//!
//! Imported from leachim6/hello-world (`i/INCAL.incal` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Incal => "INCAL",
    versions: [Incal => "INCAL"],
    extensions: [Incal => "incal"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Incal,
        extensions: &[FileExtension::Incal],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Incal,
            syntax: r"send,66#111#102#<101#42#93#101#120#102#110#43#
die",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"send,66#111#102#<101#42#93#101#120#102#110#43#
die",
            )],
        }],
    };
