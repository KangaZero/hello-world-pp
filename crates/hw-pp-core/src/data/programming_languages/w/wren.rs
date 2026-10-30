//! Hello-world snippet for `ProgrammingLanguageName::Wren`.
//!
//! Imported from leachim6/hello-world (`w/Wren.wren` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/w/Wren.wren>
//! - <https://www.wikidata.org/wiki/Q99329231> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Wren => "Wren",
    versions: [Wren => "Wren"],
    extensions: [Wren => "wren"],
}

const WREN: &str = concat!(r#"IO.print(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Wren,
        extensions: &[FileExtension::Wren],
        categories: &[Scripting],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Wren,
            syntax: WREN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    WREN,
                    [
                        (TokenKind::Plain, r"IO"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"print"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r")"),
                    ],
                )
            },
        }],
    };
