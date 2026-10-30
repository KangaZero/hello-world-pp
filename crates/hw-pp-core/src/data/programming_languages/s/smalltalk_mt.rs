//! Hello-world snippet for `ProgrammingLanguageName::SmalltalkMt`.
//!
//! Imported from the Hello World Collection (`files/Smalltalk&nbsp;MT.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Smalltalk%26nbsp%3BMT.htm>
//! - <http://helloworldcollection.de/#Smalltalk%C2%A0MT>
//! - <https://en.wikipedia.org/wiki/Smalltalk> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageFamily::Smalltalk,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: SmalltalkMt => "Smalltalk MT",
    versions: [SmalltalkMt => "Smalltalk MT"],
    extensions: [],
}

const SMALLTALK_MT: &str = concat!(
    r#"""#,
    output_for_hello_world!(),
    r" in Smalltalk MT

FrameWindow new
   title: '",
    output_for_hello_world!(),
    r"';
   open"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::SmalltalkMt,
        extensions: &[],
        categories: &[],
        families: &[Smalltalk],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::SmalltalkMt,
            syntax: SMALLTALK_MT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SMALLTALK_MT,
                    [(
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
                )
            },
        }],
    };
