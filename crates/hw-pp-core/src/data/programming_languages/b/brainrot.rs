//! Hello-world snippet for `ProgrammingLanguageName::Brainrot`.
//!
//! Imported from leachim6/hello-world (`b/Brainrot.brainrot` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Brainrot.brainrot>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Brainrot => "Brainrot",
    versions: [Brainrot => "Brainrot"],
    extensions: [Brainrot => "brainrot"],
}

const BRAINROT: &str = concat!(
    r#"skibidi main {
    yapping(""#,
    output_for_hello_world!(),
    r#"");
    bussin 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Brainrot,
        extensions: &[FileExtension::Brainrot],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Brainrot,
            syntax: BRAINROT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BRAINROT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"skibidi main {
    yapping(""#,
                            output_for_hello_world!(),
                            r#"");
    bussin 0;
}"#
                        ),
                    )],
                )
            },
        }],
    };
