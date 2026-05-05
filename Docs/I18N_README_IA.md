# I18N — Harbour functions for SWed transpiler
# Language/codepage selection and localized message access.
# count:5

hb_cdpSelect|hb_cdpSelect( [<cNewLang>] ) --> cOldLang|Select the active code page by language ID|ARGS:<cNewLang>:<cNewLang>  The optional ID of the language module.; <table>:<table>; </table>:</table>
hb_langErrMsg|hb_langErrMsg( <nErrorCode> ) --> cErrorMessage|Description of an error code using current language|ARGS:<nErrorCode>:<nErrorCode> is one of the generic error codes (EG_...) defined in error.ch
hb_langMessage|hb_langMessage( <nMsg>[, <cLangID>] ) --> cMessage|Returns international strings messages and errors|ARGS:<nMsg>:<nMsg>    is the message number to get.; <cLangID>:<cLangID> is an optional language module ID. Uses the currently selected language module, if not specified.
hb_langName|hb_langName( [<cLangID>] ) --> cLangName|Return the name of the language module|ARGS:<cLangID>:<cLangID> is an optional language module ID. Uses the currently selected language module, if not specified.
hb_langSelect|hb_langSelect( [<cNewLang>][, <cCodepage>] ) --> cOldLang|Select a specific nation message module|ARGS:<cNewLang>:<cNewLang>  The optional ID of the language module.; <cCodepage>:<cCodepage>  Optional codepage ID into which the language module strings are automatically converted by Harbour.; <table>:<table>; </table>:</table>