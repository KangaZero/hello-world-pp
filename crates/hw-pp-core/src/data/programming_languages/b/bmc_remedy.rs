//! Hello-world snippet for `ProgrammingLanguageName::BmcRemedy`.
//!
//! Imported from the Hello World Collection (`files/BMC&nbsp;Remedy.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/BMC%26nbsp%3BRemedy.htm>
//! - <http://helloworldcollection.de/#BMC%C2%A0Remedy>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: BmcRemedy => "BMC Remedy",
    versions: [BmcRemedy => "BMC Remedy"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::BmcRemedy,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::BmcRemedy,
            syntax: concat!(
                r"char-set: windows-1252
#
#  ",
                output_for_hello_world!(),
                r" in BMC Remedy 7.0
#  File exported Thu May  8 09:36:46 2008
#
begin active link
   name           : Remedy_HelloWorld
   timestamp      : 1210249958
   export-version : 9
   owner          : Demo
   last-changed   : Demo
   actlink-order  : 0
   wk-conn-type   : 1
   schema-name    : _1
   actlink-mask   : 16
   enable         : 1
   permission     : 0
   action {
      message-type: 0
      message-num : 10000
      message-pane: 1
      message-text: ",
                output_for_hello_world!(),
                r"!
   }
   object-prop    : 2\60016\4\1\0\60017\4\1\0\
end"
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"char-set: windows-1252
#
#  ",
                    output_for_hello_world!(),
                    r" in BMC Remedy 7.0
#  File exported Thu May  8 09:36:46 2008
#
begin active link
   name           : Remedy_HelloWorld
   timestamp      : 1210249958
   export-version : 9
   owner          : Demo
   last-changed   : Demo
   actlink-order  : 0
   wk-conn-type   : 1
   schema-name    : _1
   actlink-mask   : 16
   enable         : 1
   permission     : 0
   action {
      message-type: 0
      message-num : 10000
      message-pane: 1
      message-text: ",
                    output_for_hello_world!(),
                    r"!
   }
   object-prop    : 2\60016\4\1\0\60017\4\1\0\
end"
                ),
            )],
        }],
    };
