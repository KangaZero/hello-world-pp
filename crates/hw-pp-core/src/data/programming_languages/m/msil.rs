//! Hello-world snippet for `ProgrammingLanguageName::Msil`.
//!
//! Imported from the Hello World Collection (`files/MSIL.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MSIL.txt>
//! - <http://helloworldcollection.de/#MSIL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Msil => "MSIL",
    versions: [Msil => "MSIL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Msil,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Msil,
            syntax: concat!(
                r"//",
                output_for_hello_world!(),
                r#" in MSIL (.NET assembler)

.assembly helloworld {}
.class helloworld
{
 .method static void Main() cil managed
 {
  .entrypoint
  ldstr ""#,
                output_for_hello_world!(),
                r#""
  call void [mscorlib]System.Console::WriteLine(string)
  ret
 }
}"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Comment,
                    concat!(
                        r"//",
                        output_for_hello_world!(),
                        r" in MSIL (.NET assembler)"
                    ),
                ),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"assembly helloworld "),
                (TokenKind::Punctuation, r"{}"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Keyword, r"class"),
                (
                    TokenKind::Plain,
                    r" helloworld
",
                ),
                (TokenKind::Punctuation, r"{"),
                (
                    TokenKind::Plain,
                    r"
 ",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"method "),
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
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Punctuation, r"}"),
            ],
        }],
    };
