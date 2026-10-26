//! Hello-world snippet for `ProgrammingLanguageName::Draco`.
//!
//! Imported from leachim6/hello-world (`d/Draco.d` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/d/Draco.d>

use crate::data::programming_languages::{
    ProgrammingLanguage,
    ProgrammingLanguageCategory::{Compiled, Historical},
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::Algol,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Draco => "Draco",
    versions: [Draco => "Draco"],
    extensions: [],
}

const DRACO: &str = concat!(
    r#"proc main()void:
    writeln(""#,
    output_for_hello_world!(),
    r#"");
corp;"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Draco,
        extensions: &[],
        categories: &[Compiled, Historical],
        families: &[Algol],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Draco,
            syntax: DRACO,
            tokenized_syntax: &const {
                checked_token_lengths(
                    DRACO,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"proc main()void:
    writeln(""#,
                            output_for_hello_world!(),
                            r#"");
corp;"#
                        ),
                    )],
                )
            },
        }],
    };
