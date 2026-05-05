# Keyboard

> Leitura de teclas, atribuição de ações a teclas e configuração do buffer de teclado.

**Funções neste módulo:** 13

---

## `__Input`

```
__Input( <cMessage> ) --> <cString>
```

Stops application

**Parâmetros:**

  - `<cMessage>` — <cMessage> is any valid expression.

---

## `__Wait`

```
__Wait( <cMessage> ) --> <cKey>
```

Stops the application until a key is pressed.

**Parâmetros:**

  - `<cMessage>` — <cMessage> is a string.

---

## `hb_keyPut`

```
hb_keyPut( <nInkeyCode> )
```

Put an inkey code to the keyboard buffer.

**Parâmetros:**

  - `<nInkeyCode>` — <nInkeyCode> is the inkey code, which should be inserted into the keyboard buffer.

---

## `hb_SetKeyCheck`

```
hb_SetKeyCheck( <nKey> [, <p1> ][, <p2> ][, <p3> ] )
```

Implements common hot-key activation code

**Parâmetros:**

  - `<nKey>` — <nKey> is a numeric key value to be tested code-block, if executed
  - `<p1>` — <p1>..<p3> are optional parameters that will be passed to the code-block

---

## `hb_SetKeyGet`

```
hb_SetKeyGet( <nKey> [, <bConditionByRef> ] )
```

Determine a set-key code block and condition-block

**Parâmetros:**

  - `<anKey>` — <anKey> is an numeric key value
  - `<bConditionByRef>` — <bConditionByRef> is an optional return-parameter

---

## `hb_SetKeySave`

```
hb_SetKeySave( [ <OldKeys> ] )
```

Returns a copy of internal set-key list, optionally overwriting

**Parâmetros:**

  - `<OldKeys>` — <OldKeys> is an optional set-key list from a previous call to hb_SetKeySave(), or NIL to clear current set-key list

---

## `hb_SetMacro`

```
hb_SetMacro( <nOption>, [<lOnOff>] ) --> <lOldSetting>
```

Enable/disable the macro compiler runtime features.

**Parâmetros:**

  - `<nOption>` — <nOption> One of the HB_SM_* constants defined in set.ch.
  - `<lOnOff>` — <lOnOff> .T. to enable or .F. to disable a feature

---

## `Inkey`

```
Inkey( [<nTimeout>] [, <nEvents>] ) --> nKey
```

Extracts the next key code from the Harbour keyboard buffer.

**Parâmetros:**

  - `<nTimeout>` — <nTimeout> is an optional timeout value in seconds, with a granularity of 1/10th of a second. If omitted, Inkey() returns immediately. If set to 0, Inkey() waits until an input event occurs. If set to any other value, Inkey() will return either when an input event occurs or when the timeout period has elapsed. If only this parameter is specified and it is not numeric, it will be treated as if it were 0. But if both parameters are specified and this parameter is not numeric, it will be treated as if it were not present.
  - `<nEvents>` — <nEvents> is an optional mask of input events that are to be enabled. If omitted, defaults to hb_set.HB_SET_EVENTMASK. Valid input masks are in inkey.ch and are explained below. It is recommended that the mask names be used rather than their numeric values, in case the numeric values change in future releases of Harbour. To allow more than one type of input event, simply add the various mask names together.
  - `<table>` — <table> inkey.ch            Meaning INKEY_MOVE          Mouse motion events are allowed INKEY_LDOWN         The mouse left click down event is allowed INKEY_LUP           The mouse left click up event is allowed INKEY_RDOWN         The mouse right click down event is allowed INKEY_RUP           The mouse right click up event is allowed INKEY_KEYBOARD      All keyboard events are allowed INKEY_ALL           All mouse and keyboard events are allowed HB_INKEY_EXTENDED   Extended keyboard codes are used.
  - `</table>` — </table> If the parameter is not numeric, it will be treated as if it were set to hb_set.HB_SET_EVENTMASK.

---

## `LastKey`

```
LastKey( [<nInputMask>] ) --> nKey
```

Get the last key extracted from the keyboard buffer.

---

## `NextKey`

```
NextKey( [<nInputMask>] ) --> nKey
```

Get the next key code in the buffer without extracting it.

---

## `ReadKey`

```
ReadKey() --> nKeyCode
```

Determine which key terminated a READ.

---

## `ReadVar`

```
ReadVar( [<cVarName>] ) --> cOldVarName
```

Return variable name of current GET or MENU

**Parâmetros:**

  - `<cVarName>` — <cVarName> is a new variable name to set.

---

## `SetKey`

```
SetKey( <anKey> [, <bAction> [, <bCondition> ] ] )
```

Assign an action block to a key

**Parâmetros:**

  - `<anKey>` — <anKey> is either a numeric key value, or an array of such values
  - `<bAction>` — <bAction> is an optional code-block to be assigned
  - `<bCondition>` — <bCondition> is an optional condition code-block

---
