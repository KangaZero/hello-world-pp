//! Hello-world snippet for `ProgrammingLanguageName::Sather`.
//!
//! Imported from leachim6/hello-world (`s/Sather.sa` at `a152253`),
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
    language: Sather => "Sather",
    versions: [Sather => "Sather"],
    extensions: [Sa => "sa"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Sather,
        extensions: &[FileExtension::Sa],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Sather,
            syntax: concat!(
                r#"class MAIN is
   main is 
      loop 
         #OUT + ""#,
                output_for_hello_world!(),
                r#"\n" 
      end 
   end
end"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"class MAIN is
   main is 
      loop 
         #OUT + ""#,
                    output_for_hello_world!(),
                    r#"\n" 
      end 
   end
end"#
                ),
            )],
        }],
    };
