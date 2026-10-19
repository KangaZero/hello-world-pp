//! Hello-world snippet for `ProgrammingLanguageName::Logtalk`.
//!
//! Imported from leachim6/hello-world (`l/Logtalk.lgt` at `a152253`),
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
    language: Logtalk => "Logtalk",
    versions: [Logtalk => "Logtalk"],
    extensions: [Lgt => "lgt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Logtalk,
        extensions: &[FileExtension::Lgt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Logtalk,
            syntax: concat!(r"write('", output_for_hello_world!(), r"')"),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"write('", output_for_hello_world!(), r"')"),
            )],
        }],
    };
