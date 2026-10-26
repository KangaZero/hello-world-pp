//! Hello-world snippet for `ProgrammingLanguageName::Coffeescript`.
//!
//! Imported from leachim6/hello-world (`c/CoffeeScript.coffee` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CoffeeScript.coffee>
//! - <https://www.wikidata.org/wiki/Q1106819> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    AspectOriented, Functional, Imperative, PrototypeBased,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Coffeescript => "CoffeeScript",
    versions: [Coffeescript => "CoffeeScript"],
    extensions: [Coffee => "coffee"],
}

const COFFEESCRIPT: &str = concat!(r#"alert ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Coffeescript,
        extensions: &[FileExtension::Coffee],
        categories: &[Scripting],
        families: &[JavaScript],
        paradigms: &[Imperative, PrototypeBased, Functional, AspectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Coffeescript,
            syntax: COFFEESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COFFEESCRIPT,
                    [
                        (TokenKind::Plain, r"alert "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
