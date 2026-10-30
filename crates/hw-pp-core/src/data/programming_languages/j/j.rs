//! Hello-world snippet for `ProgrammingLanguageName::J`.
//!
//! Imported from leachim6/hello-world (`j/J.ijs` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/j/J.ijs>
//! - <https://www.wikidata.org/wiki/Q383994> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Array, Functional, ObjectOriented};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::Apl, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: J => "J",
    versions: [J => "J"],
    extensions: [Ijs => "ijs"],
}

const J: &str = concat!(
    r"#!/opt/local/bin/jc
echo '",
    output_for_hello_world!(),
    r"'
exit ''"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::J,
        extensions: &[FileExtension::Ijs],
        categories: &[Scripting],
        families: &[Apl],
        paradigms: &[ObjectOriented, Functional, Array],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::J,
            syntax: J,
            tokenized_syntax: &const {
                checked_token_lengths(
                    J,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"#!/opt/local/bin/jc
echo '",
                            output_for_hello_world!(),
                            r"'
exit ''"
                        ),
                    )],
                )
            },
        }],
    };
