//! Hello-world snippet for `ProgrammingLanguageName::Actionscript`.
//!
//! Imported from leachim6/hello-world (`a/ActionScript.as` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ActionScript.as>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Actionscript => "ActionScript",
    versions: [Actionscript => "ActionScript"],
    extensions: [As => "as"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Actionscript,
        extensions: &[FileExtension::As],
        categories: &[Scripting],
        versions: &[ProgrammingLanguageVersion {
            label: ProgrammingLanguageDialect::Actionscript,
            syntax: concat!(
                r#"package 
{
	import flash.display.Sprite;
	import flash.text.TextField;
	
	public class actionscript extends Sprite
	{
		private var hello:TextField = new TextField();
		
		public function actionscript(){
			hello.text = ""#,
                output_for_hello_world!(),
                r#"";
			addChild(hello);
		}
	}
}"#
            ),
            tokenized_syntax: &[
                (
                    TokenKind::Plain,
                    r"package 
{
	",
                ),
                (TokenKind::Keyword, r"import"),
                (
                    TokenKind::Plain,
                    r" flash.display.Sprite;
	",
                ),
                (TokenKind::Keyword, r"import"),
                (TokenKind::Plain, r" flash."),
                (TokenKind::Function, r"text"),
                (
                    TokenKind::Plain,
                    r".TextField;
	
	",
                ),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"class"),
                (TokenKind::Plain, r" "),
                (TokenKind::Type, r"actionscript"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"extends"),
                (
                    TokenKind::Plain,
                    r" Sprite
	{
		",
                ),
                (TokenKind::Keyword, r"private"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"var"),
                (TokenKind::Plain, r" hello"),
                (TokenKind::Keyword, r":"),
                (TokenKind::Plain, r"TextField "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"new"),
                (
                    TokenKind::Plain,
                    r" TextField();
		
		",
                ),
                (TokenKind::Keyword, r"public"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"function"),
                (TokenKind::Plain, r" "),
                (TokenKind::Function, r"actionscript"),
                (TokenKind::Punctuation, r"()"),
                (
                    TokenKind::Plain,
                    r"{
			hello.",
                ),
                (TokenKind::Function, r"text"),
                (TokenKind::Plain, r" "),
                (TokenKind::Keyword, r"="),
                (TokenKind::Plain, r" "),
                (
                    TokenKind::String,
                    concat!(r#"""#, output_for_hello_world!(), r#"""#),
                ),
                (
                    TokenKind::Plain,
                    r";
			addChild(hello);
		}
	}
}",
                ),
            ],
        }],
    };
