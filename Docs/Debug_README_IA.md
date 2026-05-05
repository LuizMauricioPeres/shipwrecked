# Debug — Harbour functions for SWed transpiler
# Call stack inspection: procedure name, source file, line number.
# count:3

ProcFile|ProcFile( <xExp> ) --> <cEmptyString>|This function allways returns an empty string.|ARGS:<xExp>:<xExp> is any valid type.
ProcLine|ProcLine( <nLevel> ) --> <nLine>|Gets the line number of the current function on the stack.|ARGS:<nLevel>:<nLevel> is the function level required.
ProcName|ProcName( <nLevel> ) --> <cProcName>|Gets the name of the current function on the stack|ARGS:<nLevel>:<nLevel> is the function level required.