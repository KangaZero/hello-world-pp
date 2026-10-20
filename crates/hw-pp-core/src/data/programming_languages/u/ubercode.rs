//! Hello-world snippet for `ProgrammingLanguageName::Ubercode`.
//!
//! Imported from leachim6/hello-world (`u/Ubercode.cls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Ubercode.cls>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Ubercode => "Ubercode",
    versions: [Ubercode => "Ubercode"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ubercode,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ubercode,
            syntax: concat!(
                r#"  Ubercode 1 class HelloWorld  

  public function Main()
  code
    call Msgbox(""#,
                output_for_hello_world!(),
                r#"", "My first program", "OK")
  end function

  end class"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"  Ubercode 1 class HelloWorld  

  public function Main()
  code
    call Msgbox(""#,
                    output_for_hello_world!(),
                    r#"", "My first program", "OK")
  end function

  end class"#
                ),
            )],
        }],
    };
