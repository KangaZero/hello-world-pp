//! Hello-world snippet for `ProgrammingLanguageName::Ubercode`.
//!
//! Imported from leachim6/hello-world (`u/Ubercode.cls` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/u/Ubercode.cls>
//! - <https://www.wikidata.org/wiki/Q7876482> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Ubercode => "Ubercode",
    versions: [Ubercode => "Ubercode"],
    extensions: [],
}

const UBERCODE: &str = concat!(
    r#"  Ubercode 1 class HelloWorld  

  public function Main()
  code
    call Msgbox(""#,
    output_for_hello_world!(),
    r#"", "My first program", "OK")
  end function

  end class"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ubercode,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ubercode,
            syntax: UBERCODE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    UBERCODE,
                    [(
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
                )
            },
        }],
    };
