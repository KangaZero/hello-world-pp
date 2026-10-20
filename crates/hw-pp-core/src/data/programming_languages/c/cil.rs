//! Hello-world snippet for `ProgrammingLanguageName::Cil`.
//!
//! Imported from leachim6/hello-world (`c/Cil.il` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/Cil.il>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Assembly,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cil => "Cil",
    versions: [Cil => "Cil"],
    extensions: [Il => "il"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cil,
        extensions: &[FileExtension::Il],
        categories: &[Assembly],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cil,
            syntax: concat!(
                r#"// ilasm cil.il
.assembly HelloWorld {}
.method public static void Main() cil managed
{
     .entrypoint
     .maxstack 1
     ldstr ""#,
                output_for_hello_world!(),
                r#""
     call void [mscorlib]System.Console::WriteLine(string)
     ret
}"#
            ),
            tokenized_syntax: &[
                (TokenKind::Comment, r"// ilasm cil.il"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"assembly HelloWorld "),
                (TokenKind::Punctuation, r"{}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"method "),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"static"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"Main"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r" cil managed
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
     ",
                ),
                (TokenKind::Punctuation, r"."),
                (
                    TokenKind::Plain,
                    r"entrypoint
     ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"maxstack "),
                (TokenKind::Number, r"1"),
                (
                    TokenKind::Plain,
                    r"
     ldstr ",
                ),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r"
     call ",
                ),
                (TokenKind::Keyword, r"void"),
                (TokenKind::Plain, r" "),
                (TokenKind::Punctuation, r"["),
                (TokenKind::Plain, r"mscorlib"),
                (TokenKind::Punctuation, r"]"),
                (TokenKind::Plain, r"System"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Console"),
                (TokenKind::Punctuation, r"::"),
                (TokenKind::Function, r"WriteLine"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Type, r"string"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
     ret
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
