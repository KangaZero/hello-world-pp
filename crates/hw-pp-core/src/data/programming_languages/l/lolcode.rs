//! Hello-world snippet for `ProgrammingLanguageName::Lolcode`.
//!
//! Sources:
//! - <https://esolangs.org/wiki/LOLCODE>
//! - <https://github.com/justinmeza/lolcode-spec/blob/master/v1.2/lolcode-spec-v1.2.md>
//! - <https://www.wikidata.org/wiki/Q513238> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural, Structured};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Esolang,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Lolcode => "LOLCODE",
    versions: [Lolcode12 => "LOLCODE 1.2"],
    extensions: [Lol => "lol"],
}

const LOLCODE12: &str = concat!(
    r#"HAI 1.2
    VISIBLE ""#,
    output_for_hello_world!(),
    r#""
KTHXBYE"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Lolcode,
        extensions: &[FileExtension::Lol],
        categories: &[Esolang],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Lolcode12,
            syntax: LOLCODE12,
            tokenized_syntax: &const {
                checked_token_lengths(
                    LOLCODE12,
                    [
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
                )
            },
        }],
    };
