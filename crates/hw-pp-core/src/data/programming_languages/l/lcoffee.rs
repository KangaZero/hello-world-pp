//! Hello-world snippet for `ProgrammingLanguageName::Lcoffee`.
//!
//! Imported from leachim6/hello-world (`l/LCoffee.lcoff` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LCoffee.lcoff>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lcoffee => "LCoffee",
    versions: [Lcoffee => "LCoffee"],
    extensions: [Lcoff => "lcoff"],
}

const LCOFFEE: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lcoffee,
        extensions: &[FileExtension::Lcoff],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lcoffee,
            syntax: LCOFFEE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LCOFFEE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
