//! Hello-world snippet for `ProgrammingLanguageName::Smil`.
//!
//! Imported from leachim6/hello-world (`s/Smil.smil` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageDialect, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{macros::language_names, programming_languages_token::TokenKind};

language_names! {
    language: Smil => "Smil",
    versions: [Smil => "Smil"],
    extensions: [Smil => "smil"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Smil,
        extensions: &[FileExtension::Smil],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Smil,
            syntax: r#"<smil>
 <head>
  <layout>
   <root-layout width="300" height="160" background-color="white"/>
   <region id="text_region" left="115" top="60"/>
  </layout>
 </head>
 <body>
  <text src="data:,Hello%20World!" region="text_region">
   <param name="fontFace" value="Arial"/>
  </text>
 </body>
</smil>"#,
            tokenized_syntax: &[(
                TokenKind::Plain,
                r#"<smil>
 <head>
  <layout>
   <root-layout width="300" height="160" background-color="white"/>
   <region id="text_region" left="115" top="60"/>
  </layout>
 </head>
 <body>
  <text src="data:,Hello%20World!" region="text_region">
   <param name="fontFace" value="Arial"/>
  </text>
 </body>
</smil>"#,
            )],
        }],
    };
