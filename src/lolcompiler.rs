use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;


/// Trait for a simple lolcompiler front-end.
/// Errors should cause immediate exit inside the implementation.
pub trait Compiler {
/// Begin the compilation process (entry point).
fn compile(&mut self, source: &str);
/// Get the next token from the lexical analyzer.
fn next_token(&mut self) -> String;
/// Run the syntax analyzer starting from <lolcode>.
fn parse(&mut self);
/// Get the current token being processed.
fn current_token(&self) -> String;
/// Set the current token (typically used internally).
fn set_current_token(&mut self, tok: String);
}


/// Trait for a simple lexical analyzer.
/// Implements a character-by-character analysis
/// from a state machine design.
pub trait LexicalAnalyzer {
/// Return the next character from the input.
/// If input is exhausted, should terminate the program.
fn get_char(&mut self) -> char;
/// Add a character to the current potential token.
fn add_char(&mut self, c: char);
/// Lookup a potential token to determine if it is valid.
/// Returns true if a valid token/lexeme, false otherwise.
fn lookup(&self, s: &str) -> bool;
}

/*

This is the structure for the lexical analyzer. I based mine off
the one used in assignment 5.

Each terminal symbol is represented by its own token, as per the 
given example ANTLR/BNF code.
*/
pub struct MarkdownLexicalAnalyzer {
    //input chars, current read pos, current token build,d and list of tokens
    input: Vec<char>,
    position: usize,
    current_build: String,
    pub tokens: Vec<String>, 
    //all the keywords for annotations
    pub hai_token: Vec<String>,
    pub bai_token: Vec<String>,
    pub obtw_token: Vec<String>,
    pub tldr_token: Vec<String>,
    pub maek_token: Vec<String>,
    pub oic_token: Vec<String>,
    pub gimmeh_token: Vec<String>,
    pub mkay_token: Vec<String>,
    pub head_token: Vec<String>,
    pub paragraf_token: Vec<String>,
    pub bold_token: Vec<String>,
    pub italics_token: Vec<String>,
    pub list_token: Vec<String>,
    pub item_token: Vec<String>,
    pub newline_token: Vec<String>,
    pub soundz_token: Vec<String>,
    pub vidz_token: Vec<String>,
    pub ihaz_token: Vec<String>,
    pub itiz_token: Vec<String>,
    pub lemmesee_token: Vec<String>,
    //these are mostly used as placeholders
    pub vardef: Vec<String>,
    pub varval: Vec<String>,
    pub text: Vec<String>,
    pub address: Vec<String>,
}

/* 

Implementation of the Markdown Lexical Analyzer, contains helper methods for token operatiosn.
Here, again similar to assignment #5, I made the constructor with the same format
Each keyword token is created into a Vec<String>
 */
impl MarkdownLexicalAnalyzer {
    pub fn new(source: &str) -> Self{
        Self {
            input: source.chars().collect(),
            position: 0,
            current_build: String::new(),
            tokens: Vec::new(),
            hai_token: vec!["#HAI".into()],
            bai_token: vec!["#KTHXBYE".into()],
            obtw_token: vec!["#OBTW".into()],
            tldr_token: vec!["#TLDR".into()],
            maek_token: vec!["#MAEK".into()],
            oic_token: vec!["#OIC".into()],
            gimmeh_token: vec!["#GIMMEH".into()],
            mkay_token: vec!["#MKAY".into()],
            head_token: vec!["HEAD".into()],
            paragraf_token: vec!["PARAGRAF".into()],
            bold_token: vec!["BOLD".into()],
            italics_token: vec!["ITALICS".into()],
            list_token: vec!["LIST".into()],
            item_token: vec!["ITEM".into()],
            newline_token: vec!["#GIMMEH NEWLINE".into()],
            soundz_token: vec!["SOUNDZ".into()],
            vidz_token: vec!["VIDZ".into()],
            ihaz_token: vec!["#I".into(), "HAZ".into()],
            itiz_token: vec!["#IT".into(), "IZ".into()],
            lemmesee_token: vec!["#LEMME".into(), "SEE".into()],
            vardef: Vec::new(),
            varval: Vec::new(),
            text: Vec::new(),
            address: Vec::new(),
        }
    }

    /*
      Tokenize reads source code char by char, and translates it into  actual tokens like #HAI, #MAEK, etc.
      It stores these tokens in a self.tokens for the parser to understand later.
      Also similar to assignment 5.
     */
    pub fn tokenize(&mut self) {
        //we're assuming get_char() returns a valid char here
        loop {
            let c = self.get_char();
            if c == '\0' {
                break;
            }

            if c.is_whitespace() {
                if !self.current_build.is_empty() {
                //finish token and push it
                let last_token = std::mem::take(&mut self.current_build);

                //checks to see if lexical error is detected
                //every # must be followed by a recognized keyword
                //we check for partial multi part tokens since they form valid annotations later like "#I HAZ"
               if last_token.starts_with('#') && !self.lookup(&last_token) && !["#I", "#IT", "#LEMME"].contains(&last_token.to_uppercase().as_str())
                {
                    eprintln!("Lexical error, '#' is misused in token {}", last_token);
                    std::process::exit(1);
                }

                //no error, then push it
                self.tokens.push(last_token);
            } 
            } else {
                //if not whitespace, just append character to current build
                self.add_char(c); 
            }
        }

        //after the loop, check one last time if token was being built
        if !self.current_build.is_empty() {
                let last_token = std::mem::take(&mut self.current_build);


                //another lexical error check
                if last_token.contains('#') && (!last_token.starts_with('#') || !self.lookup(&last_token)) {
                    eprintln!("Lexical error, '#' is misused in token {}", last_token);
                    std::process::exit(1);
                }

                self.tokens.push(last_token);
        }
        //then reverse the list with .reverse()
        self.tokens.reverse(); 
    }
}


/*
Implements required behavior from trait
Again, also similar to assignment 5. The funcs are reused and I use similar logic for lookup.
*/
impl LexicalAnalyzer for MarkdownLexicalAnalyzer {
    //reads 1 char at a time from the inputted string
    fn get_char(&mut self) -> char {
        if self.position < self.input.len() {
            let c = self.input[self.position]; 
            self.position +=1;
            c
        } else {
            '\0' //if we're at the end of len, return the null character
        }
    }
    
    //add_char builds a token character by character
    fn add_char(&mut self, c: char) {
        self.current_build.push(c); //adds character to current build
    }

    //what kind of token does the current lexeme/symbol represent
    fn lookup(&self, s: &str) -> bool {
        //removes # when necessary
        //ignores case-sensitivity
        let name = s.trim_start_matches('#');
           self.hai_token.iter().any(|h| h.eq_ignore_ascii_case(s))
        || self.bai_token.iter().any(|b| b.eq_ignore_ascii_case(s))
        || self.obtw_token.iter().any(|o| o.eq_ignore_ascii_case(s))
        || self.maek_token.iter().any(|m| m.eq_ignore_ascii_case(s))
        || self.oic_token.iter().any(|o| o.eq_ignore_ascii_case(s))
        || self.tldr_token.iter().any(|t| t.eq_ignore_ascii_case(s))
        || self.gimmeh_token.iter().any(|g| g.eq_ignore_ascii_case(s))
        || self.mkay_token.iter().any(|m| m.eq_ignore_ascii_case(s))
        || self.head_token.iter().any(|h| h.eq_ignore_ascii_case(name))
        || self.paragraf_token.iter().any(|p| p.eq_ignore_ascii_case(name))
        || self.bold_token.iter().any(|b| b.eq_ignore_ascii_case(name))
        || self.italics_token.iter().any(|i| i.eq_ignore_ascii_case(name))
        || self.list_token.iter().any(|l| l.eq_ignore_ascii_case(name))
        || self.item_token.iter().any(|i| i.eq_ignore_ascii_case(name))
        || self.newline_token.iter().any(|n| n.eq_ignore_ascii_case(name))
        || self.soundz_token.iter().any(|s2| s2.eq_ignore_ascii_case(name))
        || self.vidz_token.iter().any(|v| v.eq_ignore_ascii_case(name))
        || self.ihaz_token.iter().any(|i| i.eq_ignore_ascii_case(name))
        || self.itiz_token.iter().any(|i| i.eq_ignore_ascii_case(name))
        || self.lemmesee_token.iter().any(|l| l.eq_ignore_ascii_case(name))
    }
}



/// OPTION 1 - Trait for a recursive descent Syntax Analyzer
/// over Vec<String>. Each function parses a nonterminal in
/// the grammar. On error: exit immediately.
pub trait SyntaxAnalyzer {
    fn parse_lolcode(&mut self);
    fn parse_head(&mut self);
    fn parse_title(&mut self);
    fn parse_comment(&mut self);
    fn parse_body(&mut self);
    fn parse_paragraph(&mut self);
    fn parse_inner_paragraph(&mut self);
    fn parse_inner_text(&mut self);
    fn parse_variable_define(&mut self);
    fn parse_variable_use(&mut self);
    fn parse_bold(&mut self);
    fn parse_italics(&mut self);
    fn parse_list(&mut self);
    fn parse_list_items(&mut self);
    fn parse_inner_list(&mut self);
    fn parse_audio(&mut self);
    fn parse_video(&mut self);
    fn parse_newline(&mut self);
    fn parse_text(&mut self);
}

/*

Defining the struct for the syntax analyzer.
Takes in lexer, current_token, and defined_variable

 */
pub struct MarkdownSyntaxAnalyzer {
    pub lexer: MarkdownLexicalAnalyzer,
    current_token: String,
    //this is used for later static semantic error checking in the SemanticAnalyzer
    defined_variables: Vec<String>,
}

/*
    Methods for token recognition and navigation
    to be used by recusirve descent parser 
 */
impl MarkdownSyntaxAnalyzer {

    //constructor
    pub fn new() -> Self {
        Self {
            lexer: MarkdownLexicalAnalyzer::new(""),
            current_token: String::new(),
            defined_variables: Vec::new(),
        }
    }

    //pops next token from lexer’s token stack and updates current_token.
    //again, similar to the implementation of next_token in assignment 5, but i decided
    //move functionality into the parser instead of in the compiler
    fn next_token(&mut self) -> String{
        let candidate = self.lexer.tokens.pop().unwrap_or_default();
        if candidate.is_empty() {
            self.current_token.clear();
            return String::new();
        }
        
        //if token is valid (in lookup, not starting with #, or partial tag for var defintiosns and use)
        if self.lexer.lookup(&candidate) || (!candidate.starts_with('#') && !candidate.is_empty()) || ["#I", "#IT", "#LEMME"].iter().any(|kw| kw.eq_ignore_ascii_case(&candidate))
        {
            //store and return 
            self.current_token = candidate.clone();
            candidate
        } else {
            //otherwise throw lexical error
            eprintln!("Lexical error: '{}' is not a recognized token.", candidate);
            std::process::exit(1);
        }
    }

    //boolean recognizers 
    #[inline] fn is_hai(&self, s: &str) -> bool { self.lexer.hai_token.iter().any(|h| h.eq_ignore_ascii_case(s)) }
    #[inline] fn is_bai(&self, s: &str) -> bool { self.lexer.bai_token.iter().any(|b| b.eq_ignore_ascii_case(s)) }
    #[inline] fn is_obtw(&self, s: &str) -> bool { self.lexer.obtw_token.iter().any(|o| o.eq_ignore_ascii_case(s)) }
    #[inline] fn is_tldr(&self, s: &str) -> bool { self.lexer.tldr_token.iter().any(|t| t.eq_ignore_ascii_case(s)) }
    #[inline] fn is_maek(&self, s: &str) -> bool { self.lexer.maek_token.iter().any(|m| m.eq_ignore_ascii_case(s)) }
    #[inline] fn is_oic(&self, s: &str) -> bool { self.lexer.oic_token.iter().any(|o| o.eq_ignore_ascii_case(s)) }
    #[inline] fn is_gimmeh(&self, s: &str) -> bool { self.lexer.gimmeh_token.iter().any(|g| g.eq_ignore_ascii_case(s)) }
    #[inline] fn is_mkay(&self, s: &str) -> bool { self.lexer.mkay_token.iter().any(|m| m.eq_ignore_ascii_case(s)) }
    #[inline] fn is_head(&self, s: &str) -> bool { self.lexer.head_token.iter().any(|h| h.eq_ignore_ascii_case(s)) }
    #[inline] fn is_paragraf(&self, s: &str) -> bool { self.lexer.paragraf_token.iter().any(|p| p.eq_ignore_ascii_case(s)) }
    #[inline] fn is_bold(&self, s: &str) -> bool { self.lexer.bold_token.iter().any(|b| b.eq_ignore_ascii_case(s)) }
    #[inline] fn is_italics(&self, s: &str) -> bool { self.lexer.italics_token.iter().any(|i| i.eq_ignore_ascii_case(s)) }
    #[inline] fn is_list(&self, s: &str) -> bool { self.lexer.list_token.iter().any(|l| l.eq_ignore_ascii_case(s)) }
    #[inline] fn is_item(&self, s: &str) -> bool { self.lexer.item_token.iter().any(|i| i.eq_ignore_ascii_case(s)) }
    //#[inline] fn is_newline(&self, s: &str) -> bool { self.lexer.newline_token.iter().any(|n| n.eq_ignore_ascii_case(s)) }
    #[inline] fn is_soundz(&self, s: &str) -> bool { self.lexer.soundz_token.iter().any(|s2| s2.eq_ignore_ascii_case(s)) }
    #[inline] fn is_vidz(&self, s: &str) -> bool { self.lexer.vidz_token.iter().any(|v| v.eq_ignore_ascii_case(s)) }
    #[inline] fn is_ihaz(&self, s: &str) -> bool { self.lexer.ihaz_token.iter().any(|i| i.eq_ignore_ascii_case(s)) }
    //#[inline] fn is_itiz(&self, s: &str) -> bool { self.lexer.itiz_token.iter().any(|i| i.eq_ignore_ascii_case(s)) }
    #[inline] fn is_lemmesee(&self, s: &str) -> bool { self.lexer.lemmesee_token.iter().any(|l| l.eq_ignore_ascii_case(s)) }
   }


/*
    Implements the SyntaxAnalyzer trait for MarkdownSyntaxAnalyzer
   Each func is for a nonterminal in the grammar 
 */
impl SyntaxAnalyzer for MarkdownSyntaxAnalyzer {

    //<lolcode> ::= HAI <comments> <head> <body> KTHXBYE
    fn parse_lolcode(&mut self) {
        if self.is_hai(&self.current_token) {
            let _ = self.next_token();

            //optional comment block after HAI
            if self.is_obtw(&self.current_token) {
            self.parse_comment();
            }

            //appropriate heady and body sections
            self.parse_head();
            self.parse_body();

            //ends with KTHXBYE
            if self.is_bai(&self.current_token) {
                let _ = self.next_token();
            } else {
                eprintln!("Syntax error: Program MUST end with #KTHXBYE")
            }
        } else {
            eprintln!("Syntax error: Program MUST start with #HAI")
        }
    }

    //<head> ::= MAEK HEAD <title> OIC
    fn parse_head(&mut self) {
        if self.is_maek(&self.current_token) {
            let _ = self.next_token();

            if self.is_head(&self.current_token) {
                let _ = self.next_token();

                //parse title inside head
                self.parse_title();
                //checks for #OIC at end
                if self.is_oic(&self.current_token) {
                    let _ = self.next_token();
                }
                else {
                    eprintln!("Syntax error: A head annotation must end with #OIC");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Syntax error: A head annotation must have HEAD after #MAEK");
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: A head annotation start with #MAEK")
        }
    }

    //<title> ::= GIMMEH TITLE TEXT MKAY 
    fn parse_title(&mut self) {

       if !self.is_gimmeh(&self.current_token) {
            eprintln!("Syntax error: title annotation must start with #GIMMEH");
            std::process::exit(1);
        }

    
        // skip #GIMMEH
        let _ = self.next_token();

        if !self.current_token.eq_ignore_ascii_case("TITLE") {
            eprintln!("Syntax error: expected TITLE after #GIMMEH, found '{}'", self.current_token);
            std::process::exit(1);
        }

        // skip TITLE
        let _ = self.next_token();

        // accept all tokens until #MKAY
        while !self.is_mkay(&self.current_token) {
            if self.current_token.is_empty() {
                eprintln!("Syntax error: missing #MKAY at end of TITLE");
                std::process::exit(1);
            }
            let _ = self.next_token();
        }

        // skip #MKAY
        let _ = self.next_token();
    }

    
    //<comments> ::= <comment> <comment> | ε
    //<comment> ::= OBTW TEXT TLDR
    fn parse_comment(&mut self) {

    //comment has to start with OBTW
    if !self.is_obtw(&self.current_token) {
        eprintln!("Syntax error: comment must begin with #OBTW");
        std::process::exit(1);
    }

    // move past #OBTW
    let _ = self.next_token();

    // keep consuming tokens until #TLDR
    while !self.is_tldr(&self.current_token) {
        if self.current_token.is_empty() {
            eprintln!("Syntax error: comment missing #TLDR terminator");
            std::process::exit(1);
        }
        let _ = self.next_token();
    }

    // move past #TLDR
    let _ = self.next_token();
    }


    //<body> ::= <inner_body> <body>
    //<inner-body> ::= <paragraph> | <bold> | <italics> | <list> | <audio> | <video> | <newline> 
    //| <variable-define> | <variable-use> | TEXT | <comment>
    fn parse_body(&mut self) {

        // stop if we reach end of program or block
        if self.is_bai(&self.current_token) || self.is_oic(&self.current_token) {
            return;
        }

        //this is fo r the paragraph block 
        if self.is_paragraf(&self.current_token) || self.is_maek(&self.current_token) {
            self.parse_paragraph();
        }

        //inline annotations starting with #GIMMEH
        else if self.is_gimmeh(&self.current_token) {

            //look at next token without removing it
            let next_tok = if !self.lexer.tokens.is_empty() {
                self.lexer.tokens[self.lexer.tokens.len() - 1].to_uppercase()
            } else {
                String::new()
            };


            if next_tok == "BOLD" {
                self.parse_bold();
            } else if next_tok == "ITALICS" {
                self.parse_italics();
            } else if next_tok == "LIST" {
                self.parse_list();
            } else if next_tok == "SOUNDZ" {
                self.parse_audio();
            } else if next_tok == "VIDZ" {
                self.parse_video();
            } else if next_tok == "NEWLINE" {
                self.parse_newline();
            } else {
                eprintln!("Syntax error: unrecognized #GIMMEH annotation '{}'", next_tok);
                std::process::exit(1);

            } return; //done handling inline
        }

        // variable definitions and usage

        //IHAZ
        else if self.is_ihaz(&self.current_token) {
            self.parse_variable_define();
        } 
        //LEMMESEE
        else if self.is_lemmesee(&self.current_token) {
            self.parse_variable_use();
        } 
        // plain text
        else if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
            self.parse_text();
        } 
        // comments
        else if self.is_obtw(&self.current_token) {
            self.parse_comment();
        } 
        // unknown token
        else {
            eprintln!("Syntax error: unknown '{}' token inside body", self.current_token);
            std::process::exit(1);
        }

        // recurisve call to continue with rest of body
        if !self.is_bai(&self.current_token) && !self.is_oic(&self.current_token) {
        self.parse_body();
        }
        }

    //<paragraph> ::= MAEK PARAGRAF <variable-define> <inner-paragraph> OIC
    fn parse_paragraph(&mut self) {
        if self.is_maek(&self.current_token) {
            let _ = self.next_token();

            if self.is_paragraf(&self.current_token) {
                let _ = self.next_token();

                //optional var def at paragraph start
                self.parse_variable_define();
                //parse text/inline annotations
                self.parse_inner_paragraph();

                if self.is_oic(&self.current_token) {
                    let _ = self.next_token();
                } else {
                    eprintln!("Syntax error: A head paragraph must end with #OIC");
                    std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: Found '{}' after #MAEK, should be #PARAGRAF", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: A paragraph annotation should start with '#MAEK");
        std::process::exit(1);
    }
    }


    //<inner-paragraph> ::= <inner-text> <inner-paragraph> | ε
    fn parse_inner_paragraph(&mut self) {
        //stop recursion if we reach paragraph or program end
        if self.is_oic(&self.current_token) || self.is_bai(&self.current_token) {
        return;
        }

        //parse everything inside
        self.parse_inner_text();
        //recursion until para finishes
        self.parse_inner_paragraph();
    }

    // <inner-text> ::= <variable-use> | <bold> | <italics> .... TEXT | ε
    fn parse_inner_text(&mut self) {

        //Case 1 var use (LEMMESEE MKAY)
        if self.is_lemmesee(&self.current_token) {
            self.parse_variable_use();
        }

        //Case 2: inline annotations
        else if self.is_gimmeh(&self.current_token) {
            // Look ahead one token to see which kind of annotation it is
            let next_tok = if !self.lexer.tokens.is_empty() {
                self.lexer.tokens[self.lexer.tokens.len() - 1].to_uppercase()
            } else {
                String::new()
            };

            //caals parsers for different types
            if next_tok == "BOLD" {
                self.parse_bold();
            } else if next_tok == "ITALICS" {
                self.parse_italics();
            } else if next_tok == "LIST" {
                self.parse_list();
            } else if next_tok == "SOUNDZ" {
                self.parse_audio();
            } else if next_tok == "VIDZ" {
                self.parse_video();
            } else if next_tok == "NEWLINE" {
                self.parse_newline();
            } else {
                eprintln!("Syntax error: unrecognized #GIMMEH annotation '{}' inside paragraph", next_tok);
                std::process::exit(1);
            }
        }

        // case 3: plain text
        else if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
            self.parse_text();
        }
        else {
            eprintln!("Syntax error: unknown token '{}' inside inner-paragraph", self.current_token);
            std::process::exit(1);
        }
    }   

    // <variable-define> ::= IHAZ VARDEF ITIZ TEXT MKAY | ε
    //handles static variable definitions
    fn parse_variable_define(&mut self) {
    if self.is_ihaz(&self.current_token) {
        
        let _ = self.next_token();

        // expect HAZ instantly after #I
        if !self.current_token.eq_ignore_ascii_case("HAZ") {
            eprintln!("Syntax error: expected 'HAZ' after #I, got '{}'", self.current_token);
            std::process::exit(1);
        }
        let _ = self.next_token();

        // variable name enxt
        let name = self.current_token.clone();
        if name.starts_with('#') || name.is_empty() {
            eprintln!("Syntax error: expected variable name after HAZ, got '{}'", name);
            std::process::exit(1);
        }
        let _ = self.next_token();

        // expect #IT
        if !self.current_token.eq_ignore_ascii_case("#IT") {
            eprintln!("Syntax error: expected #IT after variable name, got '{}'", self.current_token);
            std::process::exit(1);
        }
        let _ = self.next_token();

        // expect IZ
        if !self.current_token.eq_ignore_ascii_case("IZ") {
            eprintln!("Syntax error: expected 'IZ' after #IT, got '{}'", self.current_token);
            std::process::exit(1);
        }
        let _ = self.next_token();

        // variable value (TEXT)
        if self.current_token.starts_with('#') || self.current_token.is_empty() {
            eprintln!("Syntax error: expected value after IZ, got '{}'", self.current_token);
            std::process::exit(1);
        }

        let _ = self.next_token();

        // expect #MKAY at end
        if !self.is_mkay(&self.current_token) {
            eprintln!("Syntax error: var def must end with #MKAY, got '{}'", self.current_token);
            std::process::exit(1);
        }
        let _ = self.next_token();

        //IMPORTANT: STORE VARIABLE LATER FOR SEMANTIC CHECKING
        if !self.defined_variables.contains(&name) {
            self.defined_variables.push(name);
        }
    }
    }
    

    //<variable-use> ::= LEMME SEE VAR_NAME MKAY
    fn parse_variable_use(&mut self) {
    if self.is_lemmesee(&self.current_token) {
        let _ = self.next_token(); 

        // Expect SEE after LEMME
        if !self.current_token.eq_ignore_ascii_case("SEE") {
            eprintln!("Syntax error: expected 'SEE' after #LEMME, got '{}'", self.current_token);
            std::process::exit(1);
        }
        let _ = self.next_token(); // skip SEE

       
        //text
        if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
            // Static check
            let _ = self.next_token();
            if self.is_mkay(&self.current_token) {
                let _ = self.next_token();
                return;
            } else {
                eprintln!("Syntax error: the variable use must end with #MKAY");
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected variable name after SEE, got '{}'", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: variable use must start with #LEMME SEE");
        std::process::exit(1);
    }
    }

    //<bold> ::= GIMMEH BOLD TEXT MKAY
    fn parse_bold(&mut self) {
        if self.is_gimmeh(&self.current_token) {
        let _ = self.next_token();
        if self.is_bold(&self.current_token) {
            let _ = self.next_token();
            //text right after BOLD
            if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
                let _ = self.next_token();
                while !self.is_mkay(&self.current_token) && !self.current_token.is_empty() {
                    let _ = self.next_token();
                }

                if self.is_mkay(&self.current_token) {
                    let _ = self.next_token();
                } else {
                    eprintln!("Syntax error: bold annotation has to end with #MKAY");
                    std::process::exit(1);
                }

            } else {
                eprintln!("Syntax error: expected TEXT after bold, but found '{}'", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected bold after #GIMMEH but found '{}'", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: bold annotation must start with #GIMMEH");
        std::process::exit(1);
    }
    }

    /*
    Same logic as parse_bold
     */
    fn parse_italics(&mut self) {
     if self.is_gimmeh(&self.current_token) {
        let _ = self.next_token(); 

        if self.is_italics(&self.current_token) {
            let _ = self.next_token(); 

            // Keep reading tokens until we hit #MKAY
            while !self.is_mkay(&self.current_token) && !self.current_token.is_empty() {
                // Text inside italics (ignore inline tokens for now)
                if self.current_token.starts_with('#') {
                    eprintln!("Syntax error: unexpected '{}' inside italics annotation", self.current_token);
                    std::process::exit(1);
                }
                let _ = self.next_token();
            }

            if self.is_mkay(&self.current_token) {
                let _ = self.next_token();
                return;
            } else {
                eprintln!("Syntax error: italics annotation has to end with #MKAY");
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected ITALICS after #GIMMEH but found '{}'", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: italics annotation must start with #GIMMEH");
        std::process::exit(1);
    }
    }


    //<list> ::= MAEK LIST <list-items> OIC
    fn parse_list(&mut self) {
        if self.is_maek(&self.current_token) {
            let _ = self.next_token();
            if self.is_list(&self.current_token) {
                let _ = self.next_token();
                //parse list items recursion
                self.parse_list_items();
                if self.is_oic(&self.current_token) {
                    let _ = self.next_token();
                } else {
                    eprintln!("Syntax error: list annotation must end with #OIC");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Syntax error, expected LIST after #MAEK, but got '{}' instead", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: List annotation must start wuth #MAEK");
            std::process::exit(1);
        }
    }


    //<list-items> ::= <list-item> <list-items> | ε
    fn parse_list_items(&mut self) {

        //stop recursion when list or program ends
        if self.is_oic(&self.current_token) || self.is_bai(&self.current_token) {
        return;
        }

        //otherwise continue on
        if self.is_gimmeh(&self.current_token) {
            let _ = self.next_token();
            if self.is_item(&self.current_token) {
                let _ = self.next_token();
                //inner text of list
                self.parse_inner_list();
                if self.is_mkay(&self.current_token) {
                    let _ = self.next_token();
                    self.parse_list_items(); 
                } else {
                    eprintln!("Syntax error: list item must end with #MKAY");
                    std::process::exit(1);
                }
            } else {
                eprintln!("Syntax error: expected item after #GIMMEH, got '{}'", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected #GIMMEH ITEM inside the lis, '{}' instead", self.current_token);
            std::process::exit(1);
        }
        }

    //stop parsing if at end of list or item
    fn parse_inner_list(&mut self) {
         if self.is_mkay(&self.current_token) || self.is_oic(&self.current_token) || self.is_bai(&self.current_token) {
        return;
    }

    // Inner list items can have   inline bold or italics
    if self.is_gimmeh(&self.current_token) {
    let _ = self.next_token();
        if self.is_bold(&self.current_token) {
            self.parse_bold();
        } else if self.is_italics(&self.current_token) {
            self.parse_italics();
        } else {
            eprintln!("Syntax error: expected BOLD or ITALICS after #GIMMEH, found '{}'", self.current_token);
            std::process::exit(1);
        }
    return;
    }
}

    //<audio> ::= GIMMEH SOUNDZ ADDRESS MKAY
    fn parse_audio(&mut self) {
        if self.is_gimmeh(&self.current_token) {
            let _ = self.next_token();
            if self.is_soundz(&self.current_token) {
                let _ = self.next_token();
                //literal file path/url
                if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
                    let _ = self.next_token();
                    if self.is_mkay(&self.current_token) {
                        let _ = self.next_token();
                    } else {
                        eprintln!("Syntax error: audio annotation must end with #MKAY");
                        std::process::exit(1);
                    }
            } else {
                eprintln!("Syntax error: expected ADDRESS after SOUNDZ, got '{}' instead", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected SOUNDZ after GIMMEH, got '{}' instead", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: audio annotation must start with #GIMMEH");
        std::process::exit(1);
    }
}

    //<video> ::= GIMMEH VIDZ ADDRESS MKAY
    fn parse_video(&mut self) {
           if self.is_gimmeh(&self.current_token) {
            let _ = self.next_token();
            if self.is_vidz(&self.current_token) {
                let _ = self.next_token();
                if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
                    let _ = self.next_token();
                    if self.is_mkay(&self.current_token) {
                        let _ = self.next_token();
                    } else {
                        eprintln!("Syntax error: video annotation must end with #MKAY");
                        std::process::exit(1);
                    }
            } else {
                eprintln!("Syntax error: video ADDRESS after VIDZ, got '{}' instead", self.current_token);
                std::process::exit(1);
            }
        } else {
            eprintln!("Syntax error: expected VIDZ after GIMMEH, got '{}' instead", self.current_token);
            std::process::exit(1);
        }
    } else {
        eprintln!("Syntax error: video annotation must start with #GIMMEH");
        std::process::exit(1);
    }
    }

    //Parses manual line break annotations
    fn parse_newline(&mut self) {
        let _ = self.next_token(); 

        //next token must literally be "NEWLINE"
        if self.current_token.eq_ignore_ascii_case("NEWLINE") {
            let _ = self.next_token();
            return;
        }

        eprintln!("Syntax error: expected 'NEWLINE' after #GIMMEH, got '{}'", self.current_token);
        std::process::exit(1);
    }

    //parses raw text
    fn parse_text(&mut self) {
        //text annotations dont begin with #
        if !self.current_token.starts_with('#') && !self.current_token.is_empty() {
        let _ = self.next_token();
        } else { 
            eprintln!("Syntax error: expected TEXT token, found '{}'", self.current_token);
            std::process::exit(1);
        }
    }
}


/*
Semantic analyzer: converts LOLCODE markdown tags to HTML and checks static variable usage

 */
pub struct SemanticAnalyzer {
    pub tokens: Vec<String>,

    /* these are parallel for variable definitions, the indexes map with each other
      initially, i tried to use a hashmap since that was what gpt suggested but got confused
    */
    pub variable_names: Vec<String>,
    pub variable_values: Vec<String>,
}


impl SemanticAnalyzer {
    pub fn new(tokens: Vec<String>) -> Self {
        Self {
            tokens,
            variable_names:Vec::new(),
            variable_values:Vec::new(),
        }
    }

    //convert directly from tokens to HTML 
     pub fn convert_html(&mut self) -> String {
        //for the generated html
        let mut html = String::new();
        //index for tokens
        let mut i = 0;

        //mock/fake stack. push and pop when necessary
        let mut stack: Vec<&'static str> = Vec::new();

        //go entire token stream left-to-right since its reverse
        while i < self.tokens.len() {
            let t = &self.tokens[i];

            //valid file then open <html>
            if t.eq_ignore_ascii_case("#HAI") {
                html.push_str("<html>\n");
            } else if t.eq_ignore_ascii_case("#KTHXBYE") {
                //left blank bcs we deal with it later
            }

            //Comments
            else if t.eq_ignore_ascii_case("#OBTW") {
                //start
                html.push_str("<!-- ");

                //text used until we hit TLDR
                i += 1;
                while i < self.tokens.len() && !self.tokens[i].eq_ignore_ascii_case("#TLDR") {
                    html.push_str(&self.tokens[i]);
                    html.push(' ');
                    i += 1;
                }
                //comment end
                html.push_str("-->\n");
            }

            //BLOCKS oepned by #MAEK, hanldes <head>, <paragraph>, and <list>
            else if t.eq_ignore_ascii_case("#MAEK") && i + 1 < self.tokens.len() {
                let next = &self.tokens[i + 1];
                if next.eq_ignore_ascii_case("HEAD") {
                    html.push_str("<head>\n");
                    stack.push("HEAD");
                } else if next.eq_ignore_ascii_case("PARAGRAF") {
                    html.push_str("<p>");
                    stack.push("PARAGRAF");
                } else if next.eq_ignore_ascii_case("LIST") {
                    html.push_str("<ul>");
                    stack.push("LIST");
                }
                //skip name token after #MAEK
                i += 1; 
            }

            //Pops most recent block and emits approrpirate tag
            //I was having issues with some of the tests and this is what gpt suggested
            else if t.eq_ignore_ascii_case("#OIC") {
                if let Some(open) = stack.pop() {
                    match open {
                        "HEAD" => html.push_str("</head>\n"),
                        "PARAGRAF" => html.push_str("</p>\n"),
                        "LIST" => html.push_str("</ul>\n"),
                        _ => {}
                    }
                }
            }

            //Inline constructs opened after #GIMMEH X BOLD/ITALICS, etc.
            else if t.eq_ignore_ascii_case("#GIMMEH") && i + 1 < self.tokens.len() {
                let next = &self.tokens[i + 1];

                if next.eq_ignore_ascii_case("TITLE") {
                    html.push_str("<title>");
                    //skips title token
                    stack.push("TITLE"); 
                    i += 1;
                } else if next.eq_ignore_ascii_case("BOLD") {
                    html.push_str("<b>");
                    stack.push("BOLD");
                    i += 1;
                } else if next.eq_ignore_ascii_case("ITALICS") {
                    html.push_str("<i>");
                    stack.push("ITALICS");
                    i += 1;
                } else if next.eq_ignore_ascii_case("ITEM") {
                    html.push_str("<li>");
                    stack.push("ITEM");
                    i += 1;
                } else if next.eq_ignore_ascii_case("NEWLINE") {
                    html.push_str("<br>\n");
                    i += 1;
                } else if next.eq_ignore_ascii_case("SOUNDZ") && i + 2 < self.tokens.len() {
                    html.push_str("<audio controls><source src=\"");
                    html.push_str(&self.tokens[i + 2]);
                    html.push_str("\"></audio>");
                    //its +2 bcs we're consuming both soundz and address
                    i += 2; 
                } else if next.eq_ignore_ascii_case("VIDZ") && i + 2 < self.tokens.len() {
                    html.push_str("<iframe src=\"");
                    html.push_str(&self.tokens[i + 2]);
                    html.push_str("\"/>");
                    i += 2; 
                }
            }

            // Close the most recent inline with #MKAY
            //also used gpt for this chunk:
            else if t.eq_ignore_ascii_case("#MKAY") {
                if let Some(open) = stack.pop() {
                    match open {
                        "TITLE" => html.push_str("</title>\n"),
                        "BOLD" => html.push_str("</b>\n"),
                        "ITALICS" => html.push_str("</i>\n"),
                        "ITEM" => html.push_str("</li>\n"),
                        // If someone erroneously pushes a container, ignore here (containers use #OIC)
                        _ => {}
                    }
                }
            }

            // Variables use and definition
            // 0 for I, 1 for HAZ, 2 for NAME, 3 for #IT, 5 for VALUE, 6 for MKAY
            else if t.eq_ignore_ascii_case("#I") && i + 5 < self.tokens.len() {
                let name = self.tokens[i + 2].clone();
                let val  = self.tokens[i + 5].clone();
                //append definition
                self.variable_names.push(name);
                self.variable_values.push(val);
                i += 5;
            } else if t.eq_ignore_ascii_case("#LEMME") && i + 2 < self.tokens.len() {
                //similar pattern are before

                //Search backwards to support shadowing semantics (nearest definition wins).
                //this small chunk is also heavily assisted with gpt:
                let name = self.tokens[i + 2].clone();
                let mut found = None;
                for idx in (0..self.variable_names.len()).rev() {
                    if self.variable_names[idx].eq_ignore_ascii_case(&name) {
                        found = Some(idx); break;
                    }
                }
                // If found emit the corresponding value, otherwise static semantci error
                if let Some(idx) = found {
                    html.push_str(&self.variable_values[idx]);
                } else {
                    eprintln!("Static semantic error: variable '{}' not defined.", name);
                    std::process::exit(1);
                }
                i += 2;
            }

            // Plain text
            else if !t.starts_with('#') && !t.is_empty() {
                html.push_str(t);
                html.push(' ');
            }

            i += 1;
        }

        //some tests like test6, test8, test9 were having issues with the </p>
        //so added incase token stream forgot something 
        if stack.contains(&"PARAGRAF") {
            html.push_str("</p>\n");
        }
        if stack.contains(&"HEAD") {
            html.push_str("</head>\n");
        }

        //</html> pushed at end here
        html.push_str("\n</html>\n");
        html

    }
}


//compiler implementation. tokenizes using lexical, parses using syntax, rebuilds token stream using semantic
impl Compiler for MarkdownSyntaxAnalyzer {
        fn compile(&mut self, source: &str) {

        //lexical analysis
        self.lexer = MarkdownLexicalAnalyzer::new(source);
        self.lexer.tokenize();

        //clone of tokens before syntax analyzer uses it, later for semantic analysis
        let all_tokens = self.lexer.tokens.clone();

        //pop first availble token for parser to start
        if let Some(first) = self.lexer.tokens.pop() {
            self.current_token = first;
        }
        //parse
        self.parse_lolcode();

        //since parser consumes tokens from back, we reverse order here (to keep it left to right)
        let mut sem = SemanticAnalyzer::new({ let mut fixed = all_tokens.clone(); fixed.reverse(); fixed });

        //html semantics conbevrt
        let html_output = sem.convert_html();

        
        //output file
        //IMPORTANT: ALL OUTPUTS FOR THE TESTS ARE PRINTED IN OUTPUT.HTML
        let output_path = Path::new("output.html");
        fs::write(&output_path, html_output)
            .unwrap_or_else(|e| eprintln!("Error writing HTML output: {}", e));

        
        //open in browser
        //used gpt here as recommended in the sintructions
        if cfg!(target_os = "windows") {
            if let Ok(abs_path) = output_path.canonicalize() {
            let _ = Command::new("cmd")
            .args([
                "/C",
                "start",
                "", // window title placeholder
                &abs_path.to_string_lossy()
            ])
            .spawn();
    }
}
    }


    /*
    THe compilation logic is mostly done by the lexical, syntax, and semantic analyzer
    I don't think I structured the compiler design correctly, so this is delegated elsewhere
     */
    fn next_token(&mut self) -> String {
        self.lexer.tokens.pop().unwrap_or_default()
    }


    fn parse(&mut self) {
        self.parse_lolcode();
    }

    fn current_token(&self) -> String {
        self.current_token.clone()
    }

    fn set_current_token(&mut self, tok: String) {
        self.current_token = tok;
    }
}



fn main() {

   let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file.lol>", args[0]);
        std::process::exit(1);
    }

    //file extension
    let filename = &args[1];
    if !filename.ends_with(".lol") {
        eprintln!("Error!: input file needs a .lol extension!");
        std::process::exit(1);
    }

    //file content
    let source = fs::read_to_string(filename).unwrap_or_else(|err| {
        eprintln!("Error reading file '{}': {}", filename, err);
        std::process::exit(1);
    });

    //run compiler
    let mut compiler = MarkdownSyntaxAnalyzer::new();
    compiler.compile(&source);
}
