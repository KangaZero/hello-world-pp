//! Hello-world snippet for `ProgrammingLanguageName::Sqr`.
//!
//! Imported from the Hello World Collection (`files/SQR.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/SQR.htm>
//! - <http://helloworldcollection.de/#SQR>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Sqr => "SQR",
    versions: [Sqr => "SQR"],
    extensions: [],
}

const SQR: &str = concat!(
    r"! ",
    output_for_hello_world!(),
    r" in SQR
begin-program
   print '",
    output_for_hello_world!(),
    r".' (1,1)
end-program"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sqr,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sqr,
            syntax: SQR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SQR,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"! ",
                            output_for_hello_world!(),
                            r" in SQR
begin-program
   print '",
                            output_for_hello_world!(),
                            r".' (1,1)
end-program"
                        ),
                    )],
                )
            },
        }],
    };
