//! Hello-world snippet for `ProgrammingLanguageName::AssemblerDgNova`.
//!
//! Imported from the Hello World Collection (`files/Assembler&nbsp;(DG-Nova).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Assembler%26nbsp%3B%28DG-Nova%29.txt>
//! - <http://helloworldcollection.de/#Assembler%C2%A0(DG-Nova)>
//! - <https://en.wikipedia.org/wiki/Assembly_language> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{Imperative, Metaprogramming};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Assembly, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: AssemblerDgNova => "Assembler (DG-Nova)",
    versions: [AssemblerDgNova => "Assembler (DG-Nova)"],
    extensions: [],
}

const ASSEMBLER_DG_NOVA: &str = concat!(
    r#"                        .TITL HELLO
02                      ; ""#,
    output_for_hello_world!(),
    r#"" FOR NOVA RUNNING RDOS
03                      ; USES PCHAR SYSTEM CALL
04                      .NREL
05                      .ENT START
06
07              START:
08 00000'022424 DOCHAR: LDA 0,@PMSG     ; LOAD AC0 WITH NEXT CHARACTER,
09 00001'101015         MOV# 0,0,SNR    ; TEST AC0;
10 00002'000412          JMP DONE ; SKIPPED IF NONZERO
11 00003'006017         .SYSTM
12 00004'010000         .PCHAR          ; PRINT FIRST
13 00005'000413          JMP ER ; SKIPPED IF OK
14 00006'101300         MOVS 0,0        ; SWAP BYTES
15 00007'006017         .SYSTM
16 00010'010000         .PCHAR          ; PRINT SECOND
17 00011'000407          JMP ER ; SKIPPED IF OK
18 00012'010412         ISZ PMSG        ; POINT TO NEXT WORD
19 00013'000765         JMP DOCHAR      ; GO AROUND AGAIN
20
21 00014'006017 DONE:   .SYSTM          ; NORMAL EXIT
22 00015'004400         .RTN
23 00016'000402          JMP ER
24 00017'063077         HALT
25 00020'006017 ER:     .SYSTM          ; ERROR EXIT
26 00021'006400         .ERTN
27 00022'063077          HALT
28 00023'063077         HALT
29
30 00024'000025'PMSG:   .+1     ; ADDRESS OF FIRST WORD OF TEXT
31                      ; NOTE BYTES ARE PACKED RIGHT-TO-LEFT BY DEFAULT
32 00025'042510         .TXT /"#,
    output_for_hello_world!(),
    r"<15><12>/ ; THAT'S CR LF
33       046114
34       026117
35       053440
36       051117
37       042114
38       006441
39       000012
40 00035'000000         0 ; FLAG WORD TO END STRING
41
42                      .END START"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerDgNova,
        extensions: &[],
        categories: &[Assembly],
        families: &[],
        paradigms: &[Imperative, Metaprogramming],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerDgNova,
            syntax: ASSEMBLER_DG_NOVA,
            tokenized_syntax: &const {
                checked_token_lengths(
                    ASSEMBLER_DG_NOVA,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"                        .TITL HELLO
02                      ; ""#,
                            output_for_hello_world!(),
                            r#"" FOR NOVA RUNNING RDOS
03                      ; USES PCHAR SYSTEM CALL
04                      .NREL
05                      .ENT START
06
07              START:
08 00000'022424 DOCHAR: LDA 0,@PMSG     ; LOAD AC0 WITH NEXT CHARACTER,
09 00001'101015         MOV# 0,0,SNR    ; TEST AC0;
10 00002'000412          JMP DONE ; SKIPPED IF NONZERO
11 00003'006017         .SYSTM
12 00004'010000         .PCHAR          ; PRINT FIRST
13 00005'000413          JMP ER ; SKIPPED IF OK
14 00006'101300         MOVS 0,0        ; SWAP BYTES
15 00007'006017         .SYSTM
16 00010'010000         .PCHAR          ; PRINT SECOND
17 00011'000407          JMP ER ; SKIPPED IF OK
18 00012'010412         ISZ PMSG        ; POINT TO NEXT WORD
19 00013'000765         JMP DOCHAR      ; GO AROUND AGAIN
20
21 00014'006017 DONE:   .SYSTM          ; NORMAL EXIT
22 00015'004400         .RTN
23 00016'000402          JMP ER
24 00017'063077         HALT
25 00020'006017 ER:     .SYSTM          ; ERROR EXIT
26 00021'006400         .ERTN
27 00022'063077          HALT
28 00023'063077         HALT
29
30 00024'000025'PMSG:   .+1     ; ADDRESS OF FIRST WORD OF TEXT
31                      ; NOTE BYTES ARE PACKED RIGHT-TO-LEFT BY DEFAULT
32 00025'042510         .TXT /"#,
                            output_for_hello_world!(),
                            r"<15><12>/ ; THAT'S CR LF
33       046114
34       026117
35       053440
36       051117
37       042114
38       006441
39       000012
40 00035'000000         0 ; FLAG WORD TO END STRING
41
42                      .END START"
                        ),
                    )],
                )
            },
        }],
    };
