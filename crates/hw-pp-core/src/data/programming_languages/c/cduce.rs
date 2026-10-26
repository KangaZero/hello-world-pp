//! Hello-world snippet for `ProgrammingLanguageName::Cduce`.
//!
//! Imported from leachim6/hello-world (`c/Cduce.cduce` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cduce.cduce>
//! - <https://www.wikidata.org/wiki/Q2931220> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Functional;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Cduce => "Cduce",
    versions: [Cduce => "Cduce"],
    extensions: [Cduce => "cduce"],
}

const CDUCE: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"";;"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cduce,
        extensions: &[FileExtension::Cduce],
        categories: &[Compiled],
        families: &[],
        paradigms: &[Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cduce,
            syntax: CDUCE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CDUCE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"";;"#),
                    )],
                )
            },
        }],
    };
