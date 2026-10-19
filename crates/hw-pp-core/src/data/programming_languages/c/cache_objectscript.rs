//! Hello-world snippet for `ProgrammingLanguageName::CacheObjectscript`.
//!
//! Imported from leachim6/hello-world (`c/Cache ObjectScript.mac` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: CacheObjectscript => "Cache ObjectScript",
    versions: [CacheObjectscript => "Cache ObjectScript"],
    extensions: [Mac => "mac"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::CacheObjectscript,
        extensions: &[FileExtension::Mac],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::CacheObjectscript,
            syntax: concat!(
                r#"HelloWorld  ;
  Write ""#,
                output_for_hello_world!(),
                r#""
  Quit"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"HelloWorld  ;
  Write ""#,
                    output_for_hello_world!(),
                    r#""
  Quit"#
                ),
            )],
        }],
    };
