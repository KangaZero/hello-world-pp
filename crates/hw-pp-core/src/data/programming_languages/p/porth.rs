//! Hello-world snippet for `ProgrammingLanguageName::Porth`.
//!
//! Imported from leachim6/hello-world (`p/Porth.porth` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/p/Porth.porth>
//! - <https://gitlab.com/tsoding/porth> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concatenative, StackBased};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Porth => "Porth",
    versions: [Porth => "Porth"],
    extensions: [Porth => "porth"],
}

const PORTH: &str = concat!(
    r#"include "std.porth"

proc main in
  ""#,
    output_for_hello_world!(),
    r#"\n" puts
end"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Porth,
        extensions: &[FileExtension::Porth],
        categories: &[],
        families: &[],
        paradigms: &[StackBased, Concatenative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Porth,
            syntax: PORTH,
            tokenized_syntax: &const {
                checked_token_lengths(
                    PORTH,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"include "std.porth"

proc main in
  ""#,
                            output_for_hello_world!(),
                            r#"\n" puts
end"#
                        ),
                    )],
                )
            },
        }],
    };
