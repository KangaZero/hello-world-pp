//! Hello-world snippet for `ProgrammingLanguageName::Huf`.
//!
//! Imported from leachim6/hello-world (`h/Huf.huf` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/Huf.huf>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Huf => "Huf",
    versions: [Huf => "Huf"],
    extensions: [Huf => "huf"],
}

const HUF: &str = r"++++++++++|+++++++!++>
++++++++++|++++++++++!+>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!++++++++>
++++++++++|+++++++++++!+>
++++++|+++++!++>
++++++++++|++++++++!+++++++>
++++++++++|+++++++++++!+>
++++++++++|+++++++++++!++++>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!>";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Huf,
        extensions: &[FileExtension::Huf],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Huf,
            syntax: HUF,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HUF,
                    [(
                        TokenKind::Plain,
                        r"++++++++++|+++++++!++>
++++++++++|++++++++++!+>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!++++++++>
++++++++++|+++++++++++!+>
++++++|+++++!++>
++++++++++|++++++++!+++++++>
++++++++++|+++++++++++!+>
++++++++++|+++++++++++!++++>
++++++++++|++++++++++!++++++++>
++++++++++|++++++++++!>",
                    )],
                )
            },
        }],
    };
