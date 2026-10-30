//! Hello-world snippet for `ProgrammingLanguageName::N1cEnterprise`.
//!
//! Imported from leachim6/hello-world (`#/1C Enterprise` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/%23/1C%20Enterprise>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: N1cEnterprise => "1C Enterprise",
    versions: [N1cEnterprise => "1C Enterprise"],
    extensions: [],
}

const N1C_ENTERPRISE: &str = concat!(r#"Message(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::N1cEnterprise,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::N1cEnterprise,
            syntax: N1C_ENTERPRISE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    N1C_ENTERPRISE,
                    [(
                        TokenKind::Plain,
                        concat!(r#"Message(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
