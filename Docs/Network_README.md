# Network

> Sockets TCP/UDP: criação, conexão, envio/recebimento e gerenciamento de estado.

**Funções neste módulo:** 41

---

## `hb_inetAccept`

```
hb_inetAccept( <socket> ) -> SOCKET
```

Wait until a socket is ready

---

## `hb_inetAddress`

```
hb_inetAddress( <socket> ) -> cResult
```

Get a remote server address

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetCleanup`

```
hb_inetCleanup()
```

Terminate Harbour INET support

---

## `hb_inetClearError`

```
hb_inetClearError( <socket> )
```

Clear the socket error value

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetClearPeriodCallback`

```
hb_inetClearPeriodCallback( <socket> )
```

Clear the periodic callback value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetClearTimeLimit`

```
hb_inetClearTimeLimit( <socket> )
```

Clear the time limit value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetClearTimeout`

```
hb_inetClearTimeout( <socket> )
```

Clear the timeout value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetClose`

```
hb_inetClose( <socket> ) -> nResult
```

Close an INET socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetConnect`

```
hb_inetConnect( <cAddress>, <nPort> ) -> SOCKET  hb_inetConnect( <cAddress>, <nPort>, <socket> ) -> NIL
```

Connect a socket to a remote server by IP address or name

**Parâmetros:**

  - `<cAddress>` — <cAddress>
  - `<nPort>` — <nPort>
  - `<socket>` — <socket>

---

## `hb_inetConnectIP`

```
hb_inetConnectIP( <cAddress>, <nPort> ) -> SOCKET  hb_inetConnectIP( <cAddress>, <nPort>, <socket> ) -> NIL
```

Connect to a remote server by IP address

**Parâmetros:**

  - `<cAddress>` — <cAddress>
  - `<nPort>` — <nPort>
  - `<socket>` — <socket>

---

## `hb_inetCount`

```
hb_inetCount( <socket> ) -> nResult
```

Get the number of bytes last read or sent

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetCreate`

```
hb_inetCreate( [ <nTimeout> ] ) -> SOCKET
```

Create an INET socket

**Parâmetros:**

  - `<nTimeout>` — <nTimeout> Socket timeout (optional) TODO: what is the scale (seconds, milliseconds?)

---

## `hb_inetCRLF`

```
hb_inetCRLF() -> cResult
```

Get a CRLF sequence for internet protocols

---

## `hb_inetDataReady`

```
hb_inetDataReady( <socket>, [ <nMillisec> ] ) -> nResult
```

Get whether there is data ready in a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<nMillisec>` — <nMillisec>

---

## `hb_inetDGram`

```
hb_inetDGram( [<lBroadcast>] ) -> SOCKET
```

Create a datagram socket

---

## `hb_inetDGramBind`

```
hb_inetDGramBind( <nPort>, [<cAddress> [, <lBroadcast>] ] ) -> SOCKET
```

Create a bound datagram socket

**Parâmetros:**

  - `<nPort>` — <nPort>
  - `<cAddress>` — <cAddress>
  - `<bBroadcast>` — <bBroadcast>

---

## `hb_inetDGramRecv`

```
hb_inetDGramRecv( <socket>, @<cBuffer> [, <nSize> ] ) -> nBytesRead
```

Get data from a datagram socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cBuffer>` — <cBuffer> is the target buffer and must be passed by reference
  - `<nSize>` — <nSize>

---

## `hb_inetDGramSend`

```
hb_inetDGramSend( <socket>, <cAddress>, <nPort>, <cBuffer> [, <nSize> ] ) -> nBytesSent
```

Send data to a datagram socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cAddress>` — <cAddress>
  - `<nPort>` — <nPort>
  - `<cBuffer>` — <cBuffer>
  - `<nSize>` — <nSize>

---

## `hb_inetErrorCode`

```
hb_inetErrorCode( <socket> ) -> nResult
```

Get the last INET error code

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetErrorDesc`

```
hb_inetErrorDesc( <socket> ) -> cResult
```

Get the last INET error code description

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetFD`

```
hb_inetFD( <socket> [, <lNoSocket> ] ) -> nResult
```

?

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<lNoSocket>` — <lNoSocket>

---

## `hb_inetGetAlias`

```
hb_inetGetAlias( <cName> ) -> aHosts
```

Get an array of aliases of a server

**Parâmetros:**

  - `<cName>` — <cName>

---

## `hb_inetGetHosts`

```
hb_inetGetHosts( <cName> ) -> aHosts
```

Get an array of IP addresses of a host

**Parâmetros:**

  - `<cName>` — <cName>

---

## `hb_inetGetRcvBufSize`

```
hb_inetGetRcvBufSize( <socket> ) -> nResult
```

Get the socket receive buffer size

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetGetSndBufSize`

```
hb_inetGetSndBufSize( <socket> ) -> nResult
```

Get the socket send buffer size

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetInit`

```
hb_inetInit() -> lResult
```

Activate Harbour INET support

---

## `hb_inetIsSocket`

```
hb_inetIsSocket( <socket> ) -> lResult
```

Get whether a variable is a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetPeriodCallback`

```
hb_inetPeriodCallback( <socket> [, <xCallback> ] ) -> xPreviousCallback
```

Get or change the periodic callback value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened xCallback a new periodic callback

---

## `hb_inetPort`

```
hb_inetPort( <socket> ) -> cResult
```

Get the port a socket is bound to.

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetRecv`

```
hb_inetRecv( <socket>, @<cResult>, [ <nAmount> ] ) -> nResult
```

Read from a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cResult>` — <cResult> is the target buffer and must be passed by reference
  - `<nAmount>` — <nAmount> is the upper limit of characters to be read from the socket. If not passed this defaults to the length of cResult

---

## `hb_inetRecvAll`

```
hb_inetRecvAll( <socket>, @<cResult>, [ <nAmount> ] ) -> nResult
```

Read from a socket without blocking

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cResult>` — <cResult> is the target buffer and must be passed by reference
  - `<nAmount>` — <nAmount> is the upper limit of characters to be read from the socket. If not passed this defaults to the length of cResult

---

## `hb_inetRecvEndblock`

```
hb_inetRecvEndblock( <socket> [, <cBlock >[, @<nBytesRead> [, <nMaxLength> [, <nBufSize> ]]]] ) -> cResult
```

Read a block from a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cBlock>` — <cBlock>
  - `<nBytesRead>` — <nBytesRead>
  - `<nMaxLength>` — <nMaxLength>
  - `<nBufSize>` — <nBufSize>

---

## `hb_inetRecvLine`

```
hb_inetRecvLine( <socket> [, @<nBytesRead>, [, <nMaxLength> [, <nBufSize> ]]] ) -> cResult
```

Read a line from a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<nBytesRead>` — <nBytesRead> must be passed by reference
  - `<nMaxLength>` — <nMaxLength>
  - `<nBufSize>` — <nBufSize>

---

## `hb_inetSend`

```
hb_inetSend( <socket>, <cBuffer> [, <nLength> ] ) -> nResult
```

Sent data through a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cBuffer>` — <cBuffer>
  - `<nLength>` — <nLength>

---

## `hb_inetSendAll`

```
hb_inetSendAll( <socket>, <cBuffer> [, <nLength> ] ) -> nResult
```

Send data through a socket with blocking

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<cBuffer>` — <cBuffer>
  - `<nLength>` — <nLength>

---

## `hb_inetServer`

```
hb_inetServer( <port> [, <cBindAddr> [, <nListenLimit> ]]  ) -> SOCKET
```

Create a socket bound to a port

**Parâmetros:**

  - `<port>` — <port>
  - `<cBindAddr>` — <cBindAddr>
  - `<nListenLimit>` — <nListenLimit> is an internal parameter and rarely needs to be passed, defaults to 10

---

## `hb_inetSetRcvBufSize`

```
hb_inetSetRcvBufSize( <socket>, nSize ) -> nSize
```

Set the receive buffer size of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened nSize

---

## `hb_inetSetSndBufSize`

```
hb_inetSetSndBufSize( <socket>, <nSize> ) -> nSize
```

Set the send buffer size of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened nSize

---

## `hb_inetstatus`

```
hb_inetstatus( <socket> ) -> nResult
```

Get the status of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened

---

## `hb_inetTimeLimit`

```
hb_inetTimeLimit( <socket> [, <nTimeLimit> ) -> NIL
```

Get or change the time limit value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<nTimeLimit>` — <nTimeLimit>

---

## `hb_inetTimeout`

```
hb_inetTimeout( <socket> [, <nTimeout> ] ) -> nPreviousTimeout
```

Get or change the timeout value of a socket

**Parâmetros:**

  - `<socket>` — <socket> a socket previously created / opened
  - `<nTimeout>` — <nTimeout> is the new socket timeout value

---
