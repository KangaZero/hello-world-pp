//! Hello-world snippet for `ProgrammingLanguageName::Zim`.
//!
//! Imported from leachim6/hello-world (`z/Zim.zim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zim.zim>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Zim => "Zim",
    versions: [Zim => "Zim"],
    extensions: [Zim => "zim"],
}

const ZIM: &str = concat!(r#"output ""#, output_for_hello_world!(), r#"""#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zim,
        extensions: &[FileExtension::Zim],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zim,
            syntax: ZIM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZIM,
                    [(
                        TokenKind::Plain,
                        concat!(r#"output ""#, output_for_hello_world!(), r#"""#),
                    )],
                )
            },
        }],
    };
