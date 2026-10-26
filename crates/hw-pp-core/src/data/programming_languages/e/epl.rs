//! Hello-world snippet for `ProgrammingLanguageName::Epl`.
//!
//! Imported from leachim6/hello-world (`e/EPL.e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/e/EPL.e>
//! - <https://www.wikidata.org/wiki/Q11088328> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Epl => "EPL",
    versions: [Epl => "EPL"],
    extensions: [],
}

const EPL: &str = concat!(r#"调试输出(""#, output_for_hello_world!(), r#"")"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Epl,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Procedural, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Epl,
            syntax: EPL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EPL,
                    [(
                        TokenKind::Plain,
                        concat!(r#"调试输出(""#, output_for_hello_world!(), r#"")"#),
                    )],
                )
            },
        }],
    };
