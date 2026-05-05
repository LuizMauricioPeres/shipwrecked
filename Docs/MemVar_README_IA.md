# MemVar — Harbour functions for SWed transpiler
# PRIVATE/PUBLIC vars and memvar pool access by name/ref.
# count:11

__mvClear|__mvClear()|This function releases all PRIVATE and PUBLIC variables
__mvDbgInfo|__mvDbgInfo( <nScope> [, <nPosition> [, @<cVarName>] ] )|This function returns the information about the variables for debugger|ARGS:<nScope>:<nScope> = the scope of variables for which an information is asked Supported values (defined in hbmemvar.ch) HB_MV_PUBL; <nPosition>:<nPosition> = the position of asked variable on the list of variables with specified scope - it should start from positi; <cVarName>:<cVarName> = the value is filled with a variable name if passed by
__mvExist|__mvExist( <cVarName> ) --> <lVariableExist>|Determine if a given name is a PUBLIC or PRIVATE memory variable|ARGS:<cVarName>:<cVarName> - string that specifies the name of variable to check
__mvGet|__mvGet( <cVarName> [, <xValue>] ) --> <xValue>|This function set the value of memory variable|ARGS:<cVarName>:<cVarName> - string that specifies the name of variable; <xValue>:<xValue>   - a value of any type that will be set - if it is not specified then NIL is assumed
__mvGet|__mvGet( <cVarName> ) --> <xVar>|This function returns value of memory variable|ARGS:<cVarName>:<cVarName> - string that specifies the name of variable
__mvPrivate|__mvPrivate( <variable_name> )|This function creates a PRIVATE variable|ARGS:<variable_name>:<variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable
__mvPublic|__mvPublic( <variable_name> )|This function creates a PUBLIC variable|ARGS:<variable_name>:<variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable
__mvRelease|__mvRelease( <skeleton>, <include_exclude_flag> )|This function releases PRIVATE variables|ARGS:<skeleton>:<skeleton> = string that contains the wildcard mask for variables' names that will be released. Supported wildcards: '*'; <include_exclude_flag>:<include_exclude_flag> = logical value that specifies if variables that match passed skeleton should be either included 
__mvScope|__mvScope( <cVarName> )|If variable exists then returns its scope.|ARGS:<cVarName>:<cVarName> = a string with a variable name to check
__mvXRelease|__mvXRelease( <variable_name> )|This function releases value stored in PRIVATE or PUBLIC variable|ARGS:<variable_name>:<variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable
MemVarBlock|MemVarBlock( <cMemvarName> ) --> <bBlock>|Returns a codeblock that sets/gets a value of memvar variable|ARGS:<cMemvarName>:<cMemvarName> - a string that contains the name of variable