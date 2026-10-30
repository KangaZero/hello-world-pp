//! Hello-world snippet for `ProgrammingLanguageName::Chaiscript`.
//!
//! Imported from the Hello World Collection (`files/ChaiScript.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ChaiScript.htm>
//! - <http://helloworldcollection.de/#ChaiScript>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Chaiscript => "ChaiScript",
    versions: [Chaiscript => "ChaiScript"],
    extensions: [],
}

const CHAISCRIPT: &str = concat!(
    r"// ",
    output_for_hello_world!(),
    r#" in ChaiScript

#include <chaiscript/chaiscript.hpp>

std::string helloWorld(const std::string &t_name) {
  return "Hello " + t_name + "!";
}

int main() {
  chaiscript::ChaiScript chai;
  chai.add(chaiscript::fun(&helloWorld), "helloWorld");

  chai.eval(R"(
    puts(helloWorld("World"));
  )");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Chaiscript,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Chaiscript,
            syntax: CHAISCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CHAISCRIPT,
                    [
                        (
                            TokenKind::Comment,
                            concat!(r"// ", output_for_hello_world!(), r" in ChaiScript"),
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Macro, r"#include <chaiscript/chaiscript.hpp>"),
                        (
                            TokenKind::Plain,
                            r"

std",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"helloWorld"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Keyword, r"const"),
                        (TokenKind::Plain, r" std"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Type, r"string"),
                        (TokenKind::Plain, r" &t_name"),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"return"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""Hello ""#),
                        (TokenKind::Plain, r" + t_name + "),
                        (TokenKind::String, r#""!""#),
                        (TokenKind::Punctuation, r";"),
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
                        (TokenKind::Type, r"int"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  chaiscript",
                        ),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Plain, r"ChaiScript chai"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"
  chai",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"add"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"chaiscript"),
                        (TokenKind::Punctuation, r"::"),
                        (TokenKind::Keyword, r"fun"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"&helloWorld"),
                        (TokenKind::Punctuation, r"),"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r#""helloWorld""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"

  chai",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"eval"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"R"),
                        (TokenKind::String, r#""("#),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"puts"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Function, r"helloWorld"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""World""#),
                        (TokenKind::Punctuation, r"));"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Punctuation, r")"),
                        (TokenKind::String, r#"");"#),
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
