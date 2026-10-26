//! Hello-world snippet for `ProgrammingLanguageName::Yaskc`.
//!
//! Imported from leachim6/hello-world (`y/YASKC.yaskc` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/y/YASKC.yaskc>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Yaskc => "YASKC",
    versions: [Yaskc => "YASKC"],
    extensions: [Yaskc => "yaskc"],
}

const YASKC: &str = concat!(
    r"using universal delay 200
using loop False

typewrite ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Yaskc,
        extensions: &[FileExtension::Yaskc],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Yaskc,
            syntax: YASKC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    YASKC,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"using universal delay 200
using loop False

typewrite ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
