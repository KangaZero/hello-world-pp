//! Hello-world snippet for `ProgrammingLanguageName::Sml`.
//!
//! Imported from leachim6/hello-world (`s/SML.sml` at `a152253`),
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
    language: Sml => "SML",
    versions: [Sml => "SML"],
    extensions: [Sml => "sml"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sml,
        extensions: &[FileExtension::Sml],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sml,
            syntax: concat!(r#"val () = print ""#, output_for_hello_world!(), r#"\n""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"val () = print ""#, output_for_hello_world!(), r#"\n""#),
            )],
        }],
    };
