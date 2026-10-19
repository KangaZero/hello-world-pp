//! Hello-world snippet for `ProgrammingLanguageName::PatternLanguage`.
//!
//! Imported from leachim6/hello-world (`p/Pattern Language.pat` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Pattern%20Language.pat>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: PatternLanguage => "Pattern Language",
    versions: [PatternLanguage => "Pattern Language"],
    extensions: [Pat => "pat"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PatternLanguage,
        extensions: &[FileExtension::Pat],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PatternLanguage,
            syntax: concat!(
                r#"#include <std/sys.pat>
std::print(""#,
                output_for_hello_world!(),
                r#"");"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"#include <std/sys.pat>
std::print(""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
            )],
        }],
    };
