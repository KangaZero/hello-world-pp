//! Hello-world snippet for `ProgrammingLanguageName::Karel`.
//!
//! Imported from leachim6/hello-world (`k/Karel.kl` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/k/Karel.kl>
//! - <https://www.wikidata.org/wiki/Q204693> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Educational,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Karel => "Karel",
    versions: [Karel => "Karel"],
    extensions: [Kl => "kl"],
}

const KAREL: &str = concat!(
    r#"PROGRAM hello_world
BEGIN
  WRITE(""#,
    output_for_hello_world!(),
    r#"", CR)
END hello_world"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Karel,
        extensions: &[FileExtension::Kl],
        categories: &[Educational],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Karel,
            syntax: KAREL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    KAREL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"PROGRAM hello_world
BEGIN
  WRITE(""#,
                            output_for_hello_world!(),
                            r#"", CR)
END hello_world"#
                        ),
                    )],
                )
            },
        }],
    };
