# Compat — Harbour functions for SWed transpiler
# Clipper-compat functions — generally avoid direct use.
# count:11

__FLedit|__FLedit( <aStruct>, [<aFieldList>] ) --> aStructFiltered|Filter a database structure array|ARGS:<aStruct>:<aStruct> is a multidimensional array with database fields structure, which is usually the output from dbStruct(), where; <table>:<table> Position   Description    dbstruct.ch 1          cFieldName     DBS_NAME 2          cFieldType     DBS_TYPE 3   ; </table>:</table>; <aFieldList>:<aFieldList> is an array where each element is a field name. Names could be specified as uppercase or lowercase.
__NoNoAlert|__NoNoAlert()|Override //NOALERT command-line switch
__SetCentury|__SetCentury([<lFlag> | <cOnOff> ] ) --> lPreviousValue|Set the Current Century|ARGS:<lFlag>:optional <lFlag> or <cOnOff> (not case sensitive) .T. or "ON" to enable the century setting (4-digit years) .F. or "OFF"
__SetFunction|__SetFunction( <nFunctionKey>, [<cString>] ) --> NIL|Assign a character string to a function key|ARGS:<nFunctionKey>:<nFunctionKey> is a number in the range 1..40 that represent the function key to be assigned.; <cString>:<cString> is a character string to set. If <cString> is not specified, the function key is going to be set to NIL releas
__SetHelpK|__SetHelpK()|Set F1 as the default help key
__TextRestore|__TextRestore()|Restore console output settings as saved by __TextSave()
__TextSave|__TextSave( <cFile> )|Redirect console output to printer or file and save old settings|ARGS:<cFile>:<cFile> is either "PRINTER" (note the uppercase) in which console output is SET to PRINTER, or a name of a text file wit
CLIPINIT|CLIPINIT() --> NIL|Initialize various Harbour sub-systems
hb_FLock|hb_FLock( <nHandle>, <nOffset>, <nBytes> [, <nType> ] ) --> <lSuccess>|Locks part or all of any file|ARGS:<nHandle>:<nHandle>  Dos file handle; <nOffset>:<nOffset>  Offset of the first byte of the region to be locked.; <nBytes>:<nBytes>   Number of bytes to be locked.; <nType>:<nType>    The type (read or write) of lock requested.
hb_FUnlock|hb_FUnlock( <nHandle>, <nOffset>, <nBytes> ) --> <lSuccess>|Unlocks part or all of any file|ARGS:<nHandle>:<nHandle>  Dos file handle set>  Offset of the first byte of the region to be locked.; <nBytes>:<nBytes>   Number of bytes to be locked.
NetErr|NetErr( [<lNewError>] ) --> lError|Tests the success of a network function|ARGS:<lNewError>:<lNewError> Is a logical Expression.