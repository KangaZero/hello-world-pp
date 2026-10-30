//! Hello-world snippet for `ProgrammingLanguageName::Inform`.
//!
//! Imported from leachim6/hello-world (`i/Inform.inform` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/i/Inform.inform>
//! - <https://www.wikidata.org/wiki/Q539268> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Declarative, ObjectOriented, Procedural,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Inform => "Inform",
    versions: [Inform => "Inform"],
    extensions: [Inform => "inform"],
}

const INFORM: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r#"" by "I.F. Author"

The world is a room.

When play begins, say ""#,
    output_for_hello_world!(),
    r#"""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Inform,
        extensions: &[FileExtension::Inform],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, ObjectOriented, Declarative],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Inform,
            syntax: INFORM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    INFORM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"""#,
                            output_for_hello_world!(),
                            r#"" by "I.F. Author"

The world is a room.

When play begins, say ""#,
                            output_for_hello_world!(),
                            r#"""#
                        ),
                    )],
                )
            },
        }],
    };
