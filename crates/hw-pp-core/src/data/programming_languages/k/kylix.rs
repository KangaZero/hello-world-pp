//! Hello-world snippet for `ProgrammingLanguageName::Kylix`.
//!
//! Imported from the Hello World Collection (`files/Kylix.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Kylix.txt>
//! - <http://helloworldcollection.de/#Kylix>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Kylix => "Kylix",
    versions: [Kylix => "Kylix"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Kylix,
        extensions: &[],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Kylix,
            syntax: concat!(
                r"{",
                output_for_hello_world!(),
                r" in Kylix} 

program Hello_World;
 
 uses
    QDialogs;
 
 begin
   ShowMessage('",
                output_for_hello_world!(),
                r"');
 end."
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r"{",
                    output_for_hello_world!(),
                    r" in Kylix} 

program Hello_World;
 
 uses
    QDialogs;
 
 begin
   ShowMessage('",
                    output_for_hello_world!(),
                    r"');
 end."
                ),
            )],
        }],
    };
