//! Hello-world snippet for `ProgrammingLanguageName::Smali`.
//!
//! Imported from leachim6/hello-world (`s/Smali.smali` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Smali.smali>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Smali => "Smali",
    versions: [Smali => "Smali"],
    extensions: [Smali => "smali"],
}

const SMALI: &str = concat!(
    r#".class public LHelloWorld;

.super Ljava/lang/Object;

.method public static main([Ljava/lang/String;)V
    .registers 2

    sget-object v0, Ljava/lang/System;->out:Ljava/io/PrintStream;

    const-string	v1, ""#,
    output_for_hello_world!(),
    r#""

    invoke-virtual {v0, v1}, Ljava/io/PrintStream;->println(Ljava/lang/String;)V

    return-void
.end method"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smali,
        extensions: &[FileExtension::Smali],
        categories: &[Assembly],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smali,
            syntax: SMALI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMALI,
                    [
                        (TokenKind::Macro, r".class"),
                        (TokenKind::Plain, r" public LHelloWorld"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Macro, r".super"),
                        (TokenKind::Plain, r" Ljava/lang/Object"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Macro, r".method"),
                        (TokenKind::Plain, r" public static main"),
                        (TokenKind::Punctuation, r"(["),
                        (TokenKind::Plain, r"Ljava/lang/"),
                        (TokenKind::Keyword, r"String"),
                        (TokenKind::Punctuation, r";)"),
                        (
                            TokenKind::Plain,
                            r"V
    ",
                        ),
                        (TokenKind::Macro, r".registers"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Number, r"2"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"sget-object"),
                        (TokenKind::Plain, r" v0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" Ljava/lang/System"),
                        (TokenKind::Punctuation, r";"),
                        (TokenKind::Plain, r"->out"),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r"Ljava/io/PrintStream"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"const-string"),
                        (TokenKind::Plain, r"	v1"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" "),
                        (
                            TokenKind::String,
                            concat!(r#"""#, output_for_hello_world!(), r#"""#),
                        ),
                        (
                            TokenKind::Plain,
                            r"

    ",
                        ),
                        (TokenKind::Keyword, r"invoke-virtual"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (TokenKind::Plain, r"v0"),
                        (TokenKind::Punctuation, r","),
                        (TokenKind::Plain, r" v1"),
                        (TokenKind::Punctuation, r"},"),
                        (TokenKind::Plain, r" Ljava/io/PrintStream"),
                        (TokenKind::Punctuation, r";"),
                        (TokenKind::Plain, r"->"),
                        (TokenKind::Function, r"println"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"Ljava/lang/"),
                        (TokenKind::Keyword, r"String"),
                        (TokenKind::Punctuation, r";)"),
                        (
                            TokenKind::Plain,
                            r"V

    ",
                        ),
                        (TokenKind::Keyword, r"return-void"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Macro, r".end"),
                        (TokenKind::Plain, r" method"),
                    ],
                )
            },
        }],
    };
