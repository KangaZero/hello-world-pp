//! Hello-world snippet for `ProgrammingLanguageName::Sacred`.
//!
//! Imported from leachim6/hello-world (`s/Sacred.sacred` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sacred.sacred>
//! - <https://esolangs.org/wiki/Sacred> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::CellBased;
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sacred => "Sacred",
    versions: [Sacred => "Sacred"],
    extensions: [Sacred => "sacred"],
}

const SACRED: &str = r")( )( (( )( (( (( () ( () ( (( () )) )( )( )( )) )( )) )( )) )( (( (( (( ) )) ) (( (( )( )( ((( (( () () () () () () ((( (( (( )( ((( ((( (( (( ((( (( () ((( )) )) ((( )) )) ((( (( (( (( ((( () () () ((( )) )) ((( )) )) )( ((( (( (( (( () (((";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sacred,
        extensions: &[FileExtension::Sacred],
        categories: &[],
        families: &[],
        paradigms: &[CellBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sacred,
            syntax: SACRED,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SACRED,
                    [(
                        TokenKind::Plain,
                        r")( )( (( )( (( (( () ( () ( (( () )) )( )( )( )) )( )) )( )) )( (( (( (( ) )) ) (( (( )( )( ((( (( () () () () () () ((( (( (( )( ((( ((( (( (( ((( (( () ((( )) )) ((( )) )) ((( (( (( (( ((( () () () ((( )) )) ((( )) )) )( ((( (( (( (( () (((",
                    )],
                )
            },
        }],
    };
