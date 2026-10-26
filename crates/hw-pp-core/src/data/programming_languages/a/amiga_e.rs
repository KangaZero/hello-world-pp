//! Hello-world snippet for `ProgrammingLanguageName::AmigaE`.
//!
//! Imported from leachim6/hello-world (`a/Amiga-E.amiga-e` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/Amiga-E.amiga-e>
//! - <https://en.wikipedia.org/wiki/Amiga_E> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Functional, ObjectOriented, Procedural,
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
    language: AmigaE => "Amiga-E",
    versions: [AmigaE => "Amiga-E"],
    extensions: [AmigaE => "amiga-e"],
}

const AMIGA_E: &str = concat!(
    r"PROC main() IS WriteF('",
    output_for_hello_world!(),
    r"\n')"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AmigaE,
        extensions: &[FileExtension::AmigaE],
        categories: &[Compiled, Historical],
        families: &[],
        paradigms: &[Procedural, ObjectOriented, Functional],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AmigaE,
            syntax: AMIGA_E,
            tokenized_syntax: &const {
                checked_token_lengths(
                    AMIGA_E,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"PROC main() IS WriteF('",
                            output_for_hello_world!(),
                            r"\n')"
                        ),
                    )],
                )
            },
        }],
    };
