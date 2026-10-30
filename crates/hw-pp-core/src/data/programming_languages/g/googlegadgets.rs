//! Hello-world snippet for `ProgrammingLanguageName::Googlegadgets`.
//!
//! Imported from the Hello World Collection (`files/GoogleGadgets.txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/GoogleGadgets.txt>
//! - <http://helloworldcollection.de/#GoogleGadgets>

use crate::data::programming_languages::{
    ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Googlegadgets => "GoogleGadgets",
    versions: [Googlegadgets => "GoogleGadgets"],
    extensions: [],
}

const GOOGLEGADGETS: &str = concat!(
    r#"<?xml version="1.0" encoding="UTF-8" ?> 
<!-- "#,
    output_for_hello_world!(),
    r#" as a Google gadget -->

<Module>
  <ModulePrefs title=""#,
    output_for_hello_world!(),
    r#" example" /> 
  <Content type="html">
     <![CDATA[ 
       "#,
    output_for_hello_world!(),
    r"
     ]]>
  </Content> 
</Module>"
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Googlegadgets,
        extensions: &[],
        categories: &[],
        families: &[],
        paradigms: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Googlegadgets,
            syntax: GOOGLEGADGETS,
            tokenized_syntax: &const {
                checked_token_lengths(
                    GOOGLEGADGETS,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"<?xml version="1.0" encoding="UTF-8" ?> 
<!-- "#,
                            output_for_hello_world!(),
                            r#" as a Google gadget -->

<Module>
  <ModulePrefs title=""#,
                            output_for_hello_world!(),
                            r#" example" /> 
  <Content type="html">
     <![CDATA[ 
       "#,
                            output_for_hello_world!(),
                            r"
     ]]>
  </Content> 
</Module>"
                        ),
                    )],
                )
            },
        }],
    };
