//! Hello-world snippet for `ProgrammingLanguageName::False`.
//!
//! Imported from leachim6/hello-world (`f/False.f` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/False.f>
//! - <https://esolangs.org/wiki/FALSE> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::StackBased;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: False => "False",
    versions: [False => "False"],
    extensions: [],
}

const FALSE: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#"
""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::False,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[StackBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::False,
            syntax: FALSE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FALSE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#"
""#
                        ),
                    )],
                )
            },
        }],
    };
