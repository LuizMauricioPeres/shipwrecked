# Database

> Acesso a work areas, navegação em DBF, filtros, lock e estrutura de tabelas.

**Funções neste módulo:** 55

---

## `__dbCopyStruct`

```
__dbCopyStruct( <cFileName>, [<aFieldList>] )
```

Create a new database based on current database structure

**Parâmetros:**

  - `<cFileName>` — <cFileName> is the name of the new database file to create. (.dbf) is the default extension if none is given.
  - `<aFieldList>` — <aFieldList> is an array where each element is a field name. Names could be specified as uppercase or lowercase.

---

## `__dbCopyXStruct`

```
__dbCopyXStruct( <cFileName> ) --> lSuccess
```

Copy current database structure into a definition file

**Parâmetros:**

  - `<cFileName>` — <cFileName> is the name of target definition file to create. (.dbf) is the default extension if none is given.

---

## `__dbCreate`

```
__dbCreate( <cFileName>, [<cFileFrom>], [<cRDDName>], [<lNew>], [<cAlias>] ) --> lUsed
```

Create structure extended file or use one to create new file

**Parâmetros:**

  - `<cFileName>` — <cFileName> is the target file name to create and then open. (.dbf) is the default extension if none is given.
  - `<cFileFrom>` — <cFileFrom> is an optional structure extended file name from which
  - `<cRDDName>` — <cRDDName> is RDD name to create target with. If omitted, the default RDD is used.
  - `<lNew>` — <lNew> is an optional logical expression, (.T.) opens the target file
  - `<cAlias>` — <cAlias> is an optional alias to USE the target file with. If not

---

## `__dbDelim`

```
__dbDelim( <lExport>, <xcFile>, [<xcDelim>], [<aFields>], [<bFor>], [<bWhile>], [<nNext>], [<nRecord>], <lRest>  )
```

Copies the contents of a database to a delimited text file or appends the contents of a delimited text file to a database.

**Parâmetros:**

  - `<lExport>` — <lExport> If set to .T., copies records to a delimited file. If set to .F., append records from a delimited file.
  - `<xcFile>` — <xcFile> The name of the text file to copy to or append from. If a file extension is not specified, ".txt" is used by default.
  - `<xcDelim>` — <xcDelim> Either the character to use as the character field delimiter (only the first character is used). or "BLANK" (not case sensitive), which eliminates the character field delimiters and sets the field separator to a single space instead of a comma.
  - `<aFields>` — <aFields> An aray of field names to limit the processint to. If not specified, or if empty, then all fields are processed.
  - `<bFor>` — <bFor> An optional code block containing a FOR expression that will reduce the number of records to be processed.
  - `<bWhile>` — <bWhile> An optional code block containing a WHILE expression that will reduce the number of records to be processed.
  - `<nNext>` — <nNext> If present, but nRecord is not present, specifies to process this number of records, starting with the current record. A value of 0 means to process no records.
  - `<nRecord>` — <nRecord> If present, specifies the only record to process. A
  - `<lRest>` — <lRest> If <lExport> is .T., then if <lRest> is set to .T. and there are no

---

## `__dbSDF`

```
__dbSDF( <lExport>, <xcFile>, [<aFields>], [<bFor>], [<bWhile>], [<nNext>], [<nRecord>], <lRest>  )
```

Copies the contents of a database to an SDF text file or appends the contents of an SDF text file to a database.

**Parâmetros:**

  - `<lExport>` — <lExport> If set to .T., copies records to an SDF file. If set to .F., append records from an SDF file.
  - `<xcFile>` — <xcFile> The name of the text file to copy to or append from. If a file extension is not specified, ".txt" is used by default.
  - `<aFields>` — <aFields> An aray of field names to limit the processint to. If not specified, or if empty, then all fields are processed.
  - `<bFor>` — <bFor> An optional code block containing a FOR expression that will reduce the number of records to be processed.
  - `<bWhile>` — <bWhile> An optional code block containing a WHILE expression that will reduce the number of records to be processed.
  - `<nNext>` — <nNext> If present, but <nRecord> is not present, specifies to process this number of records, starting with the current record. A value of 0 means to process no records.
  - `<nRecord>` — <nRecord> If present, specifies the only record to process. A
  - `<lRest>` — <lRest> If <lExport> is .T., then if <lRest> is set to .T. and there are no

---

## `__dbStructFilter`

```
__dbStructFilter( <aStruct>, [<aFieldList>] ) --> aStructFiltered
```

Filter a database structure array

**Parâmetros:**

  - `<aStruct>` — <aStruct> is a multidimensional array with database fields structure, which is usually the output from dbStruct(), where each array element has the following structure:
  - `<table>` — <table> Position   Description    dbstruct.ch 1          cFieldName     DBS_NAME 2          cFieldType     DBS_TYPE 3          nFieldLength   DBS_LEN 4          nDecimals      DBS_DEC
  - `</table>` — </table>
  - `<aFieldList>` — <aFieldList> is an array where each element is a field name. Names could be specified as uppercase or lowercase.

---

## `Alias`

```
Alias( [<nWorkArea>] ) --> <cWorkArea>
```

Returns the alias name of a work area

**Parâmetros:**

  - `<nWorkArea>` — <nWorkArea> Number of a work area

---

## `Bof`

```
Bof() --> <lBegin>
```

Test for the beginning-of-file condition

---

## `dbAppend`

```
dbAppend( [<lLock>] ) --> NIL
```

Appends a new record to a database file.

**Parâmetros:**

  - `<lLock>` — <lLock> Toggle to release record locks

---

## `dbClearFilter`

```
dbClearFilter() --> NIL
```

Clears the current filter condiction in a work area

---

## `dbCloseAll`

```
dbCloseAll() --> NIL
```

Close all open files in all work areas.

---

## `dbCloseArea`

```
dbCloseArea()
```

Close a database file in a work area.

---

## `dbCommit`

```
dbCommit()
```

Updates all index and database buffers for a given workarea

---

## `dbCommitAll`

```
dbCommitAll()
```

Flushes the memory buffer and performs a hard-disk write

---

## `dbCreate`

```
dbCreate( <cDatabase>, <aStruct>, [<cDriver>], [<lOpen>], [<cAlias>] )
```

Creates an empty database from a array.

**Parâmetros:**

  - `<cDatabase>` — <cDatabase> Name of database to be create
  - `<aStruct>` — <aStruct>   Name of a multidimensional array that contains the database structure
  - `<cDriver>` — <cDriver>   Name of the RDD
  - `<lOpenNew>` — <lOpenNew>  3-way toggle to Open the file in New or Current workarea:
  - `<table-noheader>` — <table-noheader> NIL     The file is not opened. True    It is opened in a New area. False   It is opened in the current area.
  - `</table>` — </table>
  - `<cAlias>` — <cAlias>    Name of database Alias

---

## `dbDelete`

```
dbDelete()
```

Mark a record for deletion in a database.

---

## `dbEdit`

```
dbEdit( [<nTop>], [<nLeft>], [<nBottom>], [<nRight>], [<acColumns>], [<xUserFunc>], [<xColumnSayPictures>], [<xColumnHeaders>], [<xHeadingSeparators>], [<xColumnSeparators>], [<xFootingSeparators>], [<xColumnFootings>] ) --> lOk
```

Browse records in a table

**Parâmetros:**

  - `<nTop>` — <nTop> coordinate for top row display. <nTop> could range from 0 to MaxRow(), default is 0.
  - `<nLeft>` — <nLeft> coordinate for left column display. <nLeft> could range from 0 to MaxCol(), default is 0.
  - `<nBottom>` — <nBottom> coordinate for bottom row display. <nBottom> could range from 0 to MaxRow(), default is MaxRow().
  - `<nRight>` — <nRight> coordinate for right column display. <nRight> could range from 0 to MaxCol(), default is MaxCol().
  - `<acColumns>` — <acColumns> is an array of character expressions that contain database fields names or expressions to display in each column. If not specified, the default is to display all fields from the database in the current work area.
  - `<xUserFunc>` — <xUserFunc> is a name of a user defined function or a code block that would be called every time unrecognized key is been pressed or when there are no keys waiting to be processed and dbEdit() goes
  - `<xColumnSayPictures>` — <xColumnSayPictures> is an optional picture. If <xColumnSayPictures> is a character string, all columns would used this value as a
  - `<xColumnHeaders>` — <xColumnHeaders> contain the header titles for each column, if this is a character string, all columns would have that same header, if this is an array, each element is a character string that contain the header title for one column. Header may be split to more than one line by placing semicolon (;) in places where you want to break line. If omitted, the default value for each column header is taken
  - `<xHeadingSeparators>` — <xHeadingSeparators> is an array that contain characters that draw the lines separating the headers and the fields data. Instead of an array you can use a character string that would be used to display the same line for all fields. Default value is a double line.
  - `<xColumnSeparators>` — <xColumnSeparators> is an array that contain characters that draw the lines separating displayed columns. Instead of an array you can use a character string that would be used to display the same line for all fields. Default value is a single line.
  - `<xFootingSeparators>` — <xFootingSeparators> is an array that contain characters that draw the lines separating the fields data area and the footing area. Instead of an array you can use a character string that would be used to display the same line for all footers. Default is to have to no footing separators.
  - `<xColumnFootings>` — <xColumnFootings> contain the footing to be displayed at the bottom of each column, if this is a character string, all columns would have that same footer, if this is an array, each element is a character string that contain the footer for one column. Footer may be split to more than one line by placing semicolon (;) in places where you want to break line. If omitted, no footer are displayed.

---

## `dbEval`

```
dbEval( <bBlock>, [<bFor>], [<bWhile>], [<nNext>], [<nRecord>], [<lRest>] ) --> NIL
```

Performs a code block operation on the current Database

**Parâmetros:**

  - `<bBlock>` — <bBlock> Operation that is to be performed
  - `<bFor>` — <bFor> Code block for the For condition
  - `<bWhile>` — <bWhile> Code block for the WHILE condition
  - `<nNext>` — <nNext> Number of NEXT records  to process
  - `<nRecord>` — <nRecord> Record number to work on exactly
  - `<lRest>` — <lRest> Toggle to rewind record pointer

---

## `Dbf`

```
Dbf() --> <cWorkArea>
```

Alias name of a work area

---

## `dbFilter`

```
dbFilter() --> cFilter
```

Return the filter expression in a work area

---

## `dbGoBottom`

```
dbGoBottom()
```

Moves the record pointer to the bottom of the database.

---

## `dbGoto`

```
dbGoto( <xRecordNumber> )
```

Position the record pointer to a specific location.

**Parâmetros:**

  - `<xRecordNumber>` — <xRecordNumber> Record number or unique identity

---

## `dbGoTop`

```
dbGoTop()
```

Moves the record pointer to the top of the database.

---

## `dbRecall`

```
dbRecall()
```

Recalls a record previousy marked for deletion.

---

## `dbRLock`

```
dbRLock( [<xIdentity>] ) --> lSuccess
```

This function locks the record based on identity

**Parâmetros:**

  - `<xIdentity>` — <xIdentity> Record identifier

---

## `dbRLockList`

```
dbRLockList() --> aRecordLocks
```

This function return a list of locked records in the database work area

---

## `dbRUnlock`

```
dbRUnlock( [<xIdentity>] )
```

Unlocks a record based on its identifier

**Parâmetros:**

  - `<xIdentity>` — <xIdentity> Record identifier, typically a record number

---

## `dbSeek`

```
dbSeek( <expKey>, [<lSoftSeek>], [<lFindLast>] ) --> lFound
```

Searches for a value based on an active index.

**Parâmetros:**

  - `<expKey>` — <expKey> Any expression
  - `<lSoftSeek>` — <lSoftSeek> Toggle SOFTSEEK condition
  - `<lFindLast>` — <lFindLast> is an optional logical value that set the current record position to the last record if successful

---

## `dbSelectArea`

```
dbSelectArea( <xArea> ) -
```

Change to another work area

**Parâmetros:**

  - `<xArea>` — <xArea> Alias or work area

---

## `dbSetDriver`

```
dbSetDriver( [<cDriver>] ) --> cCurrentDriver
```

Establishes the RDD name for the selected work area

**Parâmetros:**

  - `<cDriver>` — <cDriver> Optional database driver name

---

## `dbSetFilter`

```
dbSetFilter( <bCondition>, [<cCondition>] )
```

Establishes a filter condition for a work area.

**Parâmetros:**

  - `<bCondition>` — <bCondition> Code block expression for filtered evaluation.
  - `<cCondition>` — <cCondition> Optional character expression of code block.

---

## `dbSkip`

```
dbSkip( [<nRecords>] )
```

Moves the record pointer in the selected work area.

**Parâmetros:**

  - `<nRecords>` — <nRecords> Numbers of records to move record pointer.

---

## `dbStruct`

```
dbStruct() --> aStruct
```

Creates a multidimensional array of a database structure.

---

## `dbUnlock`

```
dbUnlock()
```

Unlock a record or release a file lock

---

## `dbUnlockAll`

```
dbUnlockAll()
```

Unlocks all records and releases all file locks in all work areas.

---

## `dbUseArea`

```
dbUseArea( [<lNewArea>], [<cDriver>], <cName>, [<xcAlias>], [<lShared>], [<lReadonly>] )
```

Opens a work area and uses a database file.

**Parâmetros:**

  - `<lNewArea>` — <lNewArea>  A optional logical expression for the new work area
  - `<cDriver>` — <cDriver>   Database driver name
  - `<cName>` — <cName>     File Name
  - `<xcAlias>` — <xcAlias>   Alias name
  - `<lShared>` — <lShared>   Shared/exclusive status flag
  - `<lReadonly>` — <lReadonly> Read-write status flag.

---

## `Deleted`

```
Deleted() --> lDeleted
```

Tests the record's deletion flag.

---

## `Eof`

```
Eof() --> <lEnd>
```

Test for end-of-file condition.

---

## `FCount`

```
FCount() --> nFields
```

Counts the number of fields in an active database.

---

## `FieldBlock`

```
FieldBlock( <cFieldName> ) --> bFieldBlock
```

Return a code block that sets/gets a value for a given field

**Parâmetros:**

  - `<cFieldName>` — <cFieldName> is a string that contain the field name.

---

## `FieldGet`

```
FieldGet( <nField> ) --> ValueField
```

Obtains the value  of a specified field

**Parâmetros:**

  - `<nField>` — <nField> Is the numeric field position

---

## `FieldName`

```
FieldName()/Field( <nPosition> ) --> cFieldName
```

Return the name of a field at a numeric field location.

**Parâmetros:**

  - `<nPosition>` — <nPosition> Field order in the database.

---

## `FieldPos`

```
FieldPos( <cFieldName> ) --> nFieldPos
```

Return the ordinal position of a field.

**Parâmetros:**

  - `<cFieldName>` — <cFieldName> Name of a field.

---

## `FieldPut`

```
FieldPut( <nField>, <expAssign> ) --> ValueAssigned
```

Set the value of a field variable

**Parâmetros:**

  - `<nField>` — <nField> The field numeric position
  - `<expAssign>` — <expAssign> Expression to be assigned to the specified field

---

## `FieldWBlock`

```
FieldWBlock( <cFieldName>, <nWorkArea> ) --> bFieldBlock
```

Return a sets/gets code block for field in a given work area

**Parâmetros:**

  - `<cFieldName>` — <cFieldName> is a string that contain the field name.
  - `<nWorkArea>` — <nWorkArea> is the work area number in which <cFieldName> exist.

---

## `Found`

```
Found() --> lSuccess
```

Determine the success of a previous search operation.

---

## `Header`

```
Header() --> nBytes
```

Return the length of a database file header

---

## `LastRec`

```
LastRec() | RecCount()* --> nRecords
```

Returns the number of records in an active work area or database.

---

## `LUpdate`

```
LUpdate() --> dModification
```

Yields the date the database was last updated.

---

## `RecCount`

```
RecCount()* | LastRec() --> nRecords
```

Counts the number of records in a database.

---

## `RecNo`

```
RecNo() --> Identity
```

Returns the current record number or identity.

---

## `RecSize`

```
RecSize() --> nBytes
```

Returns the size of a single record in an active database.

---

## `RLock`

```
RLock() --> lSuccess
```

Lock a record in a work area

---

## `Select`

```
Select( [<cAlias>] ) --> nWorkArea
```

Returns the work area number for a specified alias.

**Parâmetros:**

  - `<cAlias>` — <cAlias> is the target work area alias name.

---

## `Used`

```
Used() --> lDbfOpen
```

Checks whether a database is in use in a work area

---
