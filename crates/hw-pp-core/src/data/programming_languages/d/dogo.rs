//! Hello-world snippet for `ProgrammingLanguageName::Dogo`.
//!
//! Imported from leachim6/hello-world (`d/DOGO.dogo` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Dogo => "DOGO",
    versions: [Dogo => "DOGO"],
    extensions: [Dogo => "dogo"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Dogo,
        extensions: &[FileExtension::Dogo],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Dogo,
            syntax: r"roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over 
roll-over heel roll-over roll-over roll-over heel roll-over roll-over roll-over 
roll-over heel heel heel heel heel heel heel sit roll-over roll-over roll-over 
heel roll-over roll-over roll-over heel heel heel heel roll-over roll-over heel 
roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over  
roll-over roll-over heel roll-over roll-over roll-over roll-over roll-over heel 
roll-over heel heel heel heel heel heel heel roll-over roll-over roll-over 
roll-over roll-over heel heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel heel 
roll-over roll-over roll-over roll-over heel heel heel heel heel heel heel heel 
sit roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel 
heel heel roll-over roll-over heel roll-over roll-over roll-over roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over stay roll-over 
roll-over roll-over heel roll-over roll-over heel roll-over roll-over roll-over 
heel heel heel roll-over roll-over roll-over roll-over heel heel heel heel heel 
heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel heel roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over heel roll-over 
roll-over roll-over roll-over roll-over stay roll-over roll-over roll-over heel 
roll-over roll-over roll-over roll-over heel heel heel roll-over roll-over 
roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel roll-over roll-over heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over heel heel heel heel heel 
heel roll-over roll-over roll-over roll-over heel roll-over roll-over heel heel 
heel heel heel heel heel heel roll-over roll-over roll-over roll-over heel 
roll-over roll-over roll-over heel heel roll-over roll-over roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over 
roll-over heel roll-over roll-over roll-over heel roll-over roll-over roll-over 
roll-over heel heel heel heel heel heel heel sit roll-over roll-over roll-over 
heel roll-over roll-over roll-over heel heel heel heel roll-over roll-over heel 
roll-over roll-over roll-over roll-over roll-over heel roll-over roll-over 
roll-over roll-over roll-over stay roll-over roll-over roll-over heel roll-over  
roll-over roll-over heel roll-over roll-over roll-over roll-over roll-over heel 
roll-over heel heel heel heel heel heel heel roll-over roll-over roll-over 
roll-over roll-over heel heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel heel 
roll-over roll-over roll-over roll-over heel heel heel heel heel heel heel heel 
sit roll-over roll-over roll-over heel roll-over roll-over roll-over heel heel 
heel heel roll-over roll-over heel roll-over roll-over roll-over roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over stay roll-over 
roll-over roll-over heel roll-over roll-over heel roll-over roll-over roll-over 
heel heel heel roll-over roll-over roll-over roll-over heel heel heel heel heel 
heel heel heel heel heel sit roll-over roll-over roll-over heel roll-over 
roll-over roll-over heel heel heel heel heel heel heel heel heel roll-over 
roll-over heel roll-over roll-over roll-over roll-over roll-over heel roll-over 
roll-over roll-over roll-over roll-over stay roll-over roll-over roll-over heel 
roll-over roll-over roll-over roll-over heel heel heel roll-over roll-over 
roll-over roll-over heel roll-over roll-over roll-over roll-over heel heel heel 
heel roll-over roll-over heel roll-over heel heel heel roll-over roll-over 
roll-over roll-over roll-over heel roll-over roll-over heel heel heel heel heel 
heel roll-over roll-over roll-over roll-over heel roll-over roll-over heel heel 
heel heel heel heel heel heel roll-over roll-over roll-over roll-over heel 
roll-over roll-over roll-over heel heel roll-over roll-over roll-over roll-over 
heel roll-over roll-over roll-over roll-over roll-over heel",
            )],
        }],
    };
