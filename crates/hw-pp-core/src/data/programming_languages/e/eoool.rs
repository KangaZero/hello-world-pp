//! Hello-world snippet for `ProgrammingLanguageName::Eoool`.
//!
//! Imported from leachim6/hello-world (`e/EOOOL.eoool` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Eoool => "EOOOL",
    versions: [Eoool => "EOOOL"],
    extensions: [Eoool => "eoool"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eoool,
        extensions: &[FileExtension::Eoool],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eoool,
            syntax: r",{,{89+(59+~1&(7~+1&2&2&((3~+1&(4~(32_((6~+((8+(38_(},}",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r",{,{89+(59+~1&(7~+1&2&2&((3~+1&(4~(32_((6~+((8+(38_(},}",
            )],
        }],
    };
