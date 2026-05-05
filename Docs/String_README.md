# String

> Manipulação de strings: busca, fatiamento, padding, formatação e verificações de tipo de caractere.

**Funções neste módulo:** 36

---

## `AllTrim`

```
AllTrim( <cString> ) --> cExpression
```

Removes leading and trailing blank spaces from a string

**Parâmetros:**

  - `<cString>` — <cString> Any character string

---

## `Asc`

```
Asc( <cCharacter> ) --> nAscNumber
```

Returns the ASCII value of a character

**Parâmetros:**

  - `<cCharacter>` — <cCharacter> Any character expression

---

## `At`

```
At( <cSearch>, <cString> ) --> nPos
```

Locates the position of a substring in a main string.

**Parâmetros:**

  - `<cSearch>` — <cSearch> Substring to search for
  - `<cString>` — <cString> Main string

---

## `Chr`

```
Chr( <nAsciiNum> ) --> cReturn
```

Converts an ASCII value to it character value

**Parâmetros:**

  - `<nAsciiNum>` — <nAsciiNum> Any ASCII character code.

---

## `HardCR`

```
HardCR( <cString> ) --> <cConvertedString>
```

Replace all soft carriage returns with hard carriages returns.

**Parâmetros:**

  - `<cString>` — <cString> is a string of chars to convert.

---

## `hb_At`

```
hb_At( <cSearch>, <cString>, [<nStart>], [<nEnd>] ) --> nPos
```

Locates the position of a substring in a main string.

**Parâmetros:**

  - `<cSearch>` — <cSearch> Substring to search for
  - `<cString>` — <cString> Main string
  - `<nStart>` — <nStart> First position to search in cString, by default 1
  - `<nEnd>` — <nEnd> End position to search, by default cString length

---

## `hb_RAt`

```
hb_RAt( <cSearch>, <cString>, [<nStart>], [<nEnd>]  ) --> nPos
```

Searches for last occurrence a substring of a string.

**Parâmetros:**

  - `<cSearch>` — <cSearch> Substring to search for
  - `<cString>` — <cString> Main string
  - `<nStart>` — <nStart> First position to search in cString, by default 1.
  - `<nEnd>` — <nEnd> End position to search, by default cString length

---

## `hb_Translate`

```
hb_Translate( <cSrcText>, [<cPageFrom>], [<cPageTo>] ) --> cDstText
```

Translate a string from one code page to the other

**Parâmetros:**

  - `<cSrcText>` — <cSrcText> Is the source string to translate.
  - `<cPageFrom>` — <cPageFrom> Is the optional character code page ID of the source string. If not specified, the default code page is used.
  - `<cPageTo>` — <cPageTo> Is the optional character code page ID of the destination string. If not specified, the default code page is used.

---

## `hb_ValToStr`

```
hb_ValToStr( <xValue> ) --> cString
```

Converts any scalar type to a string.

**Parâmetros:**

  - `<xValue>` — <xValue> is any scalar argument.

---

## `IsAffirm`

```
IsAffirm( <cChar> ) --> <lTrueOrFalse>
```

Checks if passed char is an affirmation char

**Parâmetros:**

  - `<cChar>` — <cChar> is a char or string of chars   </par>

---

## `IsAlpha`

```
IsAlpha( <cString> ) --> lAlpha
```

Checks if leftmost character in a string is an alphabetic character

**Parâmetros:**

  - `<cString>` — <cString> Any character string

---

## `IsDigit`

```
IsDigit( <cString> ) --> lDigit
```

Checks if leftmost character is a digit character

**Parâmetros:**

  - `<cString>` — <cString> Any character string

---

## `IsLower`

```
IsLower( <cString> ) --> lLower
```

Checks if leftmost character is an lowercased letter.

**Parâmetros:**

  - `<cString>` — <cString> Any character string

---

## `IsNegative`

```
IsNegative( <cChar> ) --> <lTrueOrFalse>
```

Checks if passed char is a negation char.

**Parâmetros:**

  - `<cChar>` — <cChar> is a char or string of chars   </par>

---

## `IsUpper`

```
IsUpper( <cString> ) --> lUpper
```

Checks if leftmost character is an uppercased letter.

**Parâmetros:**

  - `<cString>` — <cString> Any character string

---

## `Left`

```
Left( <cString>, <nLen> ) --> cReturn
```

Extract the leftmost substring of a character expression

**Parâmetros:**

  - `<cString>` — <cString> Main character to be parsed
  - `<nLen>` — <nLen> Number of bytes to return beginning at the leftmost position

---

## `Len`

```
Len( <cString> | <aArray> ) --> <nLength>
```

Returns size of a string or size of an array.

**Parâmetros:**

  - `<acString>` — <acString> is a character string or the array to check.

---

## `Lower`

```
Lower( <cString> ) --> cLowerString
```

Universally lowercases a character string expression.

**Parâmetros:**

  - `<cString>` — <cString> Any character expression.

---

## `LTrim`

```
LTrim( <cString> ) --> cReturn
```

Removes leading spaces from a string

**Parâmetros:**

  - `<cString>` — <cString>  Character expression with leading spaces

---

## `PadC`

```
PadC( <xVal>, <nWidth>, <cFill> ) --> cString
```

Centers an expression for a given width

**Parâmetros:**

  - `<xVal>` — <xVal> A Number, Character or Date value to pad
  - `<nWidth>` — <nWidth> Width of output string
  - `<cFill>` — <cFill> Character to fill in the string

---

## `PadL`

```
PadL( <xVal>, <nWidth>, <cFill> ) --> cString
```

Left-justifies an expression for a given width

**Parâmetros:**

  - `<xVal>` — <xVal> An number, Character or date to pad
  - `<nWidth>` — <nWidth> Width of output string
  - `<cFill>` — <cFill> Character to fill in the string

---

## `PadR`

```
PadR( <xVal>, <nWidth>, <cFill> ) --> cString
```

Right-justifies an expression for a given width

**Parâmetros:**

  - `<xVal>` — <xVal> A Number, Character or Date value to pad
  - `<nWidth>` — <nWidth> Width of output string
  - `<cFill>` — <cFill> Character to fill in the string

---

## `RAt`

```
RAt( <cSearch>, <cString> ) --> nPos
```

Searches for last occurrence a substring of a string.

**Parâmetros:**

  - `<cSearch>` — <cSearch> Substring to search for
  - `<cString>` — <cString> Main string

---

## `Replicate`

```
Replicate( <cString>, <nSize> ) --> cReplicateString
```

Repeats a single character expression

**Parâmetros:**

  - `<cString>` — <cString> Character string to be replicated
  - `<nSize>` — <nSize> Number of times to replicate <cString>

---

## `Right`

```
Right( <cString>, <nLen> ) --> cReturn
```

Extract the rightmost substring of a character expression

**Parâmetros:**

  - `<cString>` — <cString> Character expression to be parsed
  - `<nLen>` — <nLen> Number of bytes to return beginning at the rightmost position

---

## `RTrim`

```
RTrim( <cExpression> ) --> cString
```

Remove trailing spaces from a string.

**Parâmetros:**

  - `<cExpression>` — <cExpression> Any character expression

---

## `Space`

```
Space( <nSize> ) --> cString
```

Returns a string of blank spaces

**Parâmetros:**

  - `<nSize>` — <nSize> The length of the string

---

## `Str`

```
Str( <nNumber>, [<nLength>], [<nDecimals>] ) --> cNumber
```

Convert a numeric expression to a character string.

**Parâmetros:**

  - `<nNumber>` — <nNumber> is the numeric expression to be converted to a character string.
  - `<nLength>` — <nLength> is the length of the character string to return, including decimal digits, decimal point, and sign.
  - `<nDecimals>` — <nDecimals> is the number of decimal places to return.

---

## `StrTran`

```
StrTran( <cString>, <cLocString>, [<cRepString>], [<nPos>], [<nOccurrences>] ) --> cReturn
```

Translate substring value with a main string

**Parâmetros:**

  - `<cString>` — <cString>     The main string to search
  - `<cLocString>` — <cLocString>  The string to locate in the main string
  - `<cRepString>` — <cRepString>  The string to replace the <cLocString>
  - `<nPos>` — <nPos>        The first occurrence to be replaced
  - `<nOccurrences>` — <nOccurrences> Number of occurrence to replace

---

## `StrZero`

```
StrZero( <nNumber>, [<nLength>], [<nDecimals>] ) --> cNumber
```

Convert a numeric expression to a character string, zero padded.

**Parâmetros:**

  - `<nNumber>` — <nNumber> is the numeric expression to be converted to a character string.
  - `<nLength>` — <nLength> is the length of the character string to return, including decimal digits, decimal point, and sign.
  - `<nDecimals>` — <nDecimals> is the number of decimal places to return.

---

## `SubStr`

```
SubStr( <cString>, <nStart>, [<nLen>] ) --> cReturn
```

Returns a substring from a main string

**Parâmetros:**

  - `<cString>` — <cString> Character expression to be parsed
  - `<nStart>` — <nStart> Start position
  - `<nLen>` — <nLen> Number of characters to return

---

## `Transform`

```
Transform( <xExpression>, <cTemplate> ) --> cFormatted
```

Formats a value based on a specific picture template.

**Parâmetros:**

  - `<xExpression>` — <xExpression> Any expression to be formated.
  - `<cTemplate>` — <cTemplate> Character string with picture template

---

## `Trim`

```
Trim( <cExpression> ) --> cString
```

Remove trailing spaces from a string.

**Parâmetros:**

  - `<cExpression>` — <cExpression> Any character expression

---

## `Upper`

```
Upper( <cString> ) --> cUpperString
```

Converts a character expression to uppercase format

**Parâmetros:**

  - `<cString>` — <cString> Any character expression.

---

## `Val`

```
Val( <cNumber> ) --> nNumber
```

Convert a number from a character type to numeric

**Parâmetros:**

  - `<cNumber>` — <cNumber> Any valid character string of numbers.

---

## `Word`

```
Word( <nDouble> ) --> <nInteger>
```

Converts double to integer values.

**Parâmetros:**

  - `<nDouble>` — <nDouble> is a numeric double value.

---
