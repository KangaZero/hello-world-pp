//! Hello-world snippet for `ProgrammingLanguageName::MplabIde`.
//!
//! Imported from the Hello World Collection (`files/MPLAB&nbsp;IDE.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/MPLAB%26nbsp%3BIDE.htm>
//! - <http://helloworldcollection.de/#MPLAB%C2%A0IDE>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: MplabIde => "MPLAB IDE",
    versions: [MplabIde => "MPLAB IDE"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::MplabIde,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::MplabIde,
            syntax: concat!(
                r"; ",
                output_for_hello_world!(),
                r#" in MPLAB IDE

load_HELLO_W
lfsr2,0x100
movlwd'11'
movwfPOSTINC2
movlw"H"
movwfPOSTINC2
movlw"E"
movwfPOSTINC2
movlw"L"
movwfPOSTINC2
movwfPOSTINC2
movlw"O"
movwfPOSTINC2
movlw" "
movwfPOSTINC2
movlw"W"
movwfPOSTINC2
movlw"O"
movwfPOSTINC2
movlw"R"
movwfPOSTINC2
movlw"L"
movwfPOSTINC2
movlw"D"
movwfPOSTINC2
SEND_HELLO_WORLD
lfsr1,0x100
movf    POSTINC1,w
movwfstr_length
SLmovf       POSTINC1,w
movwfTXREG
TX_wait
btfss      TXSTA,TRMT
braTX_wait
decfszstr_length,f
braSL
DONE_MESSAGE
nop"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"; ",
                    output_for_hello_world!(),
                    r#" in MPLAB IDE

load_HELLO_W
lfsr2,0x100
movlwd'11'
movwfPOSTINC2
movlw"H"
movwfPOSTINC2
movlw"E"
movwfPOSTINC2
movlw"L"
movwfPOSTINC2
movwfPOSTINC2
movlw"O"
movwfPOSTINC2
movlw" "
movwfPOSTINC2
movlw"W"
movwfPOSTINC2
movlw"O"
movwfPOSTINC2
movlw"R"
movwfPOSTINC2
movlw"L"
movwfPOSTINC2
movlw"D"
movwfPOSTINC2
SEND_HELLO_WORLD
lfsr1,0x100
movf    POSTINC1,w
movwfstr_length
SLmovf       POSTINC1,w
movwfTXREG
TX_wait
btfss      TXSTA,TRMT
braTX_wait
decfszstr_length,f
braSL
DONE_MESSAGE
nop"#
                ),
            )],
        }],
    };
