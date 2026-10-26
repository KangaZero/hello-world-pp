//! Hello-world snippet for `ProgrammingLanguageName::Beanshell`.
//!
//! Imported from leachim6/hello-world (`b/Beanshell.bsh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Beanshell.bsh>
//! - <https://www.wikidata.org/wiki/Q812964> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Beanshell => "Beanshell",
    versions: [Beanshell => "Beanshell"],
    extensions: [Bsh => "bsh"],
}

const BEANSHELL: &str = concat!(r#"print (""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Beanshell,
        extensions: &[FileExtension::Bsh],
        categories: &[Scripting],
        families: &[Java],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Beanshell,
            syntax: BEANSHELL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BEANSHELL,
                    [
                        (TokenKind::Function, r"print"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
