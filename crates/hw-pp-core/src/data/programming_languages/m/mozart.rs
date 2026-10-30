//! Hello-world snippet for `ProgrammingLanguageName::Mozart`.
//!
//! Imported from leachim6/hello-world (`m/Mozart.oz` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/m/Mozart.oz>
//! - <https://en.wikipedia.org/wiki/Oz_(programming_language)> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{
    Concurrent, Constraint, Functional, Imperative, Logic, ObjectOriented,
};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mozart => "Mozart",
    versions: [Mozart => "Mozart"],
    extensions: [Oz => "oz"],
}

const MOZART: &str = concat!(
    r"functor
import
     Application
     System
define
    {System.showInfo '",
    output_for_hello_world!(),
    r"'}
    {Application.exit 0} 
end"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mozart,
        extensions: &[FileExtension::Oz],
        categories: &[],
        families: &[],
        paradigms: &[
            Imperative,
            ObjectOriented,
            Functional,
            Logic,
            Constraint,
            Concurrent,
        ],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mozart,
            syntax: MOZART,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MOZART,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"functor
import
     Application
     System
define
    {System.showInfo '",
                            output_for_hello_world!(),
                            r"'}
    {Application.exit 0} 
end"
                        ),
                    )],
                )
            },
        }],
    };
