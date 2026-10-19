//! Hello-world snippet for `ProgrammingLanguageName::Moonscript`.
//!
//! Imported from leachim6/hello-world (`m/Moonscript.moon` at `a152253`),
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
    language: Moonscript => "Moonscript",
    versions: [Moonscript => "Moonscript"],
    extensions: [Moon => "moon"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Moonscript,
        extensions: &[FileExtension::Moon],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Moonscript,
            syntax: concat!(r"print '", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"print '", output_for_hello_world!(), r"'"),
            )],
        }],
    };
