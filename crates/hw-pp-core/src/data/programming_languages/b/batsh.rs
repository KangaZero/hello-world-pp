//! Hello-world snippet for `ProgrammingLanguageName::Batsh`.
//!
//! Imported from leachim6/hello-world (`b/Batsh.batsh` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/b/Batsh.batsh>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Batsh => "Batsh",
    versions: [Batsh => "Batsh"],
    extensions: [Batsh => "batsh"],
}

const BATSH: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Batsh,
        extensions: &[FileExtension::Batsh],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Batsh,
            syntax: BATSH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    BATSH,
                    [(
                        TokenKind::Plain,
                        concat!(r#"println(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
