//! Hello-world snippet for `ProgrammingLanguageName::TransDimensional`.
//!
//! Imported from leachim6/hello-world (`t/Trans-dimensional` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/Trans-dimensional>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: TransDimensional => "Trans-dimensional",
    versions: [TransDimensional => "Trans-dimensional"],
    extensions: [],
}

const TRANS_DIMENSIONAL: &str = concat!(
    r#"dimension X;
put [X:1] -> ""#,
    output_for_hello_world!(),
    r#"";
output get [X:1];"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TransDimensional,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TransDimensional,
            syntax: TRANS_DIMENSIONAL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TRANS_DIMENSIONAL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"dimension X;
put [X:1] -> ""#,
                            output_for_hello_world!(),
                            r#"";
output get [X:1];"#
                        ),
                    )],
                )
            },
        }],
    };
