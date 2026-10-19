//! Hello-world snippet for `ProgrammingLanguageName::Gar`.
//!
//! Imported from the Esolang wiki (`Hello world program in esoteric languages (D-G)`),
//! public domain (CC0); see <https://esolangs.org/wiki/Esolang:Copyrights>.
//!
//! Sources:
//! - <https://esolangs.org/wiki/Hello_world_program_in_esoteric_languages_(D-G)#gar>
//! - <https://esolangs.org/wiki/Gar>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageCategory::Esolang, ProgrammingLanguageDialect,
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Gar => "gar",
    versions: [Gar => "gar"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Gar,
        extensions: &[],
        categories: &[Esolang],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Gar,
            syntax: r">>>>>>>>>>
>>>>>>>
*
>>
+
>>>>>>>>>>
:
*
:
>
+
&
:
>>>>>>>>
+
&
:
>>>>>>>>
+
&
:
>>>>>>>>>>>
+
&
>>>>>>>>>>
>>>>
*
>>>>
+
&
>>>>>>>>>>
>>>
*
>>
+
&
:
>>>>>>>>>>>>>>>>>>>
+
&
:
>>>>>>>>>>>
+
&
:
>>>>>>>>>>>>>>
+
&
:
>>>>>>>>
+
&
>>>>>>>>>>>
>>>
*
,,",
            tokenized_syntax: &[(
                TokenKind::Plain,
                r">>>>>>>>>>
>>>>>>>
*
>>
+
>>>>>>>>>>
:
*
:
>
+
&
:
>>>>>>>>
+
&
:
>>>>>>>>
+
&
:
>>>>>>>>>>>
+
&
>>>>>>>>>>
>>>>
*
>>>>
+
&
>>>>>>>>>>
>>>
*
>>
+
&
:
>>>>>>>>>>>>>>>>>>>
+
&
:
>>>>>>>>>>>
+
&
:
>>>>>>>>>>>>>>
+
&
:
>>>>>>>>
+
&
>>>>>>>>>>>
>>>
*
,,",
            )],
        }],
    };
