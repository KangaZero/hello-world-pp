//! Hello-world snippet for `ProgrammingLanguageName::SAlgol`.
//!
//! Imported from leachim6/hello-world (`s/S Algol` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/S%20Algol>
//! - <https://en.wikipedia.org/wiki/S-algol> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: SAlgol => "S Algol",
    versions: [SAlgol => "S Algol"],
    extensions: [],
}

const S_ALGOL: &str = concat!(
    r#"write ""#,
    output_for_hello_world!(),
    r#""
?"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SAlgol,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SAlgol,
            syntax: S_ALGOL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    S_ALGOL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"write ""#,
                            output_for_hello_world!(),
                            r#""
?"#
                        ),
                    )],
                )
            },
        }],
    };
