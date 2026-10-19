//! Hello-world snippet for `ProgrammingLanguageName::Mirc`.
//!
//! Imported from leachim6/hello-world (`m/Mirc.mrc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Mirc => "Mirc",
    versions: [Mirc => "Mirc"],
    extensions: [Mrc => "mrc"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mirc,
        extensions: &[FileExtension::Mrc],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mirc,
            syntax: concat!(r"echo -a ", output_for_hello_world!()),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"echo -a ", output_for_hello_world!()),
            )],
        }],
    };
