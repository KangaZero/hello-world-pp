//! Hello-world snippet for `ProgrammingLanguageName::X10`.
//!
//! Imported from leachim6/hello-world (`x/X10.x10` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/x/X10.x10>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: X10 => "X10",
    versions: [X10 => "X10"],
    extensions: [X10 => "x10"],
}

const X10: &str = concat!(
    r#"/*
 *  This file is part of the X10 project (http://x10-lang.org).
 *
 *  This file is licensed to You under the Eclipse Public License (EPL);
 *  You may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *      http://www.opensource.org/licenses/eclipse-1.0.php
 *
 *  (C) Copyright IBM Corporation 2006-2016.
 */

import x10.io.Console;

public class HelloWorld {
  public static def main(Rail[String]) {
      Console.OUT.println(""#,
    output_for_hello_world!(),
    r#"");
  }
}

"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::X10,
        extensions: &[FileExtension::X10],
        categories: &[Compiled],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::X10,
            syntax: X10,
            tokenized_syntax: &const {
                checked_token_lengths(
                    X10,
                    [
                        (
                            TokenKind::Comment,
                            r"/*
 *  This file is part of the X10 project (http://x10-lang.org).
 *
 *  This file is licensed to You under the Eclipse Public License (EPL);
 *  You may not use this file except in compliance with the License.
 *  You may obtain a copy of the License at
 *      http://www.opensource.org/licenses/eclipse-1.0.php
 *
 *  (C) Copyright IBM Corporation 2006-2016.
 */",
                        ),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"import"),
                        (TokenKind::Plain, r" x10"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"io"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"Console"),
                        (TokenKind::Punctuation, r";"),
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"class"),
                        (TokenKind::Plain, r" HelloWorld "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
  ",
                        ),
                        (TokenKind::Keyword, r"public"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"static"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Keyword, r"def"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Function, r"main"),
                        (TokenKind::Punctuation, r"("),
                        (TokenKind::Plain, r"Rail"),
                        (TokenKind::Punctuation, r"["),
                        (TokenKind::Function, r"String"),
                        (TokenKind::Punctuation, r"])"),
                        (TokenKind::Plain, r" "),
                        (TokenKind::Punctuation, r"{"),
                        (
                            TokenKind::Plain,
                            r"
      Console",
                        ),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Plain, r"OUT"),
                        (TokenKind::Punctuation, r"."),
                        (TokenKind::Function, r"println"),
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
                        (
                            TokenKind::Plain,
                            r"

",
                        ),
                    ],
                )
            },
        }],
    };
