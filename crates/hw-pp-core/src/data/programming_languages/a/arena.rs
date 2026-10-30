//! Hello-world snippet for `ProgrammingLanguageName::Arena`.
//!
//! Imported from leachim6/hello-world (`a/Arena.are` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Arena.are>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Arena => "Arena",
    versions: [Arena => "Arena"],
    extensions: [Are => "are"],
}

const ARENA: &str = concat!(r#"print(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Arena,
        extensions: &[FileExtension::Are],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Arena,
            syntax: ARENA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ARENA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"print(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
