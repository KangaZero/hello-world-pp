//! Hello-world snippet for `ProgrammingLanguageName::RpgIv`.
//!
//! Imported from leachim6/hello-world (`r/RPG IV.rpgle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RPG%20IV.rpgle>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: RpgIv => "RPG IV",
    versions: [RpgIv => "RPG IV"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RpgIv,
        extensions: &[],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::RpgIv,
            syntax: concat!(
                r"/free
    dsply '",
                output_for_hello_world!(),
                r"';
    return;
/end-free"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"/free
    dsply '",
                    output_for_hello_world!(),
                    r"';
    return;
/end-free"
                ),
            )],
        }],
    };
