//! Hello-world snippet for `ProgrammingLanguageName::Dongbei`.
//!
//! Imported from leachim6/hello-world (`d/dongbei.dongbei` at `a152253`),
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
    language: Dongbei => "dongbei",
    versions: [Dongbei => "dongbei"],
    extensions: [Dongbei => "dongbei"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dongbei,
        extensions: &[FileExtension::Dongbei],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dongbei,
            syntax: concat!(r"唠唠：“", output_for_hello_world!(), r"”。 "),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r"唠唠：“", output_for_hello_world!(), r"”。 "),
            )],
        }],
    };
