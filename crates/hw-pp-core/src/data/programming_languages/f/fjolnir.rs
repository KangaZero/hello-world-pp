//! Hello-world snippet for `ProgrammingLanguageName::Fjolnir`.
//!
//! Imported from the Hello World Collection (`files/Fj&ouml;lnir.htm` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Fj%26ouml%3Blnir.htm>
//! - <http://helloworldcollection.de/#Fj%C3%B6lnir>
//! - <https://www.wikidata.org/wiki/Q3481158> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, Procedural};
use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Fjolnir => "Fjölnir",
    versions: [Fjolnir => "Fjölnir"],
    extensions: [],
}

const FJOLNIR: &str = concat!(
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
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Fjolnir,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[Procedural, ObjectOriented],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Fjolnir,
            syntax: FJOLNIR,
            tokenized_syntax: &const {
                checked_token_lengths(
                    FJOLNIR,
                    [(
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
                )
            },
        }],
    };
