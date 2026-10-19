//! Hello-world snippet for `ProgrammingLanguageName::RpgIv`.
//!
//! Imported from leachim6/hello-world (`r/RPG IV.rpgle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
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
        categories: &[],
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
