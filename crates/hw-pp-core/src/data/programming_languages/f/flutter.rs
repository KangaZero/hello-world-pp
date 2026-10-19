//! Hello-world snippet for `ProgrammingLanguageName::Flutter`.
//!
//! Imported from leachim6/hello-world (`f/Flutter.dart` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Flutter => "Flutter",
    versions: [Flutter => "Flutter"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Flutter,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Flutter,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
