//! Hello-world snippet for `ProgrammingLanguageName::Ici`.
//!
//! Imported from leachim6/hello-world (`i/ICI.ici` at `a152253`),
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
    language: Ici => "ICI",
    versions: [Ici => "ICI"],
    extensions: [Ici => "ici"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ici,
        extensions: &[FileExtension::Ici],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ici,
            syntax: concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"printf(""#, output_for_hello_world!(), r#"\n");"#),
            )],
        }],
    };
