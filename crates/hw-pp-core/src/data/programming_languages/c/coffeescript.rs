//! Hello-world snippet for `ProgrammingLanguageName::Coffeescript`.
//!
//! Imported from leachim6/hello-world (`c/CoffeeScript.coffee` at `a152253`),
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
    language: Coffeescript => "CoffeeScript",
    versions: [Coffeescript => "CoffeeScript"],
    extensions: [Coffee => "coffee"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coffeescript,
        extensions: &[FileExtension::Coffee],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coffeescript,
            syntax: concat!(r#"alert ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"alert ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
