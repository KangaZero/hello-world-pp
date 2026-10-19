//! Hello-world snippet for `ProgrammingLanguageName::Sacred`.
//!
//! Imported from leachim6/hello-world (`s/Sacred.sacred` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Sacred => "Sacred",
    versions: [Sacred => "Sacred"],
    extensions: [Sacred => "sacred"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sacred,
        extensions: &[FileExtension::Sacred],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sacred,
            syntax: r")( )( (( )( (( (( () ( () ( (( () )) )( )( )( )) )( )) )( )) )( (( (( (( ) )) ) (( (( )( )( ((( (( () () () () () () ((( (( (( )( ((( ((( (( (( ((( (( () ((( )) )) ((( )) )) ((( (( (( (( ((( () () () ((( )) )) ((( )) )) )( ((( (( (( (( () (((",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r")( )( (( )( (( (( () ( () ( (( () )) )( )( )( )) )( )) )( )) )( (( (( (( ) )) ) (( (( )( )( ((( (( () () () () () () ((( (( (( )( ((( ((( (( (( ((( (( () ((( )) )) ((( )) )) ((( (( (( (( ((( () () () ((( )) )) ((( )) )) )( ((( (( (( (( () (((",
            )],
        }],
    };
