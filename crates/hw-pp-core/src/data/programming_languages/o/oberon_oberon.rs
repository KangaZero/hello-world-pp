//! Hello-world snippet for `ProgrammingLanguageName::OberonOberon`.
//!
//! Imported from the Hello World Collection (`files/Oberon.oberon` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Oberon.oberon>
//! - <http://helloworldcollection.de/#Oberon.oberon>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: OberonOberon => "Oberon.oberon",
    versions: [OberonOberon => "Oberon.oberon"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::OberonOberon,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::OberonOberon,
            syntax: concat!(
                r"MODULE HelloWorld;

(* ",
                output_for_hello_world!(),
                r#" in Oberon for the Oberon System *)

  IMPORT Oberon, Texts;

  VAR
      W: Texts.Writer;

  PROCEDURE Do*;
    BEGIN
        Texts.WriteString(W,""#,
                output_for_hello_world!(),
                r#"");
            Texts.WriteLn(W);
                Texts.Append(Oberon.Log,W.buf)
                  END Do;

BEGIN Texts.OpenWriter(W)
END HelloWorld."#
            ),
            tokenized_syntax: &[
                (TokenKind::Keyword, r"MODULE"),
                (TokenKind::Plain, r" HelloWorld"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (
                    TokenKind::Comment,
                    concat!(
                        r"(* ",
                        output_for_hello_world!(),
                        r" in Oberon for the Oberon System *)"
                    ),
                ),
                (
                    TokenKind::Plain,
                    r"

  ",
                ),
                (TokenKind::Keyword, r"IMPORT"),
                (TokenKind::Plain, r" Oberon"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r" Texts"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

  ",
                ),
                (TokenKind::Keyword, r"VAR"),
                (
                    TokenKind::Plain,
                    r"
      W",
                ),
                (TokenKind::Punctuation, r":"),
                (TokenKind::Plain, r" Texts"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"Writer"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

  ",
                ),
                (TokenKind::Keyword, r"PROCEDURE"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Do"),
                (TokenKind::Plain, r"*"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"
    ",
                ),
                (TokenKind::Keyword, r"BEGIN"),
                (
                    TokenKind::Plain,
                    r"
        Texts",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"WriteString"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"W"),
                (TokenKind::Punctuation, r","),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
            Texts",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"WriteLn"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"W"),
                (TokenKind::Punctuation, r");"),
                (
                    TokenKind::Plain,
                    r"
                Texts",
                ),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Append"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"Oberon"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"Log"),
                (TokenKind::Punctuation, r","),
                (TokenKind::Plain, r"W"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Plain, r"buf"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
                  ",
                ),
                (TokenKind::Keyword, r"END"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"Do"),
                (TokenKind::Punctuation, r";"),
                (
                    TokenKind::Plain,
                    r"

",
                ),
                (TokenKind::Keyword, r"BEGIN"),
                (TokenKind::Plain, r" Texts"),
                (TokenKind::Punctuation, r"."),
                (TokenKind::Function, r"OpenWriter"),
                (TokenKind::Punctuation, r"("),
                (TokenKind::Plain, r"W"),
                (TokenKind::Punctuation, r")"),
                (
                    TokenKind::Plain,
                    r"
",
                ),
                (TokenKind::Keyword, r"END"),
                (TokenKind::Plain, r" HelloWorld"),
                (TokenKind::Punctuation, r"."),
            ],
        }],
    };
