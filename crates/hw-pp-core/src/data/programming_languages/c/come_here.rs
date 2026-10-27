//! Hello-world snippet for `ProgrammingLanguageName::ComeHere`.
//!
//! Imported from leachim6/hello-world (`c/Come Here` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Come%20Here>
//! - <https://esolangs.org/wiki/Come_Here> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Imperative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: ComeHere => "Come Here",
    versions: [ComeHere => "Come Here"],
    extensions: [],
}

const COME_HERE: &str = concat!(r#"TELL ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ComeHere,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ComeHere,
            syntax: COME_HERE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    COME_HERE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"TELL ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
