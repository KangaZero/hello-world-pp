//! Hello-world snippet for `ProgrammingLanguageName::Retina`.
//!
//! Imported from leachim6/hello-world (`r/Retina.ret` at `a152253`),
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
    language: Retina => "Retina",
    versions: [Retina => "Retina"],
    extensions: [Ret => "ret"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Retina,
        extensions: &[FileExtension::Ret],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Retina,
            syntax: concat!(r"K`", output_for_hello_world!()),
            tokenized_syntax: &[(TokenKind::Plain, concat!(r"K`", output_for_hello_world!()))],
        }],
    };
