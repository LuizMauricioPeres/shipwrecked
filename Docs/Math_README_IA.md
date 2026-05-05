# Math — Harbour functions for SWed transpiler
# Math ops: round, sqrt, log, abs, min/max.
# count:9

Abs|Abs( <nNumber> ) --> <nAbsNumber>|Return the absolute value of a number.|ARGS:<nNumber>:<nNumber> Any number.
Exp|Exp( <nNumber> ) --> <nValue>|Calculates the value of e raised to the passed power.|ARGS:<nNumber>:<nNumber> Any  real number.
Int|Int( <nNumber> ) --> <nIntNumber>|Return the integer port of a numeric value.|ARGS:<nNumber>:<nNumber> Any  numeric value.
Log|Log( <nNumber> ) --> <nLog>|Returns the natural logarithm of a number.|ARGS:<nNumber>:<nNumber> Any numeric expression.
Max|Max( <xValue>, <xValue1> ) --> <xMax>|Returns the maximum of two numbers or dates.|ARGS:<xValue>:<xValue>  Any date or numeric value.; <xValue1>:<xValue1> Any date or numeric value (same type as <xValue>).
Min|Min( <xValue>, <xValue1> ) --> <xMin>|Determines the minumum of two numbers or dates.|ARGS:<xValue>:<xValue>  Any date or numeric value.; <xValue1>:<xValue1> Any date or numeric value.
Mod|Mod( <nNumber>, <nNumber1> ) -->  <nRemainder>|Return the modulus of two numbers.|ARGS:<nNumber>:<nNumber>  Numerator in a divisional expression.; <nNumber1>:<nNumber1> Denominator in a divisional expression.
Round|Round( <nNumber>, <nPlace> ) --> <nResult>|Rounds off a numeric expression.|ARGS:<nNumber>:<nNumber> Any numeric value.; <nPlace>:<nPlace>  The number of places to round to.
Sqrt|Sqrt( <nNumber> ) --> <nSqrt>|Calculates the square root of a number.|ARGS:<nNumber>:<nNumber> Any  numeric value.