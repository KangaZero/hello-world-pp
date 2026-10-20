//! Hello-world snippet for `ProgrammingLanguageName::Fjolnir`.
//!
//! Imported from the Hello World Collection (`files/Fj&ouml;lnir.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Fj%26ouml%3Blnir.htm>
//! - <http://helloworldcollection.de/#Fj%C3%B6lnir>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Fjolnir => "Fjölnir",
    versions: [Fjolnir => "Fjölnir"],
    extensions: [],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fjolnir,
        extensions: &[],
        categories: &[],
        families: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fjolnir,
            syntax: concat!(
                r";; ",
                output_for_hello_world!(),
                r#" in Fjölnir (Icelandic programming language)

"hello" < main
{
   main ->
   stef(;)
   stofn
       skrifastreng(;"Halló Veröld!"),
   stofnlok
}
*
"GRUNNUR"
;"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r";; ",
                    output_for_hello_world!(),
                    r#" in Fjölnir (Icelandic programming language)

"hello" < main
{
   main ->
   stef(;)
   stofn
       skrifastreng(;"Halló Veröld!"),
   stofnlok
}
*
"GRUNNUR"
;"#
                ),
            )],
        }],
    };
