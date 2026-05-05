# Keyboard — Harbour functions for SWed transpiler
# Key read, key-action binding, keyboard buffer config.
# count:13

__Input|__Input( <cMessage> ) --> <cString>|Stops application|ARGS:<cMessage>:<cMessage> is any valid expression.
__Wait|__Wait( <cMessage> ) --> <cKey>|Stops the application until a key is pressed.|ARGS:<cMessage>:<cMessage> is a string.
hb_keyPut|hb_keyPut( <nInkeyCode> )|Put an inkey code to the keyboard buffer.|ARGS:<nInkeyCode>:<nInkeyCode> is the inkey code, which should be inserted into the keyboard buffer.
hb_SetKeyCheck|hb_SetKeyCheck( <nKey> [, <p1> ][, <p2> ][, <p3> ] )|Implements common hot-key activation code|ARGS:<nKey>:<nKey> is a numeric key value to be tested code-block, if executed; <p1>:<p1>..<p3> are optional parameters that will be passed to the code-block
hb_SetKeyGet|hb_SetKeyGet( <nKey> [, <bConditionByRef> ] )|Determine a set-key code block and condition-block|ARGS:<anKey>:<anKey> is an numeric key value; <bConditionByRef>:<bConditionByRef> is an optional return-parameter
hb_SetKeySave|hb_SetKeySave( [ <OldKeys> ] )|Returns a copy of internal set-key list, optionally overwriting|ARGS:<OldKeys>:<OldKeys> is an optional set-key list from a previous call to hb_SetKeySave(), or NIL to clear current set-key list
hb_SetMacro|hb_SetMacro( <nOption>, [<lOnOff>] ) --> <lOldSetting>|Enable/disable the macro compiler runtime features.|ARGS:<nOption>:<nOption> One of the HB_SM_* constants defined in set.ch.; <lOnOff>:<lOnOff> .T. to enable or .F. to disable a feature
Inkey|Inkey( [<nTimeout>] [, <nEvents>] ) --> nKey|Extracts the next key code from the Harbour keyboard buffer.|ARGS:<nTimeout>:<nTimeout> is an optional timeout value in seconds, with a granularity of 1/10th of a second. If omitted, Inkey() return; <nEvents>:<nEvents> is an optional mask of input events that are to be enabled. If omitted, defaults to hb_set.HB_SET_EVENTMASK. V; <table>:<table> inkey.ch            Meaning INKEY_MOVE          Mouse motion events are allowed INKEY_LDOWN         The mouse le; </table>:</table> If the parameter is not numeric, it will be treated as if it were set to hb_set.HB_SET_EVENTMASK.
LastKey|LastKey( [<nInputMask>] ) --> nKey|Get the last key extracted from the keyboard buffer.
NextKey|NextKey( [<nInputMask>] ) --> nKey|Get the next key code in the buffer without extracting it.
ReadKey|ReadKey() --> nKeyCode|Determine which key terminated a READ.
ReadVar|ReadVar( [<cVarName>] ) --> cOldVarName|Return variable name of current GET or MENU|ARGS:<cVarName>:<cVarName> is a new variable name to set.
SetKey|SetKey( <anKey> [, <bAction> [, <bCondition> ] ] )|Assign an action block to a key|ARGS:<anKey>:<anKey> is either a numeric key value, or an array of such values; <bAction>:<bAction> is an optional code-block to be assigned; <bCondition>:<bCondition> is an optional condition code-block