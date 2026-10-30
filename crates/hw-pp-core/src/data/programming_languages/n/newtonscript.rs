//! Hello-world snippet for `ProgrammingLanguageName::Newtonscript`.
//!
//! Imported from leachim6/hello-world (`n/NewtonScript.nwt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/n/NewtonScript.nwt>
//! - <https://www.wikidata.org/wiki/Q1577753> (paradigms)

use crate::data::programming_languages::ProgrammingParadigm::{ObjectOriented, PrototypeBased};
use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Historical,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::programming_languages_token::{TokenKind, checked_token_lengths};

use hw_pp_macros::{language_names, output_for_hello_world};

language_names! {
    language: Newtonscript => "NewtonScript",
    versions: [Newtonscript => "NewtonScript"],
    extensions: [Nwt => "nwt"],
}

const NEWTONSCRIPT: &str = concat!(
    r#"baseview :=
   {viewBounds: {left: -3, top: 71, right: 138, bottom: 137},
    viewFlags: 581,
    declareSelf: 'base,
    _proto: protoFloatNGo,
    debug: "baseview"
   };

textview := * child of baseview *
   {text: ""#,
    output_for_hello_world!(),
    r#"",
    viewBounds: {left: 33, top: 24, right: 113, bottom: 46},
    viewFlags: 579,
    _proto: protoStaticText,
    debug: "textview"
   };"#
);

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Newtonscript,
        extensions: &[FileExtension::Nwt],
        categories: &[Historical],
        families: &[],
        paradigms: &[ObjectOriented, PrototypeBased],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Newtonscript,
            syntax: NEWTONSCRIPT,
            tokenized_syntax: &const {
                checked_token_lengths(
                    NEWTONSCRIPT,
                    [(
                        TokenKind::Plain,
                        concat!(
                            r#"baseview :=
   {viewBounds: {left: -3, top: 71, right: 138, bottom: 137},
    viewFlags: 581,
    declareSelf: 'base,
    _proto: protoFloatNGo,
    debug: "baseview"
   };

textview := * child of baseview *
   {text: ""#,
                            output_for_hello_world!(),
                            r#"",
    viewBounds: {left: 33, top: 24, right: 113, bottom: 46},
    viewFlags: 579,
    _proto: protoStaticText,
    debug: "textview"
   };"#
                        ),
                    )],
                )
            },
        }],
    };
