//! Hello-world snippet for `ProgrammingLanguageName::Kitanai`.
//!
//! Imported from leachim6/hello-world (`k/Kitanai.ktn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Kitanai.ktn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Kitanai => "Kitanai",
    versions: [Kitanai => "Kitanai"],
    extensions: [Ktn => "ktn"],
}

const KITANAI: &str = concat!(r#"print ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kitanai,
        extensions: &[FileExtension::Ktn],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kitanai,
            syntax: KITANAI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KITANAI,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
