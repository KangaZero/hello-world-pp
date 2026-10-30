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
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: PatternLanguage => "Pattern Language",
    versions: [PatternLanguage => "Pattern Language"],
    extensions: [Pat => "pat"],
}

const PATTERN_LANGUAGE: &str = concat!(
    r#"#include <std/sys.pat>
std::print(""#,
    output_for_hello_world!(),
    r#"");"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::PatternLanguage,
        extensions: &[FileExtension::Pat],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::PatternLanguage,
            syntax: PATTERN_LANGUAGE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PATTERN_LANGUAGE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"#include <std/sys.pat>
std::print(""#,
                            output_for_hello_world!(),
                            r#"");"#
                        ),
                    )],
                )
            },
        }],
    };
