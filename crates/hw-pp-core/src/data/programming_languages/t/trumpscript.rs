//! Hello-world snippet for `ProgrammingLanguageName::Trumpscript`.
//!
//! Imported from leachim6/hello-world (`t/TrumpScript.py` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/t/TrumpScript.py>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Trumpscript => "TrumpScript",
    versions: [Trumpscript => "TrumpScript"],
    extensions: [],
}

const TRUMPSCRIPT: &str = concat!(
    r#"say ""#,
    output_for_hello_world!(),
    r#""!
America is great."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Trumpscript,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Trumpscript,
            syntax: TRUMPSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    TRUMPSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"say ""#,
                            output_for_hello_world!(),
                            r#""!
America is great."#
                        ),
                    )],
                )
            },
        }],
    };
