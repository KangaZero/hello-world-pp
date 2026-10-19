//! Hello-world snippet for `ProgrammingLanguageName::Icedcoffeescript`.
//!
//! Imported from leachim6/hello-world (`i/IcedCoffeeScript.ics` at `a152253`),
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
    language: Icedcoffeescript => "IcedCoffeeScript",
    versions: [Icedcoffeescript => "IcedCoffeeScript"],
    extensions: [Ics => "ics"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Icedcoffeescript,
        extensions: &[FileExtension::Ics],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Icedcoffeescript,
            syntax: concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
