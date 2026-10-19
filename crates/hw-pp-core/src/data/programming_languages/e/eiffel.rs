//! Hello-world snippet for `ProgrammingLanguageName::Eiffel`.
//!
//! Imported from leachim6/hello-world (`e/Eiffel.eiff` at `a152253`),
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
    language: Eiffel => "Eiffel",
    versions: [Eiffel => "Eiffel"],
    extensions: [Eiff => "eiff"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Eiffel,
        extensions: &[FileExtension::Eiff],
        categories: &[],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Eiffel,
            syntax: concat!(
                r#"indexing ""#,
                output_for_hello_world!(),
                r#" in Eiffel , from http://roesler-ac.de/wolfram/hello.htm#Eiffel"

class HELLO

creation
	run

feature

	run is
		local
			io : BASIC_IO;
		do
			!!io;
			io.put_string(""#,
                output_for_hello_world!(),
                r#"");
			io.put_newline
		end; -- run
end; -- class HELLO"#
            ),
            tokenized_syntax: &[(
                TokenKind::Plain,
                concat!(
                    r#"indexing ""#,
                    output_for_hello_world!(),
                    r#" in Eiffel , from http://roesler-ac.de/wolfram/hello.htm#Eiffel"

class HELLO

creation
	run

feature

	run is
		local
			io : BASIC_IO;
		do
			!!io;
			io.put_string(""#,
                    output_for_hello_world!(),
                    r#"");
			io.put_newline
		end; -- run
end; -- class HELLO"#
                ),
            )],
        }],
    };
