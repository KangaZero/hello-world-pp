//! Hello-world snippet for `ProgrammingLanguageName::Millfork`.
//!
//! Imported from leachim6/hello-world (`m/Millfork.mfk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Millfork.mfk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Millfork => "Millfork",
    versions: [Millfork => "Millfork"],
    extensions: [Mfk => "mfk"],
}

const MILLFORK: &str = concat!(
    r#"import stdio

void main() {
    putstrz(""#,
    output_for_hello_world!(),
    r#""z)
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Millfork,
        extensions: &[FileExtension::Mfk],
        categories: &[Compiled],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Millfork,
            syntax: MILLFORK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MILLFORK,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import stdio

void main() {
    putstrz(""#,
                            output_for_hello_world!(),
                            r#""z)
}"#
                        ),
                    )],
                )
            },
        }],
    };
