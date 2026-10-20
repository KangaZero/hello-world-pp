//! Hello-world snippet for `ProgrammingLanguageName::ClcIntercal`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (B-C)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(B-C)#CLC-INTERCAL>
//! - <https://esolangs.org/wiki/CLC-INTERCAL>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: ClcIntercal => "CLC-INTERCAL",
    versions: [ClcIntercal => "CLC-INTERCAL"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::ClcIntercal,
        extensions: &[],
        categories: &[Esolang],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::ClcIntercal,
            syntax: r#"	PLEASE ;1 <- #12
(1)	PLEASE WRITE IN ;1
	PLEASE GIVE UP

	PLEASE PLEASE PLEASE (PRETTY PLEASE)
	DO NOT ASK ME TO READ THE REST OF THIS PROGRAM

	DO COME FROM '".1/'"'¥#29¢"';1SUB#1'~#65535"'~'#0¢#65535'"~
		     "'¥#29¢"';1SUB#1'~#65535"'~'#0¢#65535'"'~#1"~#0'¢.1
	ERROR: THE FIRST CHARACTER IS NOT "H"

	DO COME FROM '".2/'"'¥#1356¢"';1SUB#2'~#65535"'~'#0¢#65535'"~
		     "'¥#1356¢"';1SUB#2'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '.2~"'¥#1¢.1'~#1"'
	ERROR: THE SECOND CHARACTER IS NOT "E"

	DO COME FROM '".3/'"'¥#377¢"';1SUB#3'~#65535"'~'#0¢#65535'"~
		     "'¥#377¢"';1SUB#3'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".22/'"V.1¢.2"~#1'"~#0'¢'.3~"'¥#1¢.22'~#1"'
	ERROR: THE THIRD CHARACTER IS NOT "L"

	DO COME FROM '".4/'"'¥#383¢"';1SUB#4'~#65535"'~'#0¢#65535'"~
		     "'¥#383¢"';1SUB#4'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".23/'"V.22¢.3"~#1'"~#0'¢'.4~"'¥#1¢.23'~#1"'
	ERROR: THE FOURTH CHARACTER IS NOT "L"

	DO COME FROM '".5/'"'¥#378¢"';1SUB#5'~#65535"'~'#0¢#65535'"~
		     "'¥#378¢"';1SUB#5'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".24/'"V.23¢.4"~#1'"~#0'¢'.5~"'¥#1¢.24'~#1"'
	ERROR: THE FIFTH CHARACTER IS NOT "O"

	DO COME FROM '".6/'"'¥#177¢"';1SUB#6'~#65535"'~'#0¢#65535'"~
		     "'¥#177¢"';1SUB#6'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".25/'"V.24¢.5"~#1'"~#0'¢'.6~"'¥#1¢.25'~#1"'
	ERROR: THE SIXTH CHARACTER IS NOT COMMA

	DO COME FROM '".7/'"'¥#373¢"';1SUB#7'~#65535"'~'#0¢#65535'"~
		     "'¥#373¢"';1SUB#7'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".26/'"V.25¢.6"~#1'"~#0'¢'.7~"'¥#1¢.26'~#1"'
	ERROR: THE SEVENTH CHARACTER IS NOT SPACE

	DO COME FROM '".8/'"'¥#4160¢"';1SUB#8'~#65535"'~'#0¢#65535'"~
		     "'¥#4160¢"';1SUB#8'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".27/'"V.26¢.7"~#1'"~#0'¢'.8~"'¥#1¢.27'~#1"'
	ERROR: THE EIGHTH CHARACTER IS NOT "W"

	DO COME FROM '".9/'"'¥#574¢"';1SUB#9'~#65535"'~'#0¢#65535'"~
		     "'¥#574¢"';1SUB#9'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".28/'"V.27¢.8"~#1'"~#0'¢'.9~"'¥#1¢.28'~#1"'
	ERROR: THE NINTH CHARACTER IS NOT "O"

	DO COME FROM '".10/'"'¥#540¢"';1SUB#10'~#65535"'~'#0¢#65535'"~
		     "'¥#540¢"';1SUB#10'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".29/'"V.28¢.9"~#1'"~#0'¢'.10~"'¥#1¢.29'~#1"'
	ERROR: THE TENTH CHARACTER IS NOT "R"

	DO COME FROM '".11/'"'¥#353¢"';1SUB#11'~#65535"'~'#0¢#65535'"~
		     "'¥#353¢"';1SUB#11'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".30/'"V.29¢.10"~#1'"~#0'¢'.11~"'¥#1¢.30'~#1"'
	ERROR: THE ELEVENTH CHARACTER IS NOT "L"

	DO COME FROM '".12/'"'¥#375¢"';1SUB#12'~#65535"'~'#0¢#65535'"~
		     "'¥#375¢"';1SUB#12'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".31/'"V.30¢.11"~#1'"~#0'¢'.12~"'¥#1¢.31'~#1"'
	ERROR: THE TWELFTH AND LAST CHARACTER IS NOT "D""#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"	PLEASE ;1 <- #12
(1)	PLEASE WRITE IN ;1
	PLEASE GIVE UP

	PLEASE PLEASE PLEASE (PRETTY PLEASE)
	DO NOT ASK ME TO READ THE REST OF THIS PROGRAM

	DO COME FROM '".1/'"'¥#29¢"';1SUB#1'~#65535"'~'#0¢#65535'"~
		     "'¥#29¢"';1SUB#1'~#65535"'~'#0¢#65535'"'~#1"~#0'¢.1
	ERROR: THE FIRST CHARACTER IS NOT "H"

	DO COME FROM '".2/'"'¥#1356¢"';1SUB#2'~#65535"'~'#0¢#65535'"~
		     "'¥#1356¢"';1SUB#2'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '.2~"'¥#1¢.1'~#1"'
	ERROR: THE SECOND CHARACTER IS NOT "E"

	DO COME FROM '".3/'"'¥#377¢"';1SUB#3'~#65535"'~'#0¢#65535'"~
		     "'¥#377¢"';1SUB#3'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".22/'"V.1¢.2"~#1'"~#0'¢'.3~"'¥#1¢.22'~#1"'
	ERROR: THE THIRD CHARACTER IS NOT "L"

	DO COME FROM '".4/'"'¥#383¢"';1SUB#4'~#65535"'~'#0¢#65535'"~
		     "'¥#383¢"';1SUB#4'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".23/'"V.22¢.3"~#1'"~#0'¢'.4~"'¥#1¢.23'~#1"'
	ERROR: THE FOURTH CHARACTER IS NOT "L"

	DO COME FROM '".5/'"'¥#378¢"';1SUB#5'~#65535"'~'#0¢#65535'"~
		     "'¥#378¢"';1SUB#5'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".24/'"V.23¢.4"~#1'"~#0'¢'.5~"'¥#1¢.24'~#1"'
	ERROR: THE FIFTH CHARACTER IS NOT "O"

	DO COME FROM '".6/'"'¥#177¢"';1SUB#6'~#65535"'~'#0¢#65535'"~
		     "'¥#177¢"';1SUB#6'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".25/'"V.24¢.5"~#1'"~#0'¢'.6~"'¥#1¢.25'~#1"'
	ERROR: THE SIXTH CHARACTER IS NOT COMMA

	DO COME FROM '".7/'"'¥#373¢"';1SUB#7'~#65535"'~'#0¢#65535'"~
		     "'¥#373¢"';1SUB#7'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".26/'"V.25¢.6"~#1'"~#0'¢'.7~"'¥#1¢.26'~#1"'
	ERROR: THE SEVENTH CHARACTER IS NOT SPACE

	DO COME FROM '".8/'"'¥#4160¢"';1SUB#8'~#65535"'~'#0¢#65535'"~
		     "'¥#4160¢"';1SUB#8'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".27/'"V.26¢.7"~#1'"~#0'¢'.8~"'¥#1¢.27'~#1"'
	ERROR: THE EIGHTH CHARACTER IS NOT "W"

	DO COME FROM '".9/'"'¥#574¢"';1SUB#9'~#65535"'~'#0¢#65535'"~
		     "'¥#574¢"';1SUB#9'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".28/'"V.27¢.8"~#1'"~#0'¢'.9~"'¥#1¢.28'~#1"'
	ERROR: THE NINTH CHARACTER IS NOT "O"

	DO COME FROM '".10/'"'¥#540¢"';1SUB#10'~#65535"'~'#0¢#65535'"~
		     "'¥#540¢"';1SUB#10'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".29/'"V.28¢.9"~#1'"~#0'¢'.10~"'¥#1¢.29'~#1"'
	ERROR: THE TENTH CHARACTER IS NOT "R"

	DO COME FROM '".11/'"'¥#353¢"';1SUB#11'~#65535"'~'#0¢#65535'"~
		     "'¥#353¢"';1SUB#11'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".30/'"V.29¢.10"~#1'"~#0'¢'.11~"'¥#1¢.30'~#1"'
	ERROR: THE ELEVENTH CHARACTER IS NOT "L"

	DO COME FROM '".12/'"'¥#375¢"';1SUB#12'~#65535"'~'#0¢#65535'"~
		     "'¥#375¢"';1SUB#12'~#65535"'~'#0¢#65535'"'~#1"~#0'¢
		     '".31/'"V.30¢.11"~#1'"~#0'¢'.12~"'¥#1¢.31'~#1"'
	ERROR: THE TWELFTH AND LAST CHARACTER IS NOT "D""#,
            )],
        }],
    };
