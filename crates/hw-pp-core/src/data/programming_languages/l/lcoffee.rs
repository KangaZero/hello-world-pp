//! Hello-world snippet for `ProgrammingLanguageName::Lcoffee`.
//!
//! Imported from leachim6/hello-world (`l/LCoffee.lcoff` at `a152253`),
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
    language: Lcoffee => "LCoffee",
    versions: [Lcoffee => "LCoffee"],
    extensions: [Lcoff => "lcoff"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lcoffee,
        extensions: &[FileExtension::Lcoff],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lcoffee,
            syntax: concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
