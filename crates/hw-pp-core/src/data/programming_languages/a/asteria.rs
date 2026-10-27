//! Hello-world snippet for `ProgrammingLanguageName::Asteria`.
//!
//! Imported from leachim6/hello-world (`a/Asteria` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Asteria>
//! - <https://github.com/lhmouse/asteria> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::Procedural;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Scripting, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Asteria => "Asteria",
    versions: [Asteria => "Asteria"],
    extensions: [],
}

const ASTERIA: &str = concat!(r#"std.io.putln(""#, output_for_hello_world!(), r#"");"#);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Asteria,
        extensions: &[],
        categories: &[Scripting],
        families: &[],
        paradigms: &[Procedural],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Asteria,
            syntax: ASTERIA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASTERIA,
                    [(
                        TokenKind::Plain,
                        concat!(r#"std.io.putln(""#, output_for_hello_world!(), r#"");"#),
                    )],
                )
            },
        }],
    };
