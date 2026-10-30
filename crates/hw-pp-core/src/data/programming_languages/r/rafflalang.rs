//! Hello-world snippet for `ProgrammingLanguageName::Rafflalang`.
//!
//! Imported from leachim6/hello-world (`r/Rafflalang.bro` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/Rafflalang.bro>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Rafflalang => "Rafflalang",
    versions: [Rafflalang => "Rafflalang"],
    extensions: [Bro => "bro"],
}

const RAFFLALANG: &str = concat!(
    r#"TODO MUNDO FALA DE MIM ""#,
    output_for_hello_world!(),
    r#"" BRO;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Rafflalang,
        extensions: &[FileExtension::Bro],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Rafflalang,
            syntax: RAFFLALANG,
            tokenized_syntax: &const {
                checked_token_lengths(
                    RAFFLALANG,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"TODO MUNDO FALA DE MIM ""#,
                            output_for_hello_world!(),
                            r#"" BRO;"#
                        ),
                    )],
                )
            },
        }],
    };
