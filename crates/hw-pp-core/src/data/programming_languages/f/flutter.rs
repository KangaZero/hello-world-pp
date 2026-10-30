//! Hello-world snippet for `ProgrammingLanguageName::Flutter`.
//!
//! Imported from leachim6/hello-world (`f/Flutter.dart` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/f/Flutter.dart>
//! - <https://www.wikidata.org/wiki/Q39072787> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Declarative;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Framework, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Flutter => "Flutter",
    versions: [Flutter => "Flutter"],
    extensions: [],
}

const FLUTTER: &str = concat!(
    r"import 'package:flutter/widgets.dart';

void main() {
  runApp(
    Text(
      '",
    output_for_hello_world!(),
    r"',
      textDirection: TextDirection.ltr,
    ),
  );
}"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flutter,
        extensions: &[],
        categories: &[Framework],
        families: &[],
        paradigms: &[Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flutter,
            syntax: FLUTTER,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FLUTTER,
                    [
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::String, r"'package:flutter/widgets.dart'"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"void"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"()"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Function, r"runApp"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Function, r"Text"),
                        (TokenKind::Punctuation, r"("),
                        (
                            TokenKind::Plain,
                            r"
      ",
                        ),
                        (
                            TokenKind::String,
                            concat!(r"'", output_for_hello_world!(), r"'"),
                        ),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"
      textDirection",
                        ),
                        (TokenKind::Punctuation, r":"),
                        (TokenKind::Plain, r" TextDirection"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"ltr"),
                        (TokenKind::Punctuation, r","),
                        (
                            TokenKind::Plain,
                            r"
    ",
                        ),
                        (TokenKind::Punctuation, r"),"),
                        (
                            TokenKind::Plain,
                            r"
  ",
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
