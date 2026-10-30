//! Hello-world snippet for `ProgrammingLanguageName::Ceylon`.
//!
//! Imported from leachim6/hello-world (`c/Ceylon.ceylon` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Ceylon.ceylon>
//! - <https://www.wikidata.org/wiki/Q93482> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Java, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ceylon => "Ceylon",
    versions: [Ceylon => "Ceylon"],
    extensions: [Ceylon => "ceylon"],
}

const CEYLON: &str = concat!(
    r#"shared void hello() {
    print(""#,
    output_for_hello_world!(),
    r#"");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ceylon,
        extensions: &[FileExtension::Ceylon],
        categories: &[Compiled],
        families: &[Java],
        paradigms: &[Imperative, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ceylon,
            syntax: CEYLON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CEYLON,
                    [
                        (TokenKind::Plain, r"shared "),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"print"),
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
