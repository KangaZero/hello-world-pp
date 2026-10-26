//! Hello-world snippet for `ProgrammingLanguageName::Soupscript`.
//!
//! Imported from leachim6/hello-world (`s/SoupScript.script` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/SoupScript.script>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Soupscript => "SoupScript",
    versions: [Soupscript => "SoupScript"],
    extensions: [Script => "script"],
}

const SOUPSCRIPT: &str = concat!(
    r"PrintLn ",
    output_for_hello_world!(),
    r"

Break"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Soupscript,
        extensions: &[FileExtension::Script],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Soupscript,
            syntax: SOUPSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SOUPSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"PrintLn ",
                            output_for_hello_world!(),
                            r"

Break"
                        ),
                    )],
                )
            },
        }],
    };
