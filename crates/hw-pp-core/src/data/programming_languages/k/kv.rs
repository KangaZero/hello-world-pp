//! Hello-world snippet for `ProgrammingLanguageName::Kv`.
//!
//! Imported from leachim6/hello-world (`k/Kv.kv` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kv.kv>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Kv => "Kv",
    versions: [Kv => "Kv"],
    extensions: [Kv => "kv"],
}

const KV: &str = concat!(
    r#"Label:
        text: ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kv,
        extensions: &[FileExtension::Kv],
        categories: &[Markup],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kv,
            syntax: KV,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KV,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Label:
        text: ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
