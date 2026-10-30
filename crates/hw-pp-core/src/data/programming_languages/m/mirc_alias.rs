//! Hello-world snippet for `ProgrammingLanguageName::MircAlias`.
//!
//! Imported from the Hello World Collection (`files/mIRC-Alias.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/mIRC-Alias.txt>
//! - <http://helloworldcollection.de/#mIRC-Alias>
//! - <https://en.wikipedia.org/wiki/MIRC_scripting_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{EventDriven, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: MircAlias => "mIRC-Alias",
    versions: [MircAlias => "mIRC-Alias"],
    extensions: [],
}

const MIRC_ALIAS: &str = concat!(
    r";",
    output_for_hello_world!(),
    r" for mIRC (alias section)

helloworld: /echo -a ",
    output_for_hello_world!()
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MircAlias,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, EventDriven],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MircAlias,
            syntax: MIRC_ALIAS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MIRC_ALIAS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r";",
                            output_for_hello_world!(),
                            r" for mIRC (alias section)

helloworld: /echo -a ",
                            output_for_hello_world!()
                        ),
                    )],
                )
            },
        }],
    };
