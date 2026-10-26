//! Hello-world snippet for `ProgrammingLanguageName::Sas`.
//!
//! Imported from leachim6/hello-world (`s/Sas.sas` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/s/Sas.sas>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scientific,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sas => "Sas",
    versions: [Sas => "Sas"],
    extensions: [Sas => "sas"],
}

const SAS: &str = concat!(
    r"%macro putit( string= ); 
     %put &string; 
     %mend; 

%putit(string=",
    output_for_hello_world!(),
    r")"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sas,
        extensions: &[FileExtension::Sas],
        categories: &[Scientific],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sas,
            syntax: SAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SAS,
                    [(
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
                )
            },
        }],
    };
