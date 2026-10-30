//! Hello-world snippet for `ProgrammingLanguageName::Nemerle`.
//!
//! Imported from leachim6/hello-world (`n/Nemerle.n` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/Nemerle.n>
//! - <https://www.wikidata.org/wiki/Q2626413> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, Imperative, Metaprogramming, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::DotNet, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Nemerle => "Nemerle",
    versions: [Nemerle => "Nemerle"],
    extensions: [N => "n"],
}

const NEMERLE: &str = concat!(
    r#"class Hello
{
    static Main () : void
    {
        System.Console.WriteLine (""#,
    output_for_hello_world!(),
    r#"");
    }
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Nemerle,
        extensions: &[FileExtension::N],
        categories: &[Compiled],
        families: &[DotNet],
        paradigms: &[
            Imperative,
            Procedural,
            ObjectOriented,
            Functional,
            Metaprogramming,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Nemerle,
            syntax: NEMERLE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NEMERLE,
                    [
                        (TokenKind::Keyword, r"class"),
                        (
                            TokenKind::Plain,
                            r" Hello
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"Main"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"void"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
        System",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Console"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"WriteLine"),
                        (TokenKind::Plain, r" "),
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
