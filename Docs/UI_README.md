# UI

> Saída em tela, cursor, cor, mouse, menus e browser de dados.

**Funções neste módulo:** 18

---

## `__AtPrompt`

```
__AtPrompt( <nRow>, <nCol>, <cPrompt>, [<xMsg>] ) --> .F.
```

Display a menu item on screen and define a message

**Parâmetros:**

  - `<nRow>` — <nRow> is the row number to display the menu <cPrompt>. Value could range from zero to MaxRow().
  - `<nCol>` — <nCol> is the column number to display the menu <cPrompt>. Value could range from zero to MaxCol().
  - `<cPrompt>` — <cPrompt> is the menu item character string to display.
  - `<xMsg>` — <xMsg> define a message to display each time this menu item is

---

## `__MenuTo`

```
__MenuTo( <bBlock>, <cVariable> ) --> nChoice
```

Invoked a menu defined by set of @...PROMPT

**Parâmetros:**

  - `<bBlock>` — <bBlock> is a set/get code block for variable named <cVariable>.
  - `<cVariable>` — <cVariable> is a character string that contain the name of the variable to hold the menu choices, if this variable does not exist

---

## `__XHelp`

```
__XHelp() --> <xValue>
```

Determines whether a HELP() user defined function exists.

---

## `__XRestScreen`

```
__XRestScreen()
```

Restore screen image and coordinate from an internal buffer

---

## `__XSaveScreen`

```
__XSaveScreen()
```

Save whole screen image and coordinate to an internal buffer

---

## `Alert`

```
Alert( <xMessage>, [<aOptions>], [<cColorNorm>], [<nDelay>] ) --> nChoice or NIL
```

Display a dialog box with a message

**Parâmetros:**

  - `<xMessage>` — <xMessage> Message to display in the dialog box. <xMessage> can be of any Harbour type.
  - `<aOptions>` — <aOptions> Array with available response. Each element should be Character string. If omitted, default is { "Ok" }.
  - `<cColorNorm>` — <cColorNorm> Color string to paint the dialog box with. If omitted, default color is "W+/R".
  - `<nDelay>` — <nDelay> Number of seconds to wait to user response before abort. Default value is 0, that wait forever.

---

## `Browse`

```
Browse( [<nTop>, <nLeft>, <nBottom>, <nRight>] ) --> lOk
```

Browse a database file

**Parâmetros:**

  - `<nTop>` — <nTop> coordinate for top row display.
  - `<nLeft>` — <nLeft> coordinate for left column display.
  - `<nBottom>` — <nBottom> coordinate for bottom row display.
  - `<nRight>` — <nRight> coordinate for right column display.

---

## `Col`

```
Col() --> nPosition
```

Returns the current screen column position

---

## `DevOutPict`

```
DevOutPict( <xExp>, <cPicture>, [<cColorString>] )
```

Displays a value to a device using a picture template

**Parâmetros:**

  - `<xExp>` — <xExp> is any valid expression.
  - `<cPicture>` — <cPicture> is any picture transformation that Transform() can use.
  - `<cColorString>` — <cColorString> is an optional string that specifies a screen color to use in place of the default color when the output goes to the screen.

---

## `hb_ColorIndex`

```
hb_ColorIndex( <cColorSpec>, <nIndex> ) --> <cColor>
```

Extract one color from a full colorspec string.

**Parâmetros:**

  - `<cColorSpec>` — <cColorSpec> is a color list
  - `<nIndex>` — <nIndex> is the position of the color item to be extracted, the first position is the zero.

---

## `MaxCol`

```
MaxCol() --> nPosition
```

Returns the maximun number of columns in the current video mode

---

## `MaxRow`

```
MaxRow() --> nPosition
```

Returns the current screen row position

---

## `MCol`

```
MCol() --> nMouseColumn
```

Returns the mouse cursor column position.

---

## `MRow`

```
MRow() --> nMouseRow
```

Returns the mouse cursor row position.

---

## `Row`

```
Row() --> nPosition
```

Returns the current screen row position

---

## `SetMode`

```
SetMode( <nRows>, <nCols> ) --> lSuccess
```

Change the video mode to a specified number of rows and columns

**Parâmetros:**

  - `<nRows>` — <nRows> is the number of rows for the video mode to set.
  - `<nCols>` — <nCols> is the number of columns for the video mode to set.

---

## `TBrowseDB`

```
TBrowseDB( [<nTop>], [<nLeft>], [<nBottom>], [<nRight>] ) --> oBrowse
```

Create a new TBrowse object to be used with database file

**Parâmetros:**

  - `<nTop>` — <nTop> coordinate for top row display.
  - `<nLeft>` — <nLeft> coordinate for left column display.
  - `<nBottom>` — <nBottom> coordinate for bottom row display.
  - `<nRight>` — <nRight> coordinate for right column display.

---

## `Tone`

```
Tone( <nFrequency>, <nDuration> ) --> NIL
```

Sound a tone with a specified frequency and duration.

**Parâmetros:**

  - `<nFrequency>` — <nFrequency>  A non-negative numeric value that specifies the frequency of the tone in hertz.
  - `<nDuration>` — <nDuration>   A positive numeric value which specifies the duration of the tone in 1/18 of a second units.

---
