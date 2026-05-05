# IO

> Saída para stdout/stderr, execução de comandos e encerramento do processo.

**Funções neste módulo:** 5

---

## `__Quit`

```
__Quit()
```

Terminates an application.

---

## `__Run`

```
__Run( <cCommand> )
```

Run an external program.

**Parâmetros:**

  - `<cCommand>` — <cCommand> Command to execute.

---

## `__TypeFile`

```
__TypeFile( <cFile>, [<lPrint>] ) --> NIL
```

Show the content of a file on the console and/or printer

**Parâmetros:**

  - `<cFile>` — <cFile> is a name of the file to display. If the file have an extension, it must be specified (there is no default value).
  - `<lPrint>` — <lPrint> is an optional logical value that specifies whether the output should go only to the screen (.F.) or to both the screen and printer (.T.), the default is (.F.).

---

## `OutErr`

```
OutErr( <xExp,...> )
```

Write a list of values to the standard error device

**Parâmetros:**

  - `<xExp,...>` — <xExp,...> is a list of expressions to display. Expressions are any mixture of Harbour data types.

---

## `OutStd`

```
OutStd( <xExp,...> )
```

Write a list of values to the standard output device

**Parâmetros:**

  - `<xExp,...>` — <xExp,...> is a list of expressions to display. Expressions are any mixture of Harbour data types.

---
