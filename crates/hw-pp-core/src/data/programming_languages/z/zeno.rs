//! Hello-world snippet for `ProgrammingLanguageName::Zeno`.
//!
//! Imported from leachim6/hello-world (PR #1540 `z/Zeno.zno`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/pull/1540>
//! - <https://www.wikidata.org/wiki/Q7475367> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Procedural};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Zeno => "Zeno",
    versions: [Zeno => "Zeno"],
    extensions: [Zno => "zno"],
}

const ZENO: &str = concat!(
    r#"program print_hello
  put ""#,
    output_for_hello_world!(),
    r#""
end program"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Zeno,
        extensions: &[FileExtension::Zno],
        categories: &[],
        families: &[],
        paradigms: &[Imperative, Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Zeno,
            syntax: ZENO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ZENO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"program print_hello
  put ""#,
                            output_for_hello_world!(),
                            r#""
end program"#
                        ),
                    )],
                )
            },
        }],
    };
