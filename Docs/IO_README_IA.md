# IO — Harbour functions for SWed transpiler
# stdout/stderr output, command execution, process exit.
# count:5

__Quit|__Quit()|Terminates an application.
__Run|__Run( <cCommand> )|Run an external program.|ARGS:<cCommand>:<cCommand> Command to execute.
__TypeFile|__TypeFile( <cFile>, [<lPrint>] ) --> NIL|Show the content of a file on the console and/or printer|ARGS:<cFile>:<cFile> is a name of the file to display. If the file have an extension, it must be specified (there is no default value; <lPrint>:<lPrint> is an optional logical value that specifies whether the output should go only to the screen (.F.) or to both th
OutErr|OutErr( <xExp,...> )|Write a list of values to the standard error device|ARGS:<xExp,...>:<xExp,...> is a list of expressions to display. Expressions are any mixture of Harbour data types.
OutStd|OutStd( <xExp,...> )|Write a list of values to the standard output device|ARGS:<xExp,...>:<xExp,...> is a list of expressions to display. Expressions are any mixture of Harbour data types.