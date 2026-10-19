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
    programming_languages_token::TokenKind,
};

language_names! {
    language: X10 => "X10",
    versions: [X10 => "X10"],
    extensions: [X10 => "x10"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::X10,
        extensions: &[FileExtension::X10],
        categories: &[Compiled],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::X10,
            syntax: concat!(
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
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
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
                ),
            )],
        }],
    };
