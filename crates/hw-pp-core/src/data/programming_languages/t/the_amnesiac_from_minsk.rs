//! Hello-world snippet for `ProgrammingLanguageName::TheAmnesiacFromMinsk`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (T-Z)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(T-Z)#The_Amnesiac_From_Minsk>
//! - <https://esolangs.org/wiki/The_Amnesiac_From_Minsk>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: TheAmnesiacFromMinsk => "The Amnesiac From Minsk",
    versions: [TheAmnesiacFromMinsk => "The Amnesiac From Minsk"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::TheAmnesiacFromMinsk,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::TheAmnesiacFromMinsk,
            syntax: r"L1	 +	 =	 -
0:	-0;	-1;	+58;	@1
1:	-0;	-2;	+59;	@2
2:	-0;	-3;	+58;	@3
3:	-0;	-4;	+59;	@2
4:	-0;	-5;	+58;	@5
5:	-0;	-6;	+59;	@3
6:	-0;	-7;	+58;	@3
7:	-0;	-8;	+59;	@2
8:	-0;	-9;	+58;	@2
9:	-0;	-10;	+59;	@2
10:	-0;	-11;	+58;	@2
11:	-0;	-12;	+59;	@3
12:	-0;	-13;	+58;	@2
13:	-0;	-14;	+59;	@3
14:	-0;	-15;	+58;	@4
15:	-0;	-16;	+59;	@3
16:	-0;	-17;	+58;	@2
17:	-0;	-18;	+59;	@3
18:	-0;	-19;	+58;	@4
19:	-0;	-20;	+59;	@3
20:	-0;	-21;	+58;	@2
21:	-0;	-22;	+59;	@5
22:	-0;	-23;	+58;	@3
23:	-0;	-24;	+59;	@2
24:	-0;	-25;	+58;	@2
25:	-0;	-26;	+59;	@3
26:	-0;	-27;	+58;	@5
27:	-0;	-28;	+59;	@2
28:	-0;	-29;	+58;	@7
29:	-0;	-30;	+59;	@4
30:	-0;	-31;	+58;	@2
31:	-0;	-32;	+59;	@4
32:	-0;	-33;	+58;	@2
33:	-0;	-34;	+59;	@3
34:	-0;	-35;	+58;	@2
35:	-0;	-36;	+59;	@5
36:	-0;	-37;	+58;	@2
37:	-0;	-38;	+59;	@4
38:	-0;	-39;	+58;	@3
39:	-0;	-40;	+59;	@2
40:	-0;	-41;	+58;	@3
41:	-0;	-42;	+59;	@3
42:	-0;	-43;	+58;	@2
43:	-0;	-44;	+59;	@3
44:	-0;	-45;	+58;	@4
45:	-0;	-46;	+59;	@3
46:	-0;	-47;	+58;	@3
47:	-0;	-48;	+59;	@2
48:	-0;	-49;	+58;	@5
49:	-0;	-50;	+59;	@2
50:	-0;	-51;	+58;	@5
51:	-0;	-52;	+59;	@2
52:	-0;	-53;	+58;	@5
53:	-0;	-54;	+59;	@2
54:	-0;	-55;	+58;	@2
55:	-0;	-56;	+59;	@2
56:	-0;	-57;	+58;	@2
57:	+57;	+57;	+57;	@2
58:	-0;	-0;	-0;	@2
59:	-0;	-0;	-0;	@2",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r"L1	 +	 =	 -
0:	-0;	-1;	+58;	@1
1:	-0;	-2;	+59;	@2
2:	-0;	-3;	+58;	@3
3:	-0;	-4;	+59;	@2
4:	-0;	-5;	+58;	@5
5:	-0;	-6;	+59;	@3
6:	-0;	-7;	+58;	@3
7:	-0;	-8;	+59;	@2
8:	-0;	-9;	+58;	@2
9:	-0;	-10;	+59;	@2
10:	-0;	-11;	+58;	@2
11:	-0;	-12;	+59;	@3
12:	-0;	-13;	+58;	@2
13:	-0;	-14;	+59;	@3
14:	-0;	-15;	+58;	@4
15:	-0;	-16;	+59;	@3
16:	-0;	-17;	+58;	@2
17:	-0;	-18;	+59;	@3
18:	-0;	-19;	+58;	@4
19:	-0;	-20;	+59;	@3
20:	-0;	-21;	+58;	@2
21:	-0;	-22;	+59;	@5
22:	-0;	-23;	+58;	@3
23:	-0;	-24;	+59;	@2
24:	-0;	-25;	+58;	@2
25:	-0;	-26;	+59;	@3
26:	-0;	-27;	+58;	@5
27:	-0;	-28;	+59;	@2
28:	-0;	-29;	+58;	@7
29:	-0;	-30;	+59;	@4
30:	-0;	-31;	+58;	@2
31:	-0;	-32;	+59;	@4
32:	-0;	-33;	+58;	@2
33:	-0;	-34;	+59;	@3
34:	-0;	-35;	+58;	@2
35:	-0;	-36;	+59;	@5
36:	-0;	-37;	+58;	@2
37:	-0;	-38;	+59;	@4
38:	-0;	-39;	+58;	@3
39:	-0;	-40;	+59;	@2
40:	-0;	-41;	+58;	@3
41:	-0;	-42;	+59;	@3
42:	-0;	-43;	+58;	@2
43:	-0;	-44;	+59;	@3
44:	-0;	-45;	+58;	@4
45:	-0;	-46;	+59;	@3
46:	-0;	-47;	+58;	@3
47:	-0;	-48;	+59;	@2
48:	-0;	-49;	+58;	@5
49:	-0;	-50;	+59;	@2
50:	-0;	-51;	+58;	@5
51:	-0;	-52;	+59;	@2
52:	-0;	-53;	+58;	@5
53:	-0;	-54;	+59;	@2
54:	-0;	-55;	+58;	@2
55:	-0;	-56;	+59;	@2
56:	-0;	-57;	+58;	@2
57:	+57;	+57;	+57;	@2
58:	-0;	-0;	-0;	@2
59:	-0;	-0;	-0;	@2",
            )],
        }],
    };
