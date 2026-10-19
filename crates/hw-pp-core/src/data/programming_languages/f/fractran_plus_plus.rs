//! Hello-world snippet for `ProgrammingLanguageName::FractranPlusPlus`.
//!
//! Imported from leachim6/hello-world (`f/Fractran++.fpp` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: FractranPlusPlus => "Fractran++",
    versions: [FractranPlusPlus => "Fractran++"],
    extensions: [Fpp => "fpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::FractranPlusPlus,
        extensions: &[FileExtension::Fpp],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::FractranPlusPlus,
            syntax: r"3,-1/2,(2*37)/3,0/0,<71 101 108 108 111 32 87 111 114 108 100 0>/37,4/0",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"3,-1/2,(2*37)/3,0/0,<71 101 108 108 111 32 87 111 114 108 100 0>/37,4/0",
            )],
        }],
    };
