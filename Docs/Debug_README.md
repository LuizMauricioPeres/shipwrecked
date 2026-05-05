# Debug

> Inspeção da call stack: nome do procedimento, arquivo e linha.

**Funções neste módulo:** 3

---

## `ProcFile`

```
ProcFile( <xExp> ) --> <cEmptyString>
```

This function allways returns an empty string.

**Parâmetros:**

  - `<xExp>` — <xExp> is any valid type.

---

## `ProcLine`

```
ProcLine( <nLevel> ) --> <nLine>
```

Gets the line number of the current function on the stack.

**Parâmetros:**

  - `<nLevel>` — <nLevel> is the function level required.

---

## `ProcName`

```
ProcName( <nLevel> ) --> <cProcName>
```

Gets the name of the current function on the stack

**Parâmetros:**

  - `<nLevel>` — <nLevel> is the function level required.

---
