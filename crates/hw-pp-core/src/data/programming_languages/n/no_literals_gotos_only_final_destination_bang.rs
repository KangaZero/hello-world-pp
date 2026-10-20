//! Hello-world snippet for `ProgrammingLanguageName::NoLiteralsGotosOnlyFinalDestinationBang`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (N-S)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(N-S)#No_Literals,_Gotos_Only,_Final_Destination!>
//! - <https://esolangs.org/wiki/No_Literals,_Gotos_Only,_Final_Destination!>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: NoLiteralsGotosOnlyFinalDestinationBang => "No Literals, Gotos Only, Final Destination!",
    versions: [NoLiteralsGotosOnlyFinalDestinationBang => "No Literals, Gotos Only, Final Destination!"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::NoLiteralsGotosOnlyFinalDestinationBang,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::NoLiteralsGotosOnlyFinalDestinationBang,
            syntax: r"\,1
1+1,2
2+1,3
3+1,4
4+1,5
5+1,6
6+2,8

8+1,9
9+1,A
A+1,B
B+2,D

D+1,E
E+1,F
F+1,G
G+10
0,0

0,0
5D
5D
15,b
55
5G
26
60





0:D9*0+4B
0:D9*0+18

3F








0:D9*0+1D



























0:D9*0+56














0:D9*0+5G












0:D9*0+1E
0:D9*0+11,a






0:D9*0+a+1,a


0:D9*0+b+1,b


0:D9*0+5D",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"\,1
1+1,2
2+1,3
3+1,4
4+1,5
5+1,6
6+2,8

8+1,9
9+1,A
A+1,B
B+2,D

D+1,E
E+1,F
F+1,G
G+10
0,0

0,0
5D
5D
15,b
55
5G
26
60





0:D9*0+4B
0:D9*0+18

3F








0:D9*0+1D



























0:D9*0+56














0:D9*0+5G












0:D9*0+1E
0:D9*0+11,a






0:D9*0+a+1,a


0:D9*0+b+1,b


0:D9*0+5D",
            )],
        }],
    };
