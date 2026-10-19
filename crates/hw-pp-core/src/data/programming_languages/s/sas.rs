//! Hello-world snippet for `ProgrammingLanguageName::Sas`.
//!
//! Imported from leachim6/hello-world (`s/Sas.sas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Sas => "Sas",
    versions: [Sas => "Sas"],
    extensions: [Sas => "sas"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sas,
        extensions: &[FileExtension::Sas],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sas,
            syntax: concat!(
                r"%macro putit( string= ); 
     %put &string; 
     %mend; 

%putit(string=",
                output_for_hello_world!(),
                r")"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"%macro putit( string= ); 
     %put &string; 
     %mend; 

%putit(string=",
                    output_for_hello_world!(),
                    r")"
                ),
            )],
        }],
    };
