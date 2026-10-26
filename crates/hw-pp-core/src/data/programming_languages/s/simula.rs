//! Hello-world snippet for `ProgrammingLanguageName::Simula`.
//!
//! Imported from leachim6/hello-world (`s/Simula.sim` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Simula.sim>
//! - <https://www.wikidata.org/wiki/Q212569> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Imperative, ObjectOriented, Procedural, Structured,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Simula => "Simula",
    versions: [Simula => "Simula"],
    extensions: [Sim => "sim"],
}

const SIMULA: &str = concat!(
    r#"Begin
   OutText (""#,
    output_for_hello_world!(),
    r#"");
   Outimage;
End;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Simula,
        extensions: &[FileExtension::Sim],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Imperative, Procedural, Structured, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Simula,
            syntax: SIMULA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SIMULA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"Begin
   OutText (""#,
                            output_for_hello_world!(),
                            r#"");
   Outimage;
End;"#
                        ),
                    )],
                )
            },
        }],
    };
