//! Hello-world snippet for `ProgrammingLanguageName::RpgIv`.
//!
//! Imported from leachim6/hello-world (`r/RPG IV.rpgle` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/RPG&nbsp;IV&nbsp;v5.htm`, `files/RPG&nbsp;IV&nbsp;v7.1.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/r/RPG%20IV.rpgle>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RPG%26nbsp%3BIV%26nbsp%3Bv5.htm>
//! - <http://helloworldcollection.de/#RPG%C2%A0IV%C2%A0v5>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/RPG%26nbsp%3BIV%26nbsp%3Bv7.1.htm>
//! - <http://helloworldcollection.de/#RPG%C2%A0IV%C2%A0v7.1>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Compiled, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: RpgIv => "RPG IV",
    versions: [RpgIv => "RPG IV", RpgIvV5 => "RPG IV v5", RpgIvV71 => "RPG IV v7.1"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::RpgIv,
        extensions: &[],
        categories: &[Compiled],
        families: &[],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RpgIv,
                syntax: concat!(
                    r"/free
    dsply '",
                    output_for_hello_world!(),
                    r"';
    return;
/end-free"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"/free
    dsply '",
                        output_for_hello_world!(),
                        r"';
    return;
/end-free"
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RpgIvV5,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r" in RPG IV version 5

D wait           S              1

  /Free
   dsply ( '",
                    output_for_hello_world!(),
                    r"') ' ' wait;

   *inlr = *on;    "
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"// ",
                        output_for_hello_world!(),
                        r" in RPG IV version 5

D wait           S              1

  /Free
   dsply ( '",
                        output_for_hello_world!(),
                        r"') ' ' wait;

   *inlr = *on;    "
                    ),
                )],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::RpgIvV71,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r" in RPG IV version 7.1

dcl-s wait char(1);

dsply ( '",
                    output_for_hello_world!(),
                    r"') ' ' wait;

*inlr = *on;"
                ),
                tokenized_syntax: &[(
                    TokenKind::Plain,
                    concat!(
                        r"// ",
                        output_for_hello_world!(),
                        r" in RPG IV version 7.1

dcl-s wait char(1);

dsply ( '",
                        output_for_hello_world!(),
                        r"') ' ' wait;

*inlr = *on;"
                    ),
                )],
            },
        ],
    };
