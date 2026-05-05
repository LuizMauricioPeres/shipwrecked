# Core — Harbour functions for SWed transpiler
# Types, codeblock eval, param count, essential flow control.
# count:13

Bin2I|Bin2I( <cBuffer> ) --> nNumber|Convert signed short encoded bytes into Harbour numeric|ARGS:<cBuffer>:<cBuffer> is a character string that contains 16-bit encoded signed short integer (least significant byte first). The fi
Bin2L|Bin2L( <cBuffer> ) --> nNumber|Convert signed long encoded bytes into Harbour numeric|ARGS:<cBuffer>:<cBuffer> is a character string that contains 32-bit encoded signed long integer (least significant byte first). The fir
Break|Break( <xExp> )|Exits from a BEGIN SEQUENCE block|ARGS:<xExp>:<xExp> is any valid expression. It is always required. If do not want to pass any argument, just use NIL.
Do|Do( <xFuncProc> [, <xArguments...>] ) --> <xRetVal>|Calls a procedure or a function|ARGS:<xFuncProc>:<xFuncProc> = Either a string with a function/procedure name to be called or a codeblock to evaluate.; <xArguments>:<xArguments> = arguments passed to a called function/procedure or to a codeblock.
Empty|Empty( <xExp> ) --> lIsEmpty|Checks if the passed argument is empty.|ARGS:<xExp>:<xExp> is any valid expression.
Eval|Eval( <bBlock> [, <xVal> [,...] ] ) --> xExpression|Evaluate a code block|ARGS:<bBlock>:<bBlock>   Code block expression to be evaluated; <xVal>:<xVal>     Argument to be passed to the code block expression; <xVal...>:<xVal...>  Argument list to be passed to the code block expression
hb_PIsByRef|hb_PIsByRef( nParam ) --> <lParamIsByRef>|Determine if a parameter is passed by reference.|ARGS:<nParam>:<nParam> is the parameter number to test.
hb_PValue|hb_PValue( <nArg> ) --> <xExp>|Retrieves the value of an argument.
I2Bin|I2Bin( <nNumber> ) --> cBuffer|Convert Harbour numeric into signed short encoded bytes|ARGS:<nNumber>:<nNumber> is a numeric value to convert (decimal digits are ignored).
L2Bin|L2Bin( <nNumber> ) --> cBuffer|Convert Harbour numeric into signed long encoded bytes|ARGS:<nNumber>:<nNumber> is a numeric value to convert (decimal digits are ignored).
PCount|PCount() --> <nArgs>|Retrieves the number of arguments passed to a function.
Type|Type( <cExp> ) --> <cRetType>|Retrieves the type of an expression|ARGS:<cExp>:<cExp> must be a character expression.
ValType|ValType( <xExp> ) --> <cRetType>|Retrieves the data type of an expression|ARGS:<xExp>:<xExp> is any valid expression.