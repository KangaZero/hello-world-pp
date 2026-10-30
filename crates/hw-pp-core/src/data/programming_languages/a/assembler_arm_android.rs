//! Hello-world snippet for `ProgrammingLanguageName::AssemblerArmAndroid`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(ARM,&nbsp;Android).htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28ARM%2C%26nbsp%3BAndroid%29.htm>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(ARM,%C2%A0Android)>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: AssemblerArmAndroid => "Assembler (ARM, Android)",
    versions: [AssemblerArmAndroid => "Assembler (ARM, Android)"],
    extensions: [],
}

const ASSEMBLER_ARM_ANDROID: &str = concat!(
    r"/* ",
    output_for_hello_world!(),
    r#" in ARM assembly (Android devices) */

.data

msg:
    .ascii      ""#,
    output_for_hello_world!(),
    r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     %r0, $1
    ldr     %r1, =msg
    ldr     %r2, =len
    mov     %r7, $4
    swi     $0
    mov     %r0, $0
    mov     %r7, $1
    swi     $0"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerArmAndroid,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerArmAndroid,
            syntax: ASSEMBLER_ARM_ANDROID,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_ARM_ANDROID,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r"/* ",
                            output_for_hello_world!(),
                            r#" in ARM assembly (Android devices) */

.data

msg:
    .ascii      ""#,
                            output_for_hello_world!(),
                            r#"\n"
len = . - msg

.text

.globl _start
_start:
    mov     %r0, $1
    ldr     %r1, =msg
    ldr     %r2, =len
    mov     %r7, $4
    swi     $0
    mov     %r0, $0
    mov     %r7, $1
    swi     $0"#
                        ),
                    )],
                )
            },
        }],
    };
