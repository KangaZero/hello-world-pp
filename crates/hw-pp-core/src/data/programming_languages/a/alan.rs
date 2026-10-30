//! Hello-world snippet for `ProgrammingLanguageName::Alan`.
//!
//! Imported from leachim6/hello-world (`a/Alan.ln` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Alan.ln>
//! - <https://web.archive.org/web/2021/https://alan-lang.org/> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Concurrent;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Alan => "Alan",
    versions: [Alan => "Alan"],
    extensions: [Ln => "ln"],
}

const ALAN: &str = concat!(
    r#"import @std/app

on app.start {
  app.print(""#,
    output_for_hello_world!(),
    r#"");
  emit app.exit 0;
}"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Alan,
        extensions: &[FileExtension::Ln],
        categories: &[],
        families: &[],
        paradigms: &[Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Alan,
            syntax: ALAN,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ALAN,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"import @std/app

on app.start {
  app.print(""#,
                            output_for_hello_world!(),
                            r#"");
  emit app.exit 0;
}"#
                        ),
                    )],
                )
            },
        }],
    };
