//! Hello-world snippet for `ProgrammingLanguageName::Spam`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#Spam>
//! - <https://esolangs.org/wiki/Spam>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Spam => "Spam",
    versions: [Spam => "Spam"],
    extensions: [],
}

const SPAM: &str = concat!(
    r#"I want to tell you a secret, please read line 2.
Print ""#,
    output_for_hello_world!(),
    r#"", please read line 3.
I just want to say, you are fooled."#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Spam,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Spam,
            syntax: SPAM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    SPAM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"I want to tell you a secret, please read line 2.
Print ""#,
                            output_for_hello_world!(),
                            r#"", please read line 3.
I just want to say, you are fooled."#
                        ),
                    )],
                )
            },
        }],
    };
