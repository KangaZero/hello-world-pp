//! Hello-world snippet for `ProgrammingLanguageName::Redscript`.
//!
//! Imported from leachim6/hello-world (PR #1790 `r/Redscript.reds`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1790>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Redscript => "Redscript",
    versions: [Redscript => "Redscript"],
    extensions: [Reds => "reds"],
}

const REDSCRIPT: &str = concat!(
    r#"@wrapMethod(EquipmentSystemPlayerData)
func OnRestored() -> Void {
  wrappedMethod();
  Log(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Redscript,
        extensions: &[FileExtension::Reds],
        categories: &[Scripting],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Redscript,
            syntax: REDSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    REDSCRIPT,
                    [
                        (TokenKind::Macro, r"@wrapMethod"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"EquipmentSystemPlayerData"),
                        (TokenKind::Punctuation, r")"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"func"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"OnRestored"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" -> "),
                        (TokenKind::Type, r"Void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"wrappedMethod"),
                        (TokenKind::Punctuation, r"();"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"Log"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"}"),
                    ],
                )
            },
        }],
    };
