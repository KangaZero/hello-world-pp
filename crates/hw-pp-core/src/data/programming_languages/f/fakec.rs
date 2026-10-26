//! Hello-world snippet for `ProgrammingLanguageName::Fakec`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#FakeC>
//! - <https://esolangs.org/wiki/FakeC>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::language_names,
    programming_languages_token::{TokenKind, checked_token_lengths},
};

language_names! {
    language: Fakec => "FakeC",
    versions: [Fakec => "FakeC"],
    extensions: [],
}

const FAKEC: &str = r"#include<stdio.h>
char tape[1000000];
int p;
int main(){
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
while(tape[p]){
++p;
++tape[p];
++tape[p];
++tape[p];
++tape[p];
while(tape[p]){
++p;
++tape[p];
++tape[p];
++p;
++tape[p];
++tape[p];
++tape[p];
++p;
++tape[p];
++tape[p];
++tape[p];
++p;
++tape[p];
--p;
--p;
--p;
--p;
--tape[p];
}
++p;
++tape[p];
++p;
++tape[p];
++p;
--tape[p];
++p;
++p;
++tape[p];
while(tape[p]){
--p;
}
--p;
--tape[p];
}
++p;
++p;
putchar(tape[p]);
++p;
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
++p;
++p;
putchar(tape[p]);
--p;
--tape[p];
putchar(tape[p]);
--p;
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
++p;
++p;
++tape[p];
putchar(tape[p]);
++p;
++tape[p];
++tape[p];
putchar(tape[p]);
return 0;
}";

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fakec,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fakec,
            syntax: FAKEC,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FAKEC,
                    [(
                        TokenKind::Plain,
                        r"#include<stdio.h>
char tape[1000000];
int p;
int main(){
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
while(tape[p]){
++p;
++tape[p];
++tape[p];
++tape[p];
++tape[p];
while(tape[p]){
++p;
++tape[p];
++tape[p];
++p;
++tape[p];
++tape[p];
++tape[p];
++p;
++tape[p];
++tape[p];
++tape[p];
++p;
++tape[p];
--p;
--p;
--p;
--p;
--tape[p];
}
++p;
++tape[p];
++p;
++tape[p];
++p;
--tape[p];
++p;
++p;
++tape[p];
while(tape[p]){
--p;
}
--p;
--tape[p];
}
++p;
++p;
putchar(tape[p]);
++p;
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
++p;
++p;
putchar(tape[p]);
--p;
--tape[p];
putchar(tape[p]);
--p;
putchar(tape[p]);
++tape[p];
++tape[p];
++tape[p];
putchar(tape[p]);
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
--tape[p];
putchar(tape[p]);
++p;
++p;
++tape[p];
putchar(tape[p]);
++p;
++tape[p];
++tape[p];
putchar(tape[p]);
return 0;
}",
                    )],
                )
            },
        }],
    };
