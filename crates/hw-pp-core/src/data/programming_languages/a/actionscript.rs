//! Hello-world snippets for `ProgrammingLanguageName::Actionscript`.
//!
//! Imported from leachim6/hello-world (`a/ActionScript.as` at `a152253`),
//! MIT licensed; see `programming_languages/LICENSE-leachim6-hello-world`.
//!
//! Imported from the Hello World Collection (`files/ActionScript&nbsp;(Flash&nbsp;8).txt`, `files/ActionScript&nbsp;3.0.htm`, `files/Actionscript&nbsp;(Flash&nbsp;5).txt` at `8e4c095`),
//! MIT licensed; see `programming_languages/LICENSE-helloworldcollection`.
//!
//! Sources:
//! - <https://github.com/leachim6/hello-world/blob/a152253b102999bf31988fef077101022e2ad1a5/a/ActionScript.as>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ActionScript%26nbsp%3B%28Flash%26nbsp%3B8%29.txt>
//! - <http://helloworldcollection.de/#ActionScript%C2%A0(Flash%C2%A08)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/ActionScript%26nbsp%3B3.0.htm>
//! - <http://helloworldcollection.de/#ActionScript%C2%A03.0>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/Actionscript%26nbsp%3B%28Flash%26nbsp%3B5%29.txt>
//! - <http://helloworldcollection.de/#Actionscript%C2%A0(Flash%C2%A05)>

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage, ProgrammingLanguageCategory::Scripting,
    ProgrammingLanguageDialect, ProgrammingLanguageFamily::JavaScript, ProgrammingLanguageName,
    ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Actionscript => "ActionScript",
    versions: [Actionscript => "ActionScript", ActionscriptFlash8 => "ActionScript (Flash 8)", Actionscript30 => "ActionScript 3.0", ActionscriptFlash5 => "Actionscript (Flash 5)"],
    extensions: [As => "as"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Actionscript,
        extensions: &[FileExtension::As],
        categories: &[Scripting],
        families: &[JavaScript],
        versions: &[
            ProgrammingLanguageVersion {
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
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::ActionscriptFlash8,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r##" in ActionScript 2.0 (Flash 8)
class HelloWorld
{
    private var helloWorldField:TextField;
 
    public function HelloWorld( mc:MovieClip )
    {
        mc.helloWorldField = mc.createTextField("helloWorldField", mc.getNextHighestDepth(), 0, 0, 100, 100);
        mc.helloWorldField.autoSize = "left";
	mc.helloWorldField.html = true;
        mc.helloWorldField.htmlText = '<font size="20" color="#0000FF">"##,
                    output_for_hello_world!(),
                    r"</font>';
    }
}

// on a frame
import HelloWorld;
var hw:HelloWorld = new HelloWorld( this );"
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in ActionScript 2.0 (Flash 8)
"
                        ),
                    ),
                    (TokenKind::Keyword, r"class"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"HelloWorld"),
                    (
                        TokenKind::Plain,
                        r"
{
    ",
                    ),
                    (TokenKind::Keyword, r"private"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"var"),
                    (TokenKind::Plain, r" helloWorldField"),
                    (TokenKind::Keyword, r":"),
                    (
                        TokenKind::Plain,
                        r"TextField;
 
    ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"function"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"HelloWorld"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r" mc:MovieClip "),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
    {
        mc.helloWorldField ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" mc."),
                    (TokenKind::Function, r"createTextField"),
                    (TokenKind::Plain, r"("),
                    (TokenKind::String, r#""helloWorldField""#),
                    (TokenKind::Plain, r", mc."),
                    (TokenKind::Function, r"getNextHighestDepth"),
                    (TokenKind::Plain, r"(), "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Plain, r", "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Plain, r", "),
                    (TokenKind::Number, r"100"),
                    (TokenKind::Plain, r", "),
                    (TokenKind::Number, r"100"),
                    (
                        TokenKind::Plain,
                        r");
        mc.helloWorldField.",
                    ),
                    (TokenKind::Function, r"autoSize"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r#""left""#),
                    (
                        TokenKind::Plain,
                        r";
	mc.helloWorldField.",
                    ),
                    (TokenKind::Function, r"html"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"="),
                    (
                        TokenKind::Plain,
                        r" true;
        mc.helloWorldField.",
                    ),
                    (TokenKind::Function, r"htmlText"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(
                            r##"'<font size="20" color="#0000FF">"##,
                            output_for_hello_world!(),
                            r"</font>'"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r";
    }
}

",
                    ),
                    (
                        TokenKind::Comment,
                        r"// on a frame
",
                    ),
                    (TokenKind::Keyword, r"import"),
                    (
                        TokenKind::Plain,
                        r" HelloWorld;
",
                    ),
                    (TokenKind::Keyword, r"var"),
                    (TokenKind::Plain, r" hw"),
                    (TokenKind::Keyword, r":"),
                    (TokenKind::Plain, r"HelloWorld "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" HelloWorld( "),
                    (TokenKind::Function, r"this"),
                    (TokenKind::Plain, r" );"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Actionscript30,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ActionScript 3. Place code in the first frame Actions.
var t:TextField=new TextField();
t.text=""#,
                    output_for_hello_world!(),
                    r#"";
addChild(t);"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in ActionScript 3. Place code in the first frame Actions.
"
                        ),
                    ),
                    (TokenKind::Keyword, r"var"),
                    (TokenKind::Plain, r" t"),
                    (TokenKind::Keyword, r":"),
                    (TokenKind::Plain, r"TextField"),
                    (TokenKind::Keyword, r"=new"),
                    (
                        TokenKind::Plain,
                        r" TextField();
t.",
                    ),
                    (TokenKind::Function, r"text"),
                    (TokenKind::Keyword, r"="),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (
                        TokenKind::Plain,
                        r";
addChild(t);",
                    ),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::ActionscriptFlash5,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in Actionscript (up to Flash 5, IDE only)

trace (""#,
                    output_for_hello_world!(),
                    r#"");"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in Actionscript (up to Flash 5, IDE only)
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Function, r"trace"),
                    (TokenKind::Plain, r" ("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Plain, r");"),
                ],
            },
        ],
    };
