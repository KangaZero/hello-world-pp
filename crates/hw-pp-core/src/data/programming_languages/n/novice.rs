//! Hello-world snippet for `ProgrammingLanguageName::Novice`.
//!
//! Imported from leachim6/hello-world (`n/Novice.novice` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Novice.novice>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Novice => "Novice",
    versions: [Novice => "Novice"],
    extensions: [Novice => "novice"],
}

const NOVICE: &str = concat!(
    r"a
a-",
    output_for_hello_world!(),
    r"_*
",
    output_for_hello_world!(),
    r"_*"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Novice,
        extensions: &[FileExtension::Novice],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Novice,
            syntax: NOVICE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NOVICE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"a
a-",
                            output_for_hello_world!(),
                            r"_*
",
                            output_for_hello_world!(),
                            r"_*"
                        ),
                    )],
                )
            },
        }],
    };
