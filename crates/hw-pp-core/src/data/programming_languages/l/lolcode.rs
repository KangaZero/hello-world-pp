//! Hello-world snippets for `ProgrammingLanguageName::Lolcode`.
//!
//! Sources:
//! - <https://esolangs.org/wiki/LOLCODE>
//! - <https://github.com/justinmeza/lolcode-spec/blob/master/v1.2/lolcode-spec-v1.2.md>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Lolcode => "LOLCODE",
    versions: [Lolcode12 => "LOLCODE 1.2"],
    extensions: [Lol => "lol"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &[FileExtension::Lol],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lolcode12,
            syntax: concat!(
                r#"HAI 1.2
    VISIBLE ""#,
                output_for_hello_world!(),
                r#""
KTHXBYE"#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, "HAI"),
                (TokenKind::Plain, " "),
                (TokenKind::Number, "1.2"),
                (TokenKind::Plain, "\n    "),
                (TokenKind::Keyword, "VISIBLE"),
                (TokenKind::Plain, " "),
                (
                    TokenKind::String,
                    concat!("\"", output_for_hello_world!(), "\""),
                ),
                (TokenKind::Plain, "\n"),
                (TokenKind::Keyword, "KTHXBYE"),
            ],
        }],
    };
