# Array

> Funções para criação e manipulação de arrays dinâmicos 1-based.

**Funções neste módulo:** 15

---

## `AAdd`

```
AAdd( <aArray>[, <xValue>] ) --> Value
```

Dynamically add an element to an array

**Parâmetros:**

  - `<aArray>` — <aArray> The name of an array
  - `<xValue>` — <xValue> Element to add to array <aArray>

---

## `AChoice`

```
AChoice( <nTop>, <nLeft>, <nBottom>, <nRight>, <acMenuItems>, [<alSelableItems> | <lSelableItems>], [<cUserFunction> | <bUserBlock>], [<nInitialItem>], [<nWindowRow>] ) --> nPosition
```

Allows selection of an element from an array

**Parâmetros:**

  - `<nTop>` — <nTop>           - topmost row used to display array (default 0)
  - `<nLeft>` — <nLeft>          - leftmost row used to display array (default 0)
  - `<nBottom>` — <nBottom>        - bottommost row used to display array (default MaxRow())
  - `<nRight>` — <nRight>         - rightmost row used to display array (default MaxCol())
  - `<acMenuItems>` — <acMenuItems>    - the character array of items from which to select
  - `<alSelableItems>` — <alSelableItems> - an array of items, either logical or character, which is used to determine if a particular item may be selected.  If the type of a given item is character, it is macro evaluated, and the result is expected to be a logical.  A value of .T. means that the item may be selected, .F. that it may not. (See next argument: lSelectableItems)
  - `<lSelableItems>` — <lSelableItems>  - a logical value which is used to apply to all items in acMenuItems.  If .T., all items may be selected; if .F., none may be selected. (See previous argument: alSelectableItems) Default .T.
  - `<cUserFunction>` — <cUserFunction>  - the name of a function to be called which may affect special processing of keystrokes.  It is specified without parentheses or parameters. When it is called, it will be supplied with the parameters: nMode, nCurElement, and nRowPos. Default NIL.
  - `<bUserBlock>` — <bUserBlock>     - a codeblock to be called which may affect special processing of keystrokes. It should be specified in the form {| nMode, nCurElemenet, nRowPos | ; MyFunc( nMode, nCurElemenet, nRowPos ) }. Default NIL.
  - `<nInitialItem>` — <nInitialItem>   - the number of the element to be highlighted as the current item when the array is initially displayed.  1 origin.  Default 1.
  - `<nWindowRow>` — <nWindowRow>     - the number of the window row on which the initial item is to be displayed. 0 origin.  Default 0.

---

## `AClone`

```
AClone( <aSource> ) --> aDuplicate
```

Duplicate a  multidimensional array

**Parâmetros:**

  - `<aSource>` — <aSource> Name of the array to be cloned.

---

## `ACopy`

```
ACopy( <aSource>, <aTarget>, [<nStart>], [<nCount>], [<nTargetPos>] ) --> aTarget
```

Copy elements from one array to another

**Parâmetros:**

  - `<aSource>` — <aSource> is the array to copy elements from.
  - `<aTarget>` — <aTarget> is the array to copy elements to.
  - `<nStart>` — <nStart>  is the beginning subscript position to copy from <aSource>
  - `<nCount>` — <nCount>  the number of subscript elements to copy from <aSource>.
  - `<nTargetPos>` — <nTargetPos> the starting subscript position in <aTarget> to copy elements to.

---

## `ADel`

```
ADel( <aArray>, <nPos> ) --> aTarget
```

Delete an element form an array.

**Parâmetros:**

  - `<aArray>` — <aArray> Name of array from which an element is to be removed.
  - `<nPos>` — <nPos>   Subscript of the element to be removed.

---

## `ADir`

```
ADir( [<cFileMask>], [<aName>], [<aSize>], [<aDate>], [<aTime>], [<aAttr>] ) --> nDirEnries
```

Fill pre-defined arrays with file/directory information

**Parâmetros:**

  - `<cFileMask>` — <cFileMask> File mask to include in the function return. It could contain path and standard wildcard characters as supported by your
  - `<aName>` — <aName> Array to fill with file name of files that meet <cFileMask>. Each element is a Character string and include the file name and extension without the path. The name is the long file name as reported by the OS and not necessarily the 8.3 uppercase name.
  - `<aSize>` — <aSize> Array to fill with file size of files that meet <cFileMask>. Each element is a Numeric integer for the file size in Bytes. Directories are always zero in size.
  - `<aDate>` — <aDate> Array to fill with file last modification date of files that
  - `<aTime>` — <aTime> Array to fill with file last modification time of files that
  - `<aAttr>` — <aAttr> Array to fill with attribute of files that meet <cFileMask>. Each element is a Character string, see Directory() for information

---

## `AEval`

```
AEval( <aArray>, <bBlock>, [<nStart>], [<nCount>] ) --> aArray
```

Evaluates the subscript element of an array

**Parâmetros:**

  - `<aArray>` — <aArray> Is the array to be evaluated.
  - `<bBlock>` — <bBlock> Is a code block to evaluate for each element processed.
  - `<nStart>` — <nStart> The beginning array element index to evaluate.
  - `<nCount>` — <nCount> The number of elements to process.

---

## `AFields`

```
AFields( <aNames>, [<aTypes>], [<aLen>], [<aDecs>] ) --> <nFields>
```

Fills referenced arrays with database field information

**Parâmetros:**

  - `<aNames>` — <aNames>  Array of field names
  - `<aTypes>` — <aTypes>  Array of field names
  - `<aLens>` — <aLens>  Array of field names
  - `<aDecs>` — <aDecs>  Array of field names

---

## `AFill`

```
AFill( <aArray>, <xValue>, [<nStart>], [<nCount>] ) --> aTarget
```

Fill an array with a specified value

**Parâmetros:**

  - `<aArray>` — <aArray> Name of array to be filled.
  - `<xValue>` — <xValue> Expression to be globally filled in <aArray>
  - `<nStart>` — <nStart> Subscript starting position
  - `<nCount>` — <nCount> Number of subscript to be filled

---

## `AIns`

```
AIns( <aArray>, <nPos> ) --> aTarget
```

Insert a NIL value at an array subscript position.

**Parâmetros:**

  - `<aArray>` — <aArray> Array name.
  - `<nPos>` — <nPos> Subscript position in <aArray>

---

## `Array`

```
Array( <nElements> [, <nElements>...] ) --> aArray
```

Create an uninitialized array of specified length

**Parâmetros:**

  - `<nElements>` — <nElements> is the number of elements in the specified dimension.

---

## `AScan`

```
AScan( <aTarget>, <xSearch>, [<nStart>], [<nCount>] ) --> nStoppedAt
```

Scan array elements for a specified condition

**Parâmetros:**

  - `<aTarget>` — <aTarget>   Array to be scanned.
  - `<xSearch>` — <xSearch>   Expression to search for in <aTarget>
  - `<nStart>` — <nStart>    Beginning subscript position at which to start the search.
  - `<nCount>` — <nCount>    Number of elements to scan with <aTarget>.

---

## `ASize`

```
ASize( <aArray>, <nLen> ) --> aTarget
```

Adjust the size of an array

**Parâmetros:**

  - `<aArray>` — <aArray> Name of array to be dynamically altered
  - `<nLen>` — <nLen> Numeric value representing the new size of <aArray>

---

## `ASort`

```
ASort( <aArray>, [<nStart>], [<nCount>], [<bSort>] ) --> aArray
```

Sort an array

**Parâmetros:**

  - `<aArray>` — <aArray> Array to be sorted.
  - `<nStart>` — <nStart> The first element to start the sort from, default is 1.
  - `<nCount>` — <nCount> Number of elements starting from <nStart> to sort, default is all elements.
  - `<bSort>` — <bSort> Code block for sorting order, default is ascending order {| x, y | x < y }. The code block should accept two parameters and must return .T. if the sort is in order, .F. if not.

---

## `ATail`

```
ATail( <aArray> ) --> Element
```

Returns the rightmost element of an array

**Parâmetros:**

  - `<aArray>` — <aArray> is the array.

---
