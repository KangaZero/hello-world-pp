//! Hello-world snippet for `ProgrammingLanguageName::Haystack`.
//!
//! Imported from leachim6/hello-world (`h/haystack.hstk` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/h/haystack.hstk>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Haystack => "haystack",
    versions: [Haystack => "haystack"],
    extensions: [Hstk => "hstk"],
}

const HAYSTACK: &str = concat!(r#"""#, output_for_hello_world!(), r#""o|"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Haystack,
        extensions: &[FileExtension::Hstk],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Haystack,
            syntax: HAYSTACK,
            tokenized_syntax: &const {
                checked_token_lengths(
                    HAYSTACK,
                    [(
                        TokenKind::Plain,
                        concat!(r#"""#, output_for_hello_world!(), r#""o|"#),
                    )],
                )
            },
        }],
    };
