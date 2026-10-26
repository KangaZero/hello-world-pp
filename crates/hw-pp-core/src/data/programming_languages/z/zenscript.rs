//! Hello-world snippet for `ProgrammingLanguageName::Zenscript`.
//!
//! Imported from leachim6/hello-world (PR #1691 `z/zenscript.zs`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1691>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zenscript => "ZenScript",
    versions: [Zenscript => "ZenScript"],
    extensions: [Zs => "zs"],
}

const ZENSCRIPT: &str = concat!(r#"println(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zenscript,
        extensions: &[FileExtension::Zs],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zenscript,
            syntax: ZENSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZENSCRIPT,
                    [
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                    ],
                )
            },
        }],
    };
