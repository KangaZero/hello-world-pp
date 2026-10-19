//! Hello-world snippet for `ProgrammingLanguageName::Icedcoffeescript`.
//!
//! Imported from leachim6/hello-world (`i/IcedCoffeeScript.ics` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/IcedCoffeeScript.ics>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
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
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Icedcoffeescript,
            syntax: concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#),
            )],
        }],
    };
