//! Hello-world snippet for `ProgrammingLanguageName::AssemblerHp85`.
//!
//! Imported from leachim6/hello-world (`a/Assembler HP85.asm` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: AssemblerHp85 => "Assembler HP85",
    versions: [AssemblerHp85 => "Assembler HP85"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::AssemblerHp85,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::AssemblerHp85,
            syntax: concat!(
                r#"010
020         NAM HELLO
030         DEF RUNTIM
040         DEF TOKENS
050         DEF PARSE
060         DEF ERMSG
070         DEF INIT
100 PARSE   BYT 0,0
110 RUNTIM  BYT 0,0,377,377
120 TOKENS  BYT 377
130 ERMSG   BYT 377
140 !
150 INIT    LDM R26,=MSG
160         ADMD R26,=BINTAB
170         LDM R36,=12D,0
180         JSB =OUTSTR
190         RTN
200 MSG     ASC ""#,
                output_for_hello_world!(),
                r#""
210 BINTAB  DAD 101233
220 OUTSTR  DAD 35052
300         FIN"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"010
020         NAM HELLO
030         DEF RUNTIM
040         DEF TOKENS
050         DEF PARSE
060         DEF ERMSG
070         DEF INIT
100 PARSE   BYT 0,0
110 RUNTIM  BYT 0,0,377,377
120 TOKENS  BYT 377
130 ERMSG   BYT 377
140 !
150 INIT    LDM R26,=MSG
160         ADMD R26,=BINTAB
170         LDM R36,=12D,0
180         JSB =OUTSTR
190         RTN
200 MSG     ASC ""#,
                    output_for_hello_world!(),
                    r#""
210 BINTAB  DAD 101233
220 OUTSTR  DAD 35052
300         FIN"#
                ),
            )],
        }],
    };
