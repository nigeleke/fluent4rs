searchState.loadedDescShard("fluent4rs", 0, "fluent4rs\nDerived from Project Fluent fluent.ebnf\nThe Parser enables a Fluent resource string to be parsed …\nThe Walker enables the Resource returned by the Parser to …\nArgument ::= NamedArgument | InlineExpression\nAttribute ::= line_end blank? “.” Identifier …\nAttributeAccessor ::= “.” Identifier\nblock_placeable ::= blank_block blank_inline? …\nblock_text ::= blank_block blank_inline indented_char …\nCallArguments ::= blank? “(” blank? argument_list …\nCommentLine ::= (“###” | “##” | “#”) (“\\u0020…\nDefaultVariant ::= line_end blank? “*” VariantKey …\nEntry ::= (Message line_end) | (Term line_end) | …\nFunctionReference ::= Identifier CallArguments\nIdentifier ::= [a-zA-Z] [a-zA-Z0-9_-]*\nInlineExpression ::= StringLiteral | NumberLiteral | …\ninline_text ::= text_char+\nJunk ::= junk_line (junk_line - “#” - “-” - [a-zA-Z…\nLiteral ::= NumberLiteral | StringLiteral\nMessage ::= Identifier blank_inline? “=” blank_inline? …\nMessageReference ::= Identifier AttributeAccessor?\nNamedArgument ::= Identifier blank? “:” blank? (…\nNumberLiteral ::= “-”? digits (“.” digits)?\nPattern ::= PatternElement+\nPatternElement ::= inline_text | block_text | …\nResource ::= ResourceItem*\nResourceItem ::= (Entry | blank_block | Junk)*\nSelectExpression ::= InlineExpression blank? “-&gt;” …\nStringLiteral ::= “&quot;” quoted_char* “&quot;”\nTerm ::= “-” Identifier blank_inline? “=” …\nTermReference ::= “-” Identifier AttributeAccessor? …\nVariableReference ::= “$” Identifier\nVariant ::= line_end blank? VariantKey blank_inline? …\nVariantKey ::= “[” blank? (NumberLiteral | Identifier) …\nvariant_list ::= Variant* DefaultVariant Variant* line_end\nReturns the message Attributes.\nReturns the message Attributes.\nReturn the parsed entries.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the argument unchanged.\nReturns the message identifier.\nReturns the term identifier.\nReturns the message identifier <em>name</em>.\nReturns the term identifier <em>name</em>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nReturn the parsed entries.\nReturns the message Pattern, if provided.\nReturns the term Pattern.\nReturns the argument unchanged.\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nCalls <code>U::from(self)</code>.\nParse the given string, treating any Junk as an error.\nParse the given string, returning the Junk as items in the …\nReturns the argument unchanged.\nCalls <code>U::from(self)</code>.\nWalk the AST. Each AST object is Walkable, however, …")