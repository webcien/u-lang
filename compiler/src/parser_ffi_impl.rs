// parser_ffi_impl.rs — FFI parsing implementation for U v0.9
// This file contains the implementation of parse_extern_block
// To be included in parser.rs

impl Parser {
    fn parse_extern_block(&mut self) -> Result<ExternBlock, String> {
        // extern "C" { ... }
        let abi_token = self.consume(TokenType::StringLiteral(String::new()), "Expected ABI string after 'extern'")?;
        let abi = match &abi_token.kind {
            TokenType::StringLiteral(s) => s.clone(),
            _ => return Err("Expected ABI string".to_string()),
        };
        
        if abi != "C" {
            return Err(format!("Only 'C' ABI is supported, got '{}'", abi));
        }
        
        self.consume(TokenType::LeftBrace, "Expected '{' after ABI string")?;
        
        let mut functions = Vec::new();
        while !self.check(TokenType::RightBrace) && !self.is_at_end() {
            functions.push(self.parse_extern_function()?);
        }
        
        self.consume(TokenType::RightBrace, "Expected '}' after extern block")?;
        
        Ok(ExternBlock { abi, functions })
    }
    
    fn parse_extern_function(&mut self) -> Result<ExternFunction, String> {
        // fn name(params...) -> return_type;
        self.consume(TokenType::Fn, "Expected 'fn' in extern block")?;
        
        let name_token = self.consume(TokenType::Identifier(String::new()), "Expected function name")?;
        let name = match &name_token.kind {
            TokenType::Identifier(s) => s.clone(),
            _ => return Err("Invalid function name".to_string()),
        };
        
        self.consume(TokenType::LeftParen, "Expected '(' after function name")?;
        
        let mut params = Vec::new();
        let mut is_variadic = false;
        
        if !self.check(TokenType::RightParen) {
            // Check for variadic (...)
            if self.check(TokenType::Dot) {
                self.advance(); // .
                self.advance(); // .
                self.advance(); // .
                is_variadic = true;
            } else {
                // Parse first parameter
                let param = self.parse_parameter()?;
                params.push(param);
                
                while self.match_token(TokenType::Comma) {
                    // Check for variadic after comma
                    if self.check(TokenType::Dot) {
                        self.advance(); // .
                        self.advance(); // .
                        self.advance(); // .
                        is_variadic = true;
                        break;
                    }
                    params.push(self.parse_parameter()?);
                }
            }
        }
        
        self.consume(TokenType::RightParen, "Expected ')' after parameters")?;
        
        let return_type = if self.match_token(TokenType::Arrow) {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        self.consume(TokenType::Semicolon, "Expected ';' after extern function declaration")?;
        
        Ok(ExternFunction {
            name,
            params,
            return_type,
            is_variadic,
        })
    }
}
