//! Hello-world snippet for `ProgrammingLanguageName::Assemblyscript`.
//!
//! Imported from leachim6/hello-world (PR #1805 `a/AssemblyScript.ts`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1805>
//! - <https://en.wikipedia.org/wiki/AssemblyScript> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Generic, Imperative, ObjectOriented,
};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Assemblyscript => "AssemblyScript",
    versions: [Assemblyscript => "AssemblyScript"],
    extensions: [],
}

const ASSEMBLYSCRIPT: &str = concat!(
    r#"export function hello(): string {
  return ""#,
    output_for_hello_world!(),
    r#"";
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Assemblyscript,
        extensions: &[],
        categories: &[Assembly],
        families: &[JavaScript],
        paradigms: &[Imperative, ObjectOriented, Functional, Generic],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Assemblyscript,
            syntax: ASSEMBLYSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLYSCRIPT,
                    [
                        (TokenKind::Keyword, r"export"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"function"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"hello"),
                        (TokenKind::Punctuation, r"():"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (TokenKind::Punctuation, r";"),
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
