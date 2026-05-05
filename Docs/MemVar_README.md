# MemVar

> Variáveis PRIVATE/PUBLIC e acesso ao pool de memvars por nome ou referência.

**Funções neste módulo:** 11

---

## `__mvClear`

```
__mvClear()
```

This function releases all PRIVATE and PUBLIC variables

---

## `__mvDbgInfo`

```
__mvDbgInfo( <nScope> [, <nPosition> [, @<cVarName>] ] )
```

This function returns the information about the variables for debugger

**Parâmetros:**

  - `<nScope>` — <nScope> = the scope of variables for which an information is asked Supported values (defined in hbmemvar.ch) HB_MV_PUBLIC HB_MV_PRIVATE (or any other value)
  - `<nPosition>` — <nPosition> = the position of asked variable on the list of variables with specified scope - it should start from position 1
  - `<cVarName>` — <cVarName> = the value is filled with a variable name if passed by

---

## `__mvExist`

```
__mvExist( <cVarName> ) --> <lVariableExist>
```

Determine if a given name is a PUBLIC or PRIVATE memory variable

**Parâmetros:**

  - `<cVarName>` — <cVarName> - string that specifies the name of variable to check

---

## `__mvGet`

```
__mvGet( <cVarName> [, <xValue>] ) --> <xValue>
```

This function set the value of memory variable

**Parâmetros:**

  - `<cVarName>` — <cVarName> - string that specifies the name of variable
  - `<xValue>` — <xValue>   - a value of any type that will be set - if it is not specified then NIL is assumed

---

## `__mvGet`

```
__mvGet( <cVarName> ) --> <xVar>
```

This function returns value of memory variable

**Parâmetros:**

  - `<cVarName>` — <cVarName> - string that specifies the name of variable

---

## `__mvPrivate`

```
__mvPrivate( <variable_name> )
```

This function creates a PRIVATE variable

**Parâmetros:**

  - `<variable_name>` — <variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable names No skeleton are allowed here.

---

## `__mvPublic`

```
__mvPublic( <variable_name> )
```

This function creates a PUBLIC variable

**Parâmetros:**

  - `<variable_name>` — <variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable names No skeleton are allowed here.

---

## `__mvRelease`

```
__mvRelease( <skeleton>, <include_exclude_flag> )
```

This function releases PRIVATE variables

**Parâmetros:**

  - `<skeleton>` — <skeleton> = string that contains the wildcard mask for variables' names that will be released. Supported wildcards: '*' and '?'
  - `<include_exclude_flag>` — <include_exclude_flag> = logical value that specifies if variables that match passed skeleton should be either included in deletion (if .T.) or excluded from deletion (if .F.)

---

## `__mvScope`

```
__mvScope( <cVarName> )
```

If variable exists then returns its scope.

**Parâmetros:**

  - `<cVarName>` — <cVarName> = a string with a variable name to check

---

## `__mvXRelease`

```
__mvXRelease( <variable_name> )
```

This function releases value stored in PRIVATE or PUBLIC variable

**Parâmetros:**

  - `<variable_name>` — <variable_name> = either a string that contains the variable's name or an one-dimensional array of strings with variable names No skeleton are allowed here.

---

## `MemVarBlock`

```
MemVarBlock( <cMemvarName> ) --> <bBlock>
```

Returns a codeblock that sets/gets a value of memvar variable

**Parâmetros:**

  - `<cMemvarName>` — <cMemvarName> - a string that contains the name of variable

---
