//! Hello-world snippet for `ProgrammingLanguageName::Zonnon`.
//!
//! Imported from leachim6/hello-world (`z/Zonnon.znn` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/z/Zonnon.znn>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{DotNet, Oberon},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Zonnon => "Zonnon",
    versions: [Zonnon => "Zonnon"],
    extensions: [Znn => "znn"],
}

const ZONNON: &str = concat!(
    r#"module HelloWorld;
begin
    writeln(""#,
    output_for_hello_world!(),
    r#"");
end HelloWorld.    "#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zonnon,
        extensions: &[FileExtension::Znn],
        categories: &[Compiled],
        families: &[DotNet, Oberon],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zonnon,
            syntax: ZONNON,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZONNON,
                    [
                        (TokenKind::Keyword, r"module"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Keyword, r"begin"),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"writeln"),
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
                        (TokenKind::Keyword, r"end"),
                        (TokenKind::Plain, r" HelloWorld"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"    "),
                    ],
                )
            },
        }],
    };
