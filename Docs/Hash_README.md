# Hash

> Tabelas hash (dicionários chave-valor): criação, acesso, iteração e ordenação.

**Funções neste módulo:** 25

---

## `hb_HAllocate`

```
hb_HAllocate( <hsTable>, <nItems> )
```

Preallocates a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<nItems>` — <nItems> number of items to preallocate in the hash table

---

## `hb_Hash`

```
hb_Hash( [ <Key1>, <Value1> ], [ <KeyN>, <ValueN> ], ... ) -> hsTable
```

Returns a hash table

**Parâmetros:**

  - `<Key1>` — <Key1> entry key; can be of type: number, date, datetime, string, pointer
  - `<Value1>` — <Value1> entry value; can be of type: block, string, numeric, date/datetime, logical, nil, pointer, array, hash table Equivalent to: hsTable := { => }

---

## `hb_HAutoAdd`

```
hb_HAutoAdd( <hsTable>, [<lFlag>] ) -> <lPreviousFlag>
```

Sets the 'auto add' flag for the hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<lFlag>` — <lFlag> a logical value indicating to turn on or off the 'auto add' flag of the hash table

---

## `hb_HBinary`

```
hb_HBinary( <hsTable>, [<lFlag>] ) -> <lPreviousFlag>
```

Sets the 'binary' flag for the hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<lFlag>` — <lFlag> a logical value indicating to turn on or off the 'binary' flag of the hash table

---

## `hb_HCaseMatch`

```
hb_HCaseMatch( <hsTable>, [<lFlag>] ) -> <lPreviousFlag>
```

Sets the 'case match' flag for the hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<lFlag>` — <lFlag> a logical value indicating to turn on or off the 'case match' flag of the hash table

---

## `hb_HClone`

```
hb_HClone( <hsTable> ) -> <hsDestination>
```

Creates a copy of a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table

---

## `hb_HCopy`

```
hb_HCopy( <hsDestination>, <hsSource>, [<nStart>], [<nCount>] ) -> <hsDestination>
```

Adds entries from the source hash table to the destination hash table

**Parâmetros:**

  - `<hsDestination>` — <hsDestination> a destination hash table
  - `<hsSource>` — <hsSource> a source hash table
  - `<nStart>` — <nStart> starting index, defaults to 1 if omitted
  - `<nCount>` — <nCount> counter, defaults to (length) - <nStart> is omitted

---

## `hb_HDefault`

```
hb_HDefault( <hsTable>, <DefaultValue> ) -> <OldDefaultValye>
```

Returns/sets a default value for a hash table.

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<DefaultValue>` — <DefaultValue>

---

## `hb_HDel`

```
hb_HDel( <hsTable>, <Key> ) -> <hsTable>
```

Removes a key/value pair from a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> key to be removed from the hash table; can be of type: number, date, datetime, string, pointer

---

## `hb_HDelAt`

```
hb_HDelAt( <hsTable>, <nPosition> ) -> <hsTable>
```

Removes an entry from a hash table based on its index position

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<nPosition>` — <nPosition> the position of an entry within the hash table that will be deleted

---

## `hb_HEval`

```
hb_HEval( <hsTable>, <bBlock>, [<nStart>], [<nCount>] ) -> <hsTable>
```

Evaluate a code block across the contents of a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<bBlock>` — <bBlock> code block to be evaluated
  - `<nStart>` — <nStart> starting index, defaults to 1 if omitted
  - `<nCount>` — <nCount> counter, defaults to (length) - <nStart> is omitted

---

## `hb_HFill`

```
hb_HFill( <hsTable>, <Value> ) -> <hsTable>
```

Fills a hash table with a value

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Value>` — <Value> fill value; can be of type: block, string, numeric, date/datetime, logical, nil, pointer, array, hash table

---

## `hb_HGet`

```
hb_HGet( <hsTable>, <Key> ) -> <Value>
```

Returns a hash value

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> key to be retrieve from the hash table; can be of type: number, date, datetime, string, pointer

---

## `hb_HGetDef`

```
hb_HGetDef( <hsTable>, <Key>, [<DefaultValue>] ) -> <Value>
```

Returns a hash value, or a default value if the key is not present

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> key to be retrieve from the hash table; can be of type: number, date, datetime, string, pointer
  - `<DefaultValue>` — <DefaultValue> a default value to be returned if the hash table does not contain the key

---

## `hb_HHasKey`

```
hb_HHasKey( <hsTable>, <Key> ) -> lExists
```

Determines whether a hash table has an entry with a give key

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> a key value to be queried for; can be of type: number, date, datetime, string, pointer

---

## `hb_HKeyAt`

```
hb_HKeyAt( <hsTable>, <nPosition> ) -> <Key>
```

Gets a hash table key at a given position

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<nPosition>` — <nPosition> the position of an entry within the hash table that will be returned

---

## `hb_HKeys`

```
hb_HKeys( <hsTable> ) -> <aKeys>
```

Returns an array of the keys of a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table

---

## `hb_HMerge`

```
hb_HMerge( <hsDestination>, <hsSource>, <bBlock>|<nPosition> ) -> <hsDestination>
```

Merges a source hash table into a destination hash table

**Parâmetros:**

  - `<hsDestination>` — <hsDestination> a destination hash table
  - `<hsSource>` — <hsSource> a source hash table
  - `<bBlock>` — <bBlock> a code block that will be evaluated for each entry within the source hash table; the code block will be passed the entry key, value and position; if the code block returns a true value, the entry will be added to the destination hash table
  - `<nPosition>` — <nPosition> the position of an entry within the source hash table that will be appended to the destination hash table TODO: the source code passes either a number or HB_HASH_UNION; research this

---

## `hb_HPairAt`

```
hb_HPairAt( <hsTable>, <nPosition> ) -> <aKeyValue>
```

Returns a two-dimensional array of a hash table entry key/value pair

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<nPosition>` — <nPosition> the position of an entry within the hash table that will be returned

---

## `hb_HPos`

```
hb_HPos( <hsTable>, <Key> ) -> nPosition
```

Locates the index of a key within a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> key for which its position is to be determined; can be of type: number, date, datetime, string, pointer

---

## `hb_HScan`

```
hb_HScan( <hsTable>, <Value>, [<nStart>], [<nCount>, [<lExact>] ) -> nPosition
```

Scans a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Value>` — <Value> to be located within the hash table
  - `<nStart>` — <nStart> starting index, defaults to 1 if omitted
  - `<nCount>` — <nCount> counter, defaults to (length) - <nStart> is omitted
  - `<lExact>` — <lExact> logical valuye indicating whether the comparision is to be be exact or not

---

## `hb_HSet`

```
hb_HSet( <hsTable>, <Key>, <Value> ) -> <hsTable>
```

Sets a hash value

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<Key>` — <Key> the key of the entry to be set; can be of type: number, date, datetime, string, pointer
  - `<Value>` — <Value> the entry value

---

## `hb_HSort`

```
hb_HSort( <hsTable> ) -> <hsSortedTable>
```

Reorganizes the internal list of the hash table to be sorted

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table

---

## `hb_HValueAt`

```
hb_HValueAt( <hsTable>, <nPosition>, [<NewValue>] ) -> <Value>
```

Gets/sets a hash value at a given position

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table
  - `<nPosition>` — <nPosition> the position of an entry within the hash table that will be returned
  - `<NewValue>` — <NewValue> a new value to be assigned to the hash table at the given position

---

## `hb_HValues`

```
hb_HValues( <hsTable> ) -> <aValues>
```

Returns an array of the values of a hash table

**Parâmetros:**

  - `<hsTable>` — <hsTable> a hash table

---
