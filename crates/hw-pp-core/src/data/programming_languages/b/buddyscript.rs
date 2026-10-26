//! Hello-world snippet for `ProgrammingLanguageName::Buddyscript`.
//!
//! Imported from leachim6/hello-world (`b/BuddyScript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/BuddyScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Buddyscript => "BuddyScript",
    versions: [Buddyscript => "BuddyScript"],
    extensions: [],
}

const BUDDYSCRIPT: &str = concat!(
    r"=AnythingPerfect

  - ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Buddyscript,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Buddyscript,
            syntax: BUDDYSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BUDDYSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"=AnythingPerfect

  - ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
