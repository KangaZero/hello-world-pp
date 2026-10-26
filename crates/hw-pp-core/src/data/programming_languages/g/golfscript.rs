//! Hello-world snippet for `ProgrammingLanguageName::Golfscript`.
//!
//! Imported from leachim6/hello-world (`g/Golfscript.golfscript` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/g/Golfscript.golfscript>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Golfscript => "Golfscript",
    versions: [Golfscript => "Golfscript"],
    extensions: [Golfscript => "golfscript"],
}

const GOLFSCRIPT: &str = concat!(r#"""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Golfscript,
        extensions: &[FileExtension::Golfscript],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Golfscript,
            syntax: GOLFSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOLFSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
