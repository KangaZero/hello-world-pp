//! Hello-world snippet for `ProgrammingLanguageName::Netrexx`.
//!
//! Imported from leachim6/hello-world (`n/NetRexx.nrx` at `a152253`),
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
    language: Netrexx => "NetRexx",
    versions: [Netrexx => "NetRexx"],
    extensions: [Nrx => "nrx"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Netrexx,
        extensions: &[FileExtension::Nrx],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Netrexx,
            syntax: concat!(r"say '", output_for_hello_world!(), r"'"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"say '", output_for_hello_world!(), r"'"),
            )],
        }],
    };
