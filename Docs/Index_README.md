# Index

> Criação, configuração e consulta de índices e ordens de navegação.

**Funções neste módulo:** 11

---

## `Descend`

```
Descend( <xExp> ) --> xExpInverted
```

Inverts an expression of string, logical, date or numeric type.

**Parâmetros:**

  - `<xExp>` — <xExp> is any valid expression.

---

## `IndexExt`

```
IndexExt() --> <cExtension>
```

Returns the file extension of the index module used in an application

---

## `IndexKey`

```
IndexKey( <nOrder> ) --> <cIndexKey>
```

Yields the key expression of a specified index file.

**Parâmetros:**

  - `<nOrder>` — <nOrder>  Index order number

---

## `IndexOrd`

```
IndexOrd() --> <nPosition>
```

Returns the numeric position of the controlling index.

---

## `ordBagExt`

```
ordBagExt() --> cBagExt
```

Returns the Order Bag extension

---

## `ordBagName`

```
ordBagName( <nOrder> | <cOrderName> ) --> cOrderBagName
```

Returns the Order Bag Name.

**Parâmetros:**

  - `<nOrder>` — <nOrder> A numeric value representing the Order bag number.
  - `<cOrderName>` — <cOrderName> The character name of the Order Bag.

---

## `ordCondSet`

```
ordCondSet( [<cForCondition>], [<bForCondition>], [<lAll>], [<bWhileCondition>], [<bEval>], [<nInterval>], [<nStart>], [<nNext>], [<nRecord>], [<lRest>], [<lDescend>], [<lAdditive>], [<lCurrent>], [<lCustom>], [<lNoOptimize>] )
```

Set the Condition and scope for an order

**Parâmetros:**

  - `<cForCondition>` — <cForCondition> is a string that specifies the FOR condition for the order.
  - `<bForCondition>` — <bForCondition> is a code block that defines a FOR condition that each record within the scope must meet in order to be processed. If a record does not meet the specified condition, it is ignored and the next  record is processed.Duplicate keys values are not added to the index file when a FOR condition is Used.

---

## `ordCreate`

```
ordCreate( <cOrderBagName>,[<cOrderName>], <cExpKey>, [<bExpKey>], [<lUnique>] )
```

Create an Order in an Order Bag

**Parâmetros:**

  - `<cOrderBagName>` — <cOrderBagName>  Name of the file that contains one or more Orders.
  - `<cOrderName>` — <cOrderName> Name of the order to be created.
  - `<cExpKey>` — <cExpKey> Key value for order for each record in the current work area
  - `<bExpKey>` — <bExpKey> Code block that evaluates to a key for the order for each record in the work area.
  - `<lUnique>` — <lUnique> Toggle the unique status of the index.

---

## `ordDestroy`

```
ordDestroy( <cOrderName> [, <cOrderBagName> ] )
```

Remove an Order from an Order Bag

**Parâmetros:**

  - `<cOrderName>` — <cOrderName> Name of the order to remove
  - `<cOrderBagName>` — <cOrderBagName> Name of the order bag from which order id to be removed

---

## `ordFor`

```
ordFor( <xOrder>[, <cOrderBagName>] ) --> cForExp
```

Return the FOR expression of an Order

**Parâmetros:**

  - `<xOrder>` — <xOrder>  It the name of the target order, or the numeric position of the order.
  - `<cOrderBagName>` — <cOrderBagName> Name of the order bag.

---

## `ordKey`

```
ordKey( <cOrderName> | <nOrder> [, <cOrderBagName>] ) --> cExpKey
```

Return the key expression of an Order

**Parâmetros:**

  - `<xOrder>` — <xOrder>  It the name of the target order, or the numeric position of the order.
  - `<cOrderBagName>` — <cOrderBagName> Name of the order bag.

---
