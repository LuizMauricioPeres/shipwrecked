# OOP

> Introspecção e manipulação de objetos em tempo de execução: métodos, dados e herança.

**Funções neste módulo:** 15

---

## `__objAddData`

```
__objAddData( <oObject>, <cDataName> ) --> oObject
```

Add a VAR to an already existing class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cDataName>` — <cDataName> is the symbol name of the new VAR to add.

---

## `__objAddInline`

```
__objAddInline( <oObject>, <cInlineName>, <bInline> ) --> oObject
```

Add an INLINE to an already existing class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cInlineName>` — <cInlineName> is the symbol name of the new INLINE to add.
  - `<bInline>` — <bInline> is a code block to associate with the INLINE method.

---

## `__objAddMethod`

```
__objAddMethod( <oObject>, <cMethodName>, <nFuncPtr> ) --> oObject
```

Add a METHOD to an already existing class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cMethodName>` — <cMethodName> is the symbol name of the new METHOD to add.
  - `<nFuncPtr>` — <nFuncPtr> is a pointer to a function to associate with the method.

---

## `__objDelInline`

```
__objDelInline( <oObject>, <cSymbol> ) --> oObject
```

Delete a METHOD INLINE from class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cSymbol>` — <cSymbol> is the symbol name of METHOD or INLINE method to be deleted (removed) from the object.

---

## `__objDelMethod`

```
__objDelMethod( <oObject>, <cSymbol> ) --> oObject
```

Delete a METHOD  from class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cSymbol>` — <cSymbol> is the symbol name of METHOD or INLINE method to be deleted (removed) from the object.

---

## `__objDelMethod`

```
__objDelMethod( <oObject>, <cDataName> ) --> oObject
```

Delete a VAR (instance variable) from class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cDataName>` — <cDataName> is the symbol name of VAR to be deleted (removed) from the object.

---

## `__objDerivedFrom`

```
__objDerivedFrom( <oObject>, <xSuper> ) --> lIsParent
```

Determine whether a class is derived from another class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to check.
  - `<xSuper>` — <xSuper> is the object that may be a parent. <xSuper> can be either an Object or a Character string with the class name.

---

## `__objGetMethodList`

```
__objGetMethodList( <oObject> ) --> aMethodNames
```

Return names of all METHOD for a given object

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to scan.

---

## `__objGetMsgList`

```
__objGetMsgList( <oObject>, [<lData>], [nClassType] ) --> aNames
```

Return names of all VAR or METHOD for a given object

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to scan.
  - `<lData>` — <lData> is an optional logical value that specifies the information to return. A value of .T. instruct the function to return list of all VAR names, .F. return list of all METHOD names. Default value is .T.
  - `<nClassType>` — <nClassType> is on optional numeric code for selecting which class type to return. Default value is HB_MSGLISTALL, returning the whole list.

---

## `__objGetValueList`

```
__objGetValueList( <oObject>, [<aExcept>] ) --> aData
```

Return an array of VAR names and values for a given object

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to scan.
  - `<aExcept>` — <aExcept> is an optional array with VAR names you want to exclude from the scan.

---

## `__objHasData`

```
__objHasData( <oObject>, <cSymbol> ) --> lExist
```

Determine whether a symbol exist in object as VAR

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to scan.
  - `<cSymbol>` — <cSymbol> is the name of the symbol to look for.

---

## `__objHasMethod`

```
__objHasMethod( <oObject>, <cSymbol> ) --> lExist
```

Determine whether a symbol exist in object as METHOD

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to scan.
  - `<cSymbol>` — <cSymbol> is the name of the symbol to look for.

---

## `__objModInline`

```
__objModInline( <oObject>, <cInlineName>, <bInline> ) --> oObject
```

Modify (replace) an INLINE method in an already existing class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cInlineName>` — <cInlineName> is the symbol name of the INLINE method to modify.
  - `<bInline>` — <bInline> is a new code block to associate with the INLINE method.

---

## `__objModMethod`

```
__objModMethod( <oObject>, <cMethodName>, <nFuncPtr> ) --> oObject
```

Modify (replace) a METHOD in an already existing class

**Parâmetros:**

  - `<oObject>` — <oObject> is the object to work on.
  - `<cMethodName>` — <cMethodName> is the symbol name of the METHOD to modify.
  - `<nFuncPtr>` — <nFuncPtr> is a pointer to a new function to associate with the method.

---

## `__objSetValueList`

```
__objSetValueList( <oObject>, <aData> ) --> oObject
```

Set object with an array of VAR names and values

**Parâmetros:**

  - `<oObject>` — <oObject> is an object to set.
  - `<aData>` — <aData> is a 2D array with a pair of instance variables and values for setting those variable.

---
