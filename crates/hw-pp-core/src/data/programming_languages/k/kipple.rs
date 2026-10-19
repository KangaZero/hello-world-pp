//! Hello-world snippet for `ProgrammingLanguageName::Kipple`.
//!
//! Imported from leachim6/hello-world (`k/Kipple.kipple` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Kipple => "Kipple",
    versions: [Kipple => "Kipple"],
    extensions: [Kipple => "kipple"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kipple,
        extensions: &[FileExtension::Kipple],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kipple,
            syntax: r"33>o<100 108>o<114 111>o<87 32>o<111 108>o<108 101>o<72",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"33>o<100 108>o<114 111>o<87 32>o<111 108>o<108 101>o<72",
            )],
        }],
    };
