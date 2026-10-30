//! Hello-world snippet for `ProgrammingLanguageName::Cgi`.
//!
//! Imported from leachim6/hello-world (`c/CGI.c` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/c/CGI.c>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Cgi => "CGI",
    versions: [Cgi => "CGI"],
    extensions: [],
}

const CGI: &str = concat!(
    r#"#include <stdio.h>

main ()
{
   printf ("Content-type: text/html\n");
   printf ("\n");
   printf ("<html>\n");
   printf ("<head>\n");
   printf ("<title>"#,
    output_for_hello_world!(),
    r#"</title>\n");
   printf ("</head>\n");
   printf ("\n");
   printf ("<body>\n");
   printf ("<h1>"#,
    output_for_hello_world!(),
    r#"</h1>\n");
   printf ("</body>\n");
   printf ("</html>\n");
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cgi,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Cgi,
            syntax: CGI,
            tokenized_syntax: &const {
                checked_token_lengths(
                    CGI,
                    [
                        (TokenKind::Keyword, r"#include"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"<stdio.h>"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"()"),
                        (
                            TokenKind::Plain,
                            r"
",
                        ),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""Content-type: text/html\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""<html>\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""<head>\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#""<title>"#, output_for_hello_world!(), r#"</title>\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""</head>\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""<body>\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::String,
                            concat!(r#""<h1>"#, output_for_hello_world!(), r#"</h1>\n""#),
                        ),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""</body>\n""#),
                        (TokenKind::Punctuation, r");"),
                        (
                            TokenKind::Plain,
                            r"
   ",
                        ),
                        (TokenKind::Function, r"printf"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::String, r#""</html>\n""#),
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
