//! Hello-world snippet for `ProgrammingLanguageName::MircAlias`.
//!
//! Imported from the Hello World Collection (`files/mIRC-Alias.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/mIRC-Alias.txt>
//! - <http://helloworldcollection.de/#mIRC-Alias>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MircAlias => "mIRC-Alias",
    versions: [MircAlias => "mIRC-Alias"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MircAlias,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MircAlias,
            syntax: concat!(
                r";",
                output_for_hello_world!(),
                r" for mIRC (alias section)

helloworld: /echo -a ",
                output_for_hello_world!()
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";",
                    output_for_hello_world!(),
                    r" for mIRC (alias section)

helloworld: /echo -a ",
                    output_for_hello_world!()
                ),
            )],
        }],
    };
