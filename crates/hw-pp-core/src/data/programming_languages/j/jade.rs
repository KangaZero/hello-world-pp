//! Hello-world snippet for `ProgrammingLanguageName::Jade`.
//!
//! Imported from leachim6/hello-world (`j/Jade.jade` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/Jade.jade>
//! - <https://www.wikidata.org/wiki/Q6107617> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Concurrent, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Markup,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Algol, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Jade => "Jade",
    versions: [Jade => "Jade"],
    extensions: [Jade => "jade"],
}

const JADE: &str = concat!(
    r#"helloWorld();

begin
   write ""#,
    output_for_hello_world!(),
    r#"";
end;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Jade,
        extensions: &[FileExtension::Jade],
        categories: &[Markup],
        families: &[Algol],
        paradigms: &[ObjectOriented, Concurrent],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Jade,
            syntax: JADE,
            tokenized_syntax: &const {
                checked_token_lengths(
                    JADE,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"helloWorld();

begin
   write ""#,
                            output_for_hello_world!(),
                            r#"";
end;"#
                        ),
                    )],
                )
            },
        }],
    };
