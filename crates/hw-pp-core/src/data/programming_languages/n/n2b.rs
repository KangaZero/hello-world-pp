//! Hello-world snippet for `ProgrammingLanguageName::N2b`.
//!
//! Imported from leachim6/hello-world (`#/2B.2b` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: N2b => "2B",
    versions: [N2b => "2B"],
    extensions: [E2b => "2b"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N2b,
        extensions: &[FileExtension::E2b],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N2b,
            syntax: r"+0+0+0+0+0+0+0+2)+0+0+9)+7))+3)-0-0-0-0-0-0-0-9)+0+0+0+0+0+0+0+0+7)-8)+3)-6)-8)-7-0-0-0-0-0-0)",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"+0+0+0+0+0+0+0+2)+0+0+9)+7))+3)-0-0-0-0-0-0-0-9)+0+0+0+0+0+0+0+0+7)-8)+3)-6)-8)-7-0-0-0-0-0-0)",
            )],
        }],
    };
