//! Hello-world snippet for `ProgrammingLanguageName::Newtonscript`.
//!
//! Imported from leachim6/hello-world (`n/NewtonScript.nwt` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Newtonscript => "NewtonScript",
    versions: [Newtonscript => "NewtonScript"],
    extensions: [Nwt => "nwt"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Newtonscript,
        extensions: &[FileExtension::Nwt],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Newtonscript,
            syntax: concat!(
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
            tokenized_syntax: &[(
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
        }],
    };
