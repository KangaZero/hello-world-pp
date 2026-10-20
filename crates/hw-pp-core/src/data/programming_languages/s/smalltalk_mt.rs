//! Hello-world snippet for `ProgrammingLanguageName::SmalltalkMt`.
//!
//! Imported from the Hello World Collection (`files/Smalltalk&nbsp;MT.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Smalltalk%26nbsp%3BMT.htm>
//! - <http://helloworldcollection.de/#Smalltalk%C2%A0MT>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Smalltalk,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: SmalltalkMt => "Smalltalk MT",
    versions: [SmalltalkMt => "Smalltalk MT"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmalltalkMt,
        extensions: &[],
        categories: &[],
        families: &[Smalltalk],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmalltalkMt,
            syntax: concat!(
                r#"""#,
                output_for_hello_world!(),
                r" in Smalltalk MT

FrameWindow new
   title: '",
                output_for_hello_world!(),
                r"';
   open"
            ),
            tokenized_syntax: &[(
                TokenKind::Comment,
                concat!(
                    r#"""#,
                    output_for_hello_world!(),
                    r" in Smalltalk MT

FrameWindow new
   title: '",
                    output_for_hello_world!(),
                    r"';
   open"
                ),
            )],
        }],
    };
