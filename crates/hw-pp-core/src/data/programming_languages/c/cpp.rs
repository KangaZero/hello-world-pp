//! Hello-world snippets for `ProgrammingLanguageName::Cpp`.
//!
//! Sources:
//! - <https://en.cppreference.com/cpp/io/cout>
//! - <https://en.cppreference.com/cpp/language/main_function>
//! - <https://en.cppreference.com/cpp/20>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28.NET%26nbsp%3BCLI%29.txt>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(.NET%C2%A0CLI)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28Epoc%29.txt>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(Epoc)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28FLTK%29.txt>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(FLTK)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28Gtk%2B%2B%29.txt>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(Gtk%2B%2B)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28ISO%29.htm>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(ISO)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28MFC%29>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(MFC)>
//! - <https://gitlab.com/wolframroesler/helloworld/-/blob/8e4c09528f95fde5c00813ae6669dfdebd31fec3/files/C%2B%2B%26nbsp%3B%28Qt%29.txt>
//! - <http://helloworldcollection.de/#C%2B%2B%C2%A0(Qt)>
//!
//! The `C++ (.NET CLI)`, `C++ (Epoc)`, `C++ (FLTK)`, `C++ (Gtk++)`, `C++ (ISO)`, `C++ (MFC)`,
//! `C++ (Qt)` versions are imported from the Hello World Collection, MIT licensed; see
//! `programming_languages/LICENSE-helloworldcollection`.

use crate::data::programming_languages::{
    FileExtension, ProgrammingLanguage,
    ProgrammingLanguageCategory::Compiled,
    ProgrammingLanguageDialect,
    ProgrammingLanguageFamily::{Algol, C, Cpp},
    ProgrammingLanguageName, ProgrammingLanguageVersion,
};
use crate::data::{
    macros::{language_names, output_for_hello_world},
    programming_languages_token::TokenKind,
};

language_names! {
    language: Cpp => "C++",
    versions: [Cpp98 => "C++98", Cpp20 => "C++20", CPlusPlusNetCli => "C++ (.NET CLI)", CPlusPlusEpoc => "C++ (Epoc)", CPlusPlusFltk => "C++ (FLTK)", CPlusPlusGtkPlusPlus => "C++ (Gtk++)", CPlusPlusIso => "C++ (ISO)", CPlusPlusMfc => "C++ (MFC)", CPlusPlusQt => "C++ (Qt)"],
    extensions: [Cpp => "cpp", Cc => "cc", Cxx => "cxx", Hpp => "hpp"],
}

pub(in crate::data::programming_languages) const LANGUAGE: ProgrammingLanguage =
    ProgrammingLanguage {
        name: ProgrammingLanguageName::Cpp,
        extensions: &[
            FileExtension::Cpp,
            FileExtension::Cc,
            FileExtension::Cxx,
            FileExtension::Hpp,
        ],
        categories: &[Compiled],
        families: &[Algol, C, Cpp],
        versions: &[
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Cpp98,
                syntax: concat!(
                    r#"#include <iostream>

int main() {
    std::cout << ""#,
                    output_for_hello_world!(),
                    r#"" << std::endl;
    return 0;
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "#include"),
                    (TokenKind::Plain, " "),
                    (TokenKind::String, "<iostream>"),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Type, "int"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "cout "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), "\""),
                    ),
                    (TokenKind::Plain, " "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "endl"),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n    "),
                    (TokenKind::Keyword, "return"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Number, "0"),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::Cpp20,
                syntax: concat!(
                    r#"#include <iostream>

int main() {
    std::cout << ""#,
                    output_for_hello_world!(),
                    r#"\n";
}"#
                ),
                tokenized_syntax: &[
                    (TokenKind::Keyword, "#include"),
                    (TokenKind::Plain, " "),
                    (TokenKind::String, "<iostream>"),
                    (TokenKind::Plain, "\n\n"),
                    (TokenKind::Type, "int"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Function, "main"),
                    (TokenKind::Punctuation, "()"),
                    (TokenKind::Plain, " "),
                    (TokenKind::Punctuation, "{"),
                    (TokenKind::Plain, "\n    std"),
                    (TokenKind::Punctuation, "::"),
                    (TokenKind::Plain, "cout "),
                    (TokenKind::Keyword, "<<"),
                    (TokenKind::Plain, " "),
                    (
                        TokenKind::String,
                        concat!("\"", output_for_hello_world!(), r#"\n""#),
                    ),
                    (TokenKind::Punctuation, ";"),
                    (TokenKind::Plain, "\n"),
                    (TokenKind::Punctuation, "}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusNetCli,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++/CLI for .NET

using namespace System;

void main()
{
    Console::WriteLine(""#,
                    output_for_hello_world!(),
                    r#"");
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++/CLI for .NET
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"using"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"namespace"),
                    (TokenKind::Plain, r" System"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Function, r"Console"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"WriteLine"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusEpoc,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++, Epoc style (for Symbian OS)

#include <eikapp.h>
#include <eikdoc.h>
#include <eikappui.h>

class CHelloWorldAppUi;
class CEikApplication;
class CHelloWorldAppView;

class CHelloWorldApplication : public CEikApplication
    {
        public:
            TUid AppDllUid() const;
        protected:
            CApaDocument* CreateDocumentL();
    };

class CHelloWorldDocument : public CEikDocument
    {
        public:
            static CHelloWorldDocument* NewL(CEikApplication& aApp);
            static CHelloWorldDocument* NewLC(CEikApplication& aApp);
            ~CHelloWorldDocument(){};
        public:
            CEikAppUi* CreateAppUiL();
        private:
            void ConstructL() {};
            CHelloWorldDocument(CEikApplication& aApp){};
    };

class CHelloWorldAppUi : public CEikAppUi
    {
        public:
                void ConstructL();
                CHelloWorldAppUi(){};
                ~CHelloWorldAppUi(){};
    };

static const TUid KUidHelloWorldApp = {0x10005B91};

GLDEF_C TInt E32Dll(TDllReason )
    {
    return KErrNone;
    }

EXPORT_C CApaApplication* NewApplication() 
    {
    return (new CHelloWorldApplication);
    }

CApaDocument* CHelloWorldApplication::CreateDocumentL()
    {  
    CApaDocument* document = CHelloWorldDocument::NewL(*this);
    return document;
    }

TUid CHelloWorldApplication::AppDllUid() const
    {
    return KUidHelloWorldApp;
    }
    
CHelloWorldDocument* CHelloWorldDocument::NewL(CEikApplication& aApp)
    {
    CHelloWorldDocument* self = NewLC(aApp);
    CleanupStack::Pop(self);
    return self;
    }

CHelloWorldDocument* CHelloWorldDocument::NewLC(CEikApplication& aApp)
    {
    CHelloWorldDocument* self = new (ELeave) CHelloWorldDocument(aApp);
    CleanupStack::PushL(self);
    self->ConstructL();
    return self;
    }

CEikAppUi* CHelloWorldDocument::CreateAppUiL()
    {
    CEikAppUi* appUi = new (ELeave) CHelloWorldAppUi;
    return appUi;
    }

void CHelloWorldAppUi::ConstructL()
    {
    BaseConstructL();

    _LIT(message,"Hello!");
    CAknInformationNote* informationNote = new (ELeave) CAknInformationNote;
    informationNote->ExecuteLD(message);
    }"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++, Epoc style (for Symbian OS)
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<eikapp.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<eikdoc.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<eikappui.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloWorldAppUi"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CEikApplication"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloWorldAppView"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloWorldApplication "),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (
                        TokenKind::Plain,
                        r" CEikApplication
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
            TUid ",
                    ),
                    (TokenKind::Function, r"AppDllUid"),
                    (TokenKind::Punctuation, r"()"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"const"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"protected"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
            CApaDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CreateDocumentL"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloWorldDocument "),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (
                        TokenKind::Plain,
                        r" CEikDocument
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
            ",
                    ),
                    (TokenKind::Keyword, r"static"),
                    (TokenKind::Plain, r" CHelloWorldDocument"),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"NewL"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"CEikApplication"),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r" aApp"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
            ",
                    ),
                    (TokenKind::Keyword, r"static"),
                    (TokenKind::Plain, r" CHelloWorldDocument"),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"NewLC"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"CEikApplication"),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r" aApp"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
            ",
                    ),
                    (TokenKind::Function, r"~CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"(){};"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
            CEikAppUi",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CreateAppUiL"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"private"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
            ",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"ConstructL"),
                    (TokenKind::Punctuation, r"()"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{};"),
                    (
                        TokenKind::Plain,
                        r"
            ",
                    ),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"CEikApplication"),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r" aApp"),
                    (TokenKind::Punctuation, r"){};"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloWorldAppUi "),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (
                        TokenKind::Plain,
                        r" CEikAppUi
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
                ",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"ConstructL"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
                ",
                    ),
                    (TokenKind::Function, r"CHelloWorldAppUi"),
                    (TokenKind::Punctuation, r"(){};"),
                    (
                        TokenKind::Plain,
                        r"
                ",
                    ),
                    (TokenKind::Function, r"~CHelloWorldAppUi"),
                    (TokenKind::Punctuation, r"(){};"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Keyword, r"static"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"const"),
                    (TokenKind::Plain, r" TUid KUidHelloWorldApp "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (TokenKind::Number, r"0x10005B91"),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

GLDEF_C TInt ",
                    ),
                    (TokenKind::Function, r"E32Dll"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"TDllReason "),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" KErrNone"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

EXPORT_C CApaApplication",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"NewApplication"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r" 
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" CHelloWorldApplication"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

CApaDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldApplication"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"CreateDocumentL"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"  
    CApaDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" document "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"NewL"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"this"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" document"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

TUid ",
                    ),
                    (TokenKind::Function, r"CHelloWorldApplication"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"AppDllUid"),
                    (TokenKind::Punctuation, r"()"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"const"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" KUidHelloWorldApp"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"
    
CHelloWorldDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"NewL"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"CEikApplication"),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r" aApp"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    CHelloWorldDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" self "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"NewLC"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"aApp"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Function, r"CleanupStack"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"Pop"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"self"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" self"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

CHelloWorldDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"NewLC"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"CEikApplication"),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r" aApp"),
                    (TokenKind::Punctuation, r")"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    CHelloWorldDocument",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" self "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"ELeave"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"aApp"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Function, r"CleanupStack"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"PushL"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"self"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    self",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"ConstructL"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" self"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

CEikAppUi",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldDocument"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"CreateAppUiL"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    CEikAppUi",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" appUi "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"ELeave"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" CHelloWorldAppUi"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" appUi"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloWorldAppUi"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"ConstructL"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Function, r"BaseConstructL"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"

    ",
                    ),
                    (TokenKind::Function, r"_LIT"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"message"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::String, r#""Hello!""#),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    CAknInformationNote",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r" informationNote "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"ELeave"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" CAknInformationNote"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
    informationNote",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"ExecuteLD"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"message"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusFltk,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++-FLTK

#include <FL/Fl.H>
#include <FL/Fl_Window.H>
#include <FL/Fl_Box.H>

int main(int argc, char **argv) {
   Fl_Window *ventana = new Fl_Window(300,180);
   ventana->begin();
   Fl_Box *box = new Fl_Box(20,40,260,100,""#,
                    output_for_hello_world!(),
                    r#"");
   box->labelsize(50);
   ventana->end();
   ventana->show(argc, argv);
   return Fl::run();
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++-FLTK
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<FL/Fl.H>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<FL/Fl_Window.H>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<FL/Fl_Box.H>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"**"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r")"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
   Fl_Window ",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"ventana "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"Fl_Window"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"300"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"180"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ventana",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"begin"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
   Fl_Box ",
                    ),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"box "),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"Fl_Box"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"20"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"40"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"260"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Number, r"100"),
                    (TokenKind::Punctuation, r","),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   box",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"labelsize"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"50"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ventana",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"end"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
   ventana",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"show"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" argv"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
   ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"Fl"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"run"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusGtkPlusPlus,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++ for the Gtk+ toolkit

#include <gtkmm/main.h>
#include <gtkmm/button.h>
#include <gtkmm/window.h>
#include <iostream>

void button_clicked()
{
	std::cout << ""#,
                    output_for_hello_world!(),
                    r#" !" << std::endl;
}

int main (int argc, char *argv[])
{
	Gtk::Main kit(argc, argv);
	Gtk::Window hello_window;
	Gtk::Button hello_button(""#,
                    output_for_hello_world!(),
                    r#"");
	
	hello_window.set_border_width(10);
	hello_window.add(hello_button);
	hello_button.signal_clicked().connect(sigc::ptr_fun(&button_clicked));
	hello_button.show();
	
	Gtk::Main::run(hello_window);
	return 0;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++ for the Gtk+ toolkit
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<gtkmm/main.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<gtkmm/button.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<gtkmm/window.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<iostream>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"void"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"button_clicked"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
	std",
                    ),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"cout "),
                    (TokenKind::Keyword, r"<<"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#" !""#),
                    ),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"<<"),
                    (TokenKind::Plain, r" std"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"endl"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main "),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r"[])"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
	Gtk",
                    ),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"Main "),
                    (TokenKind::Function, r"kit"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" argv"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
	Gtk",
                    ),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"Window hello_window"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
	Gtk",
                    ),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"Button "),
                    (TokenKind::Function, r"hello_button"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
	
	hello_window",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"set_border_width"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"10"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
	hello_window",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"add"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"hello_button"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
	hello_button",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"signal_clicked"),
                    (TokenKind::Punctuation, r"()."),
                    (TokenKind::Function, r"connect"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Function, r"sigc"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"ptr_fun"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"button_clicked"),
                    (TokenKind::Punctuation, r"));"),
                    (
                        TokenKind::Plain,
                        r"
	hello_button",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"show"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
	
	",
                    ),
                    (TokenKind::Function, r"Gtk"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"Main"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"run"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"hello_window"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
	",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusIso,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in ISO C++

#include <iostream>

int main()
{
    std::cout << ""#,
                    output_for_hello_world!(),
                    r#"" << std::endl;
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in ISO C++
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<iostream>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    std",
                    ),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"cout "),
                    (TokenKind::Keyword, r"<<"),
                    (TokenKind::Plain, r" "),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"<<"),
                    (TokenKind::Plain, r" std"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"endl"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusMfc,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++ for Microsoft Foundation Classes
// (Microsoft Visual C++).

#include <afxwin.h>

class CHello : public CFrameWnd
{
public:
    CHello()
    {
        Create(NULL,_T(""#,
                    output_for_hello_world!(),
                    r#""),WS_OVERLAPPEDWINDOW,rectDefault);
    }
};

class CHelloApp : public CWinApp
{
public:
    virtual BOOL InitInstance();
};

BOOL CHelloApp::InitInstance()
{
    m_pMainWnd = new CHello();
    m_pMainWnd->ShowWindow(m_nCmdShow);
    m_pMainWnd->UpdateWindow();
    return TRUE;
}

CHelloApp theApp;"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++ for Microsoft Foundation Classes
// (Microsoft Visual C++).
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<afxwin.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHello "),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (
                        TokenKind::Plain,
                        r" CFrameWnd
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Function, r"CHello"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
        ",
                    ),
                    (TokenKind::Function, r"Create"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"NULL"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Function, r"_T"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r"),"),
                    (TokenKind::Plain, r"WS_OVERLAPPEDWINDOW"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r"rectDefault"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"class"),
                    (TokenKind::Plain, r" CHelloApp "),
                    (TokenKind::Punctuation, r":"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"public"),
                    (
                        TokenKind::Plain,
                        r" CWinApp
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"public"),
                    (TokenKind::Punctuation, r":"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"virtual"),
                    (TokenKind::Plain, r" BOOL "),
                    (TokenKind::Function, r"InitInstance"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"};"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"BOOL"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHelloApp"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Function, r"InitInstance"),
                    (TokenKind::Punctuation, r"()"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
    m_pMainWnd ",
                    ),
                    (TokenKind::Keyword, r"="),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"new"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"CHello"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
    m_pMainWnd",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"ShowWindow"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"m_nCmdShow"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
    m_pMainWnd",
                    ),
                    (TokenKind::Punctuation, r"->"),
                    (TokenKind::Function, r"UpdateWindow"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
    ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Plain, r" TRUE"),
                    (TokenKind::Punctuation, r";"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                    (
                        TokenKind::Plain,
                        r"

CHelloApp theApp",
                    ),
                    (TokenKind::Punctuation, r";"),
                ],
            },
            ProgrammingLanguageVersion {
                label: ProgrammingLanguageDialect::CPlusPlusQt,
                syntax: concat!(
                    r"// ",
                    output_for_hello_world!(),
                    r#" in C++ for the Qt framework

#include <qapplication.h>
#include <qlabel.h>

int main(int argc, char *argv[])
{
  QApplication a(argc, argv);
  QLabel l(""#,
                    output_for_hello_world!(),
                    r#"", 0);
  l.setCaption("Test");
  l.setAlignment(Qt::AlignCenter);
  l.resize(300, 200);
  a.setMainWidget(&l);
  l.show();
  return(a.exec());
}"#
                ),
                tokenized_syntax: &[
                    (
                        TokenKind::Comment,
                        concat!(
                            r"// ",
                            output_for_hello_world!(),
                            r" in C++ for the Qt framework
"
                        ),
                    ),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<qapplication.h>"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Keyword, r"#include"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::String, r"<qlabel.h>"),
                    (
                        TokenKind::Plain,
                        r"

",
                    ),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Function, r"main"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Type, r"int"),
                    (TokenKind::Plain, r" argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Type, r"char"),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Keyword, r"*"),
                    (TokenKind::Plain, r"argv"),
                    (TokenKind::Punctuation, r"[])"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"{"),
                    (
                        TokenKind::Plain,
                        r"
  QApplication ",
                    ),
                    (TokenKind::Function, r"a"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"argc"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" argv"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  QLabel ",
                    ),
                    (TokenKind::Function, r"l"),
                    (TokenKind::Punctuation, r"("),
                    (
                        TokenKind::String,
                        concat!(r#"""#, output_for_hello_world!(), r#"""#),
                    ),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"0"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  l",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"setCaption"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::String, r#""Test""#),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  l",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"setAlignment"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"Qt"),
                    (TokenKind::Punctuation, r"::"),
                    (TokenKind::Plain, r"AlignCenter"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  l",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"resize"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Number, r"300"),
                    (TokenKind::Punctuation, r","),
                    (TokenKind::Plain, r" "),
                    (TokenKind::Number, r"200"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  a",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"setMainWidget"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Keyword, r"&"),
                    (TokenKind::Plain, r"l"),
                    (TokenKind::Punctuation, r");"),
                    (
                        TokenKind::Plain,
                        r"
  l",
                    ),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"show"),
                    (TokenKind::Punctuation, r"();"),
                    (
                        TokenKind::Plain,
                        r"
  ",
                    ),
                    (TokenKind::Keyword, r"return"),
                    (TokenKind::Punctuation, r"("),
                    (TokenKind::Plain, r"a"),
                    (TokenKind::Punctuation, r"."),
                    (TokenKind::Function, r"exec"),
                    (TokenKind::Punctuation, r"());"),
                    (
                        TokenKind::Plain,
                        r"
",
                    ),
                    (TokenKind::Punctuation, r"}"),
                ],
            },
        ],
    };
