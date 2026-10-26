//! Hello-world snippet for `ProgrammingLanguageName::Ezhil`.
//!
//! Imported from the Hello World Collection (`files/Ezhil.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Ezhil.htm>
//! - <http://helloworldcollection.de/#Ezhil>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Ezhil => "Ezhil",
    versions: [Ezhil => "Ezhil"],
    extensions: [],
}

const EZHIL: &str = concat!(
    r"# ",
    output_for_hello_world!(),
    r#" in Ezhil

பதிப்பி "வணக்கம்!"
பதிப்பி "உலகே வணக்கம்"
பதிப்பி "******* நன்றி!. *******"
exit()"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Ezhil,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Ezhil,
            syntax: EZHIL,
            tokenized_syntax: &const {
                checked_token_lengths(
                    EZHIL,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"# ",
                            output_for_hello_world!(),
                            r#" in Ezhil

பதிப்பி "வணக்கம்!"
பதிப்பி "உலகே வணக்கம்"
பதிப்பி "******* நன்றி!. *******"
exit()"#
                        ),
                    )],
                )
            },
        }],
    };
