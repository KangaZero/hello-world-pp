//! Hello-world snippet for `ProgrammingLanguageName::Fakeasm`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FakeASM>
//! - <https://esolangs.org/wiki/FakeASM>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fakeasm => "FakeASM",
    versions: [Fakeasm => "FakeASM"],
    extensions: [],
}

const FAKEASM: &str = concat!(
    r#" ECHO ""#,
    output_for_hello_world!(),
    r#""
 STP"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fakeasm,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fakeasm,
            syntax: FAKEASM,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FAKEASM,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#" ECHO ""#,
                            output_for_hello_world!(),
                            r#""
 STP"#
                        ),
                    )],
                )
            },
        }],
    };
