# DateTime — Harbour functions for SWed transpiler
# Date/time: conversion, component extraction, elapsed time.
# count:15

CDoW|CDoW( <dDate> ) --> cDay|Converts a date to the day of week|ARGS:<dDate>:<dDate>  Any date expression.
CMonth|CMonth( <dDate> ) --> cMonth|Return the name of the month.|ARGS:<dDate>:<dDate>  Any date expression.
CToD|CToD( <cDateString> ) --> dDate|Converts a character string to a date expression|ARGS:<cDateString>:<cDateString> A character date in format "mm/dd/yy"
Date|Date() --> dCurDate|Return the Current OS Date
Day|Day( <cDate> ) --> nMonth|Return the numeric day of the month.|ARGS:<cDate>:<cDate> Any valid date expression.
Days|Days( <nSecs> ) --> nDay|Convert elapsed seconds into days|ARGS:<nSecs>:<nSecs> The number of seconds
DoW|DoW( <dDate> ) --> nDay|Value for the day of week.|ARGS:<dDate>:<dDate>  Any valid date expression
DToC|DToC( <dDateString> ) --> cDate|Date to character conversion|ARGS:<dDateString>:<dDateString> Any date
DToS|DToS( <dDateString> ) --> cDate|Date to string conversion|ARGS:<dDateString>:<dDateString> Any date
ElapTime|ElapTime( <cStartTime>, <cEndTime> ) --> cDiference|Calculates elapted time.|ARGS:<cStartTime>:<cStartTime> Start in time as a string format; <cEndTime>:<cEndTime>   End time as a string format
Month|Month( <dDate> ) --> nMonth|Converts a date expression to a month value|ARGS:<dDate>:<dDate> Any valid date expression
Seconds|Seconds() --> nSeconds|Returns the number of elapsed seconds past midnight.
Secs|Secs( <cTime> ) --> nSeconds|Return the number of seconds from the system date.|ARGS:<cTime>:<cTime> Character expression in a time string format
Time|Time() --> cTime|Returns the system time as a string
Year|Year( <cDate> ) --> nYear|Converts the year portion of a date into a numeric value|ARGS:<dDate>:<dDate> Any valid date expression