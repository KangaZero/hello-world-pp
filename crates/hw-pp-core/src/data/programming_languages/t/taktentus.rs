//! Hello-world snippet for `ProgrammingLanguageName::Taktentus`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#Taktentus>
//! - <https://esolangs.org/wiki/Taktentus>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Taktentus => "Taktentus",
    versions: [Taktentus => "Taktentus"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Taktentus,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Taktentus,
            syntax: concat!(
                r#"%% Write ""#,
                output_for_hello_world!(),
                r#""
h:=72
 @wy := h
h+=29  @wy := h
h+=7   @wy := h @wy := h
h:=111 @wy := h
h:=44  @wy := h @wy := @spacja
h:=87  @wy := h
h+=24  @wy := h
h+=3   @wy := h
h+=-6  @wy := h
h-=8   @wy := h
h:=33  @wy := h"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"%% Write ""#,
                    output_for_hello_world!(),
                    r#""
h:=72
 @wy := h
h+=29  @wy := h
h+=7   @wy := h @wy := h
h:=111 @wy := h
h:=44  @wy := h @wy := @spacja
h:=87  @wy := h
h+=24  @wy := h
h+=3   @wy := h
h+=-6  @wy := h
h-=8   @wy := h
h:=33  @wy := h"#
                ),
            )],
        }],
    };
