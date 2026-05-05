# System

> Informações do SO, variáveis de ambiente, configurações globais e mensagens do runtime.

**Funções neste módulo:** 10

---

## `ErrorSys`

```
ErrorSys() --> NIL
```

Install default error handler

---

## `GetE`

```
GetE( <cEnviroment> ) --> <cReturn>
```

Obtains a system environmental setting.

**Parâmetros:**

  - `<cEnviroment>` — <cEnviroment> Enviromental variable to obtain.

---

## `GetEnv`

```
GetEnv( <cEnviroment> ) --> <cReturn>
```

Obtains a system environmental setting.

**Parâmetros:**

  - `<cEnviroment>` — <cEnviroment> Enviromental variable to obtain.

---

## `hb_eol`

```
hb_eol() --> cString
```

Returns the newline character(s) to use with the current OS

---

## `hb_GetEnv`

```
hb_GetEnv( <cEnviroment>, [<cDefaultValue>] ) --> <cReturn>
```

Obtains a system environmental setting.

**Parâmetros:**

  - `<cEnviroment>` — <cEnviroment> Enviromental variable to obtain.
  - `<cDefaultValue>` — <cDefaultValue> Optional value to return if <cEnvironment> is not found.

---

## `NationMsg`

```
NationMsg( <nMsg> ) --> <cMessage>
```

Returns international strings messages.

**Parâmetros:**

  - `<nMsg>` — <nMsg> is the message number you want to get.   </par>

---

## `OS`

```
OS() --> <cOperatingSystem>
```

Return the current operating system.

---

## `Set`

```
Set( <nSet> [, <xNewSetting> [, <xOption> ] ] ) --> xPreviousSetting
```

Changes or evaluated environmental settings

**Parâmetros:**

  - `<nSet>` — <nSet> Set Number
  - `<xNewSetting>` — <xNewSetting> Any expression to assign a value to the setting
  - `<xOption>` — <xOption> Logical expression
  - `<lFlag>` — _SET_ALTERNATE   <lFlag> | <cOnOff> If enabled, QOut() and QQOut() write to the screen and to a file, provided that a file has been opened or created with _SET_ALTFILE. If disabled, which is the default, QOut() and QQOut() only write to the screen (and/or to the PRINTFILE). Defaults to disabled.
  - `<cFileName>` — _SET_ALTFILE     <cFileName>             <lAdditive> When set, creates or opens file to write QOut() and
  - `<lAdditive>` — QQOut() output to. If <lAdditive> is TRUE and the file already exists, the file is opened and positioned at end of file. Otherwise, the file is created. If a file is already opened, it is closed before the new file is opened or created (even if it is the same file). The default file extension is ".txt". There is no default file name. Call with an empty string to close the file.
  - `<cColorSet>` — _SET_COLOR       <cColorSet> Sets the current color scheme, using color pairs in the
  - `<standard>` — sequence "<standard>, <enhanced>, <border>, <background>,
  - `<unselected>` — <unselected>". Each color pair uses the format
  - `<foreground>` — "<foreground>/<background>". The color codes are space or "N" for black, "B" for blue, "G" for green, "BG" for Cyan, "R" for red, "RB" for magenta, "GR" for brown, "W" for white, "N+" for gray, "B+" for bright blue, "G+" for bright green, "BG+" for bright cyan, "R+" for bright red, "RB+" for bright magenta, "GR+" for yellow, and "W+" for bright white. Special codes are "I" for inverse video, "U" for underline on a monochrome monitor (blue on a color monitor), and "X" for blank. The default color is "W/N,N/W,N,N,N/W".
  - `<nCursorType>` — _SET_CURSOR      <nCursorType> If enabled, which is the default, the cursor is displayed on screen. If disabled, the screen cursor is hidden.
  - `<cDateFormat>` — _SET_DATEFORMAT  <cDateFormat> Sets the default date format for display, date input, and date conversion. Defaults to American ("mm/dd/yy"). Other formats include ANSI ("yy.mm.dd"), British ("dd/mm/yy"), French ("dd/mm/yy"), German ("dd.mm.yy"), Italian ("dd-mm-yy"), Japan ("yy/mm/dd"), and USA ("mm-dd-yy"). SET CENTURY modifies the date format. SET CENTURY ON replaces the "y"s with "YYYY". SET CENTURY OFF replaces the "y"s with "YY".
  - `<lStatus>` — _SET_DEBUG       <lStatus> When set to .T., pressing Alt+D activates the debugger. When set to .F., which is the default, Alt+D can be read by Inkey(). (Also affected by AltD(1) and AltD(0))
  - `<nNumberOfDecimals>` — _SET_DECIMALS    <nNumberOfDecimals> Sets the number of decimal digits to use when displaying printing numeric values when SET FIXED is ON. Defaults to 2. If SET FIXED is OFF, then SET DECIMALS is only used to determine the number of decimal digits to use after using Exp(), Log(), Sqrt(), or division. Other math operations may adjust the number of decimal digits that the result will display. Note: This never affects the precision of a number. Only the display format is affected.
  - `<cDefaultDirectory>` — _SET_DEFAULT     <cDefaultDirectory> Sets the default directory in which to open, create and check for files. Defaults to current directory (blank).
  - `<cDelimiters>` — _SET_DELIMCHARS  <cDelimiters> Sets the GET delimiter characters. Defaults to "::".
  - `<cDeviceName>` — _SET_DEVICE      <cDeviceName> Selects the output device for DevOut(). When set to "PRINTER", all output is sent to the printer device or file set by _SET_PRINTFILE. When set to anything else, all output is sent to the screen. Defaults to "SCREEN".
  - `<nYear>` — _SET_EPOCH       <nYear> Determines how to handle the conversion of 2-digit years to 4 digit years. When a 2-digit year is greater than or equal to the year part of the epoch, the century part of the epoch is added to the year. When a 2-digit year is less than the year part of the epoch, the century part of the epoch is incremented and added to the year. The default epoch is 1900, which converts all 2-digit years to 19xx. Example: If the epoch is set to 1950, 2-digit years in the range from 50 to 99 get converted to 19xx and 2-digit years in the range 00 to 49 get converted to 20xx.
  - `<nEventCodes>` — _SET_EVENTMASK   <nEventCodes> Determines which events Inkey() will respond to. INKEY_MOVE allows mouse movement events. INKEY_LDOWN allows the left mouse button down click. INKEY_LUP allows the left mouse button up click. INKEY_RDOWN allows the right mouse button down click. INKEY_RUP allows the right mouse button up clock. INKEY_KEYBOARD allows keyboard keystrokes. INKEY_ALL allows all of the preceding events. Events may be combined (e.g., using INKEY_LDOWN + INKEY_RUP will allow left mouse button down clicks and right mouse button up clicks). The default is INKEY_KEYBOARD.
  - `<cLanguageID>` — _SET_LANGUAGE    <cLanguageID> Specifies the language to be used for Harbour messages. [This is a Harbour extension]
  - `<nColumns>` — _SET_MARGIN      <nColumns> Sets the left margin for all printed output. The default value is 0. Note: PCol() reflects the printer's column position including the margin (e.g., SET MARGIN TO 5 followed by DevPos(5, 10) makes PCol() return 15).
  - `<nMemoBlockSize>` — _SET_MBLOCKSIZE <nMemoBlockSize> TODO: Document
  - `<nRow>` — _SET_MESSAGE     <nRow> If set to 0, which is the default, PROMPTs are always suppressed. Otherwise, PROMPTs are displayed on the set row. Note: It is not possible to display prompts on the top-most screen row, because row 0 is reserved for the SCOREBOARD, if enabled.
  - `<cMemoFileExt>` — _SET_MFILEEXT    <cMemoFileExt> TODO: Document
  - `<cDirectories>` — _SET_PATH        <cDirectories> Specifies a path of directories to search through to locate a file that can't be located in the DEFAULT directory. Defaults to no path (""). Directories must be separated by a semicolon (e.g., "C:\hb\bin;C:\hb\tests").
  - `<nKeyStrokes>` — _SET_TYPEAHEAD   <nKeyStrokes> Sets the size of the keyboard typeahead buffer. Defaults to 50. The minimum is 16 and the maximum is 4096.
  - `<nValue>` — _SET_VIDEOMODE   <nValue> TODO: Document

---

## `SetTypeahead`

```
SetTypeahead( <nSize> ) --> <nPreviousSize>
```

Sets the typeahead buffer to given size.

**Parâmetros:**

  - `<nSize>` — <nSize> is a valid typeahead size.

---

## `Version`

```
Version() --> <cReturn>
```

Returns the version of Harbour compiler

---
