//! Hello-world snippet for `ProgrammingLanguageName::Livescript`.
//!
//! Imported from leachim6/hello-world (`l/LiveScript.ls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/l/LiveScript.ls>
//! - <https://www.wikidata.org/wiki/Q16963446> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Livescript => "LiveScript",
    versions: [Livescript => "LiveScript"],
    extensions: [Ls => "ls"],
}

const LIVESCRIPT: &str = concat!(r#"console.log ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Livescript,
        extensions: &[FileExtension::Ls],
        categories: &[Scripting],
        families: &[JavaScript],
        paradigms: &[ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Livescript,
            syntax: LIVESCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LIVESCRIPT,
                    [
                        (TokenKind::Plain, r"console"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"log"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                    ],
                )
            },
        }],
    };
