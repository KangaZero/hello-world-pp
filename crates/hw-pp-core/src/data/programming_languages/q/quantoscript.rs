//! Hello-world snippet for `ProgrammingLanguageName::Quantoscript`.
//!
//! Imported from leachim6/hello-world (PR #1832 `q/QuantoScript.qs`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1832>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Quantoscript => "QuantoScript",
    versions: [Quantoscript => "QuantoScript"],
    extensions: [],
}

const QUANTOSCRIPT: &str = concat!(
    r#"print(""#,
    output_for_hello_world!(),
    r#"")
print("درود جهان")"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Quantoscript,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Quantoscript,
            syntax: QUANTOSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    QUANTOSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"print(""#,
                            output_for_hello_world!(),
                            r#"")
print("درود جهان")"#
                        ),
                    )],
                )
            },
        }],
    };
