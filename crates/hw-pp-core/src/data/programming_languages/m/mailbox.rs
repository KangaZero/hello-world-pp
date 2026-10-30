//! Hello-world snippet for `ProgrammingLanguageName::Mailbox`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (H-M)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(H-M)#MailBox>
//! - <https://esolangs.org/wiki/MailBox>

use crate::data::programming_languages::ProgrammingParadigm::ObjectOriented;
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Mailbox => "MailBox",
    versions: [Mailbox => "MailBox"],
    extensions: [],
}

const MAILBOX: &str = concat!(
    r#"box 0
(once)
send "output"#,
    output_for_hello_world!(),
    r#"" to 0
(contains "output")
output without "output""#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Mailbox,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Mailbox,
            syntax: MAILBOX,
            tokenized_syntax: &const {
                checked_token_lengths(
                    MAILBOX,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"box 0
(once)
send "output"#,
                            output_for_hello_world!(),
                            r#"" to 0
(contains "output")
output without "output""#
                        ),
                    )],
                )
            },
        }],
    };
