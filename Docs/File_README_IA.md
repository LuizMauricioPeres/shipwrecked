# File — Harbour functions for SWed transpiler
# Low-level file I/O, directories, memo read/write, disk checks.
# count:25

__Dir|__Dir( [<cFileMask>] ) --> NIL|Display listings of files|ARGS:<cFileMask>:<cFileMask> File mask to include in the function return. It could contain path and standard wildcard characters as suppo
CurDir|CurDir( [<cDrive>] ) --> cPath|Returns the current OS directory name.|ARGS:<cDrive>:<cDrive> OS drive letter
DirChange|DirChange( <cDirectory> ) --> nError|Changes the directory|ARGS:<cDirectory>:<cDirectory>  The name of the directory you want do change into.
DirRemove|DirRemove( <cDirectory> ) --> nError|Attempt to remove an directory|ARGS:<cDirectory>:<cDirectory>  The name of the directory you want to remove.
DiskSpace|DiskSpace( [<nDrive>] ) --> nDiskbytes|Get the amount of space available on a disk|ARGS:<nDrive>:<nDrive> The number of the drive you are requesting info on where 1 = A, 2 = B, etc. For 0 or no parameter, DiskSpace wi
FClose|FClose( <nHandle> ) --> <lSuccess>|Closes an open file|ARGS:<nHandle>:<nHandle> File handle
FCreate|FCreate( <cFile>, [<nAttribute>] ) --> nHandle|Creates a file.|ARGS:<cFile>:<cFile> is the name of the file to create.; <nAttribute>:<nAttribute> Numeric code for the file attributes.
FErase|FErase( <cFile> ) --> nSuccess|Erase a file from disk|ARGS:<cFile>:<cFile> Name of file to erase.
FError|FError() --> <nErrorCode>|Reports the error status of low-level file functions
File|File( <cFileSpec> ) --> lExists|Tests for the existence of File(s)|ARGS:<cFileSpec>:<cFileSpec> Filename skeleton or file name to find.
FOpen|FOpen( <cFile>, [<nMode>] ) --> nHandle|Open a file.|ARGS:<cFile>:<cFile> Name of file to open.; <nMode>:<nMode> File open mode.
FRead|FRead( <nHandle>, @<cBuffer>, <nBytes> ) --> nBytes|Reads a specified number of bytes from a file.|ARGS:<nHandle>:<nHandle>     File handle; <cBuffer>:<cBuffer>  Character expression passed by reference.; <nBytes>:<nBytes>      Number of bytes to read.
FReadStr|FReadStr( <nHandle>, <nBytes> ) --> cString|Reads a string from a file.|ARGS:<nHandle>:<nHandle> File handle number.; <nBytes>:<nBytes>  Number of bytes to read.
FRename|FRename( <cOldFile>, <cNewFile> ) --> nSuccess|Renames a file|ARGS:<cOldFile>:<cOldFile> Old filename to be changed; <cNewFile>:<cNewFile> New filename
FSeek|FSeek( <nHandle>, <nOffset>, [<nOrigin>] ) --> nPosition|Positions the file pointer in a file.|ARGS:<nHandle>:<nHandle> File handle.; <nOffset>:<nOffset> The number of bytes to move.; <nOrigin>:<nOrigin> The relative position in the file.
FWrite|FWrite( <nHandle>, <cBuffer>, [<nBytes>] ) --> nBytesWritten|Writes characters to a file.|ARGS:<nHandle>:<nHandle>  File handle number.; <cBuffer>:<cBuffer>  Character expression to be written.; <nBytes>:<nBytes>   The number of bytes to write.
hb_DiskSpace|hb_DiskSpace( [<cDrive>] [, <nType>] ) --> nDiskbytes|Get the amount of space available on a disk|ARGS:<cDrive>:<cDrive> The drive letter you are requesting info on. The default is A:; <nType>:<nType> The type of space being requested. The default is HB_DISK_AVAIL.
hb_FEof|hb_FEof( <nHandle> ) --> lIsEof|Check for end-of-file.|ARGS:<nHandle>:<nHandle> The handle of an open file.
hb_MemoRead|hb_MemoRead( <cFileName> ) --> cString|Return the text file's contents as a character string|ARGS:<cFileName>:<cFileName> is the filename to read from disk. It must include the file extension. If file to be read lives in another d
hb_MemoWrit|hb_MemoWrit( <cFileName>, <cString> ) --> lSuccess|Write a memo field or character string to a text file on disk|ARGS:<cFileName>:<cFileName> is the filename to be written to disk. It must include the file extension. If file to be read lives in anoth; <cString>:<cString>   Is the memo field or character string, to be write to; <cFile>:<cFile>.
IsDisk|IsDisk( <cDrive> ) --> lSuccess|Verify if a drive is ready|ARGS:<cDrive>:<cDrive>  An valid Drive letter
MakeDir|MakeDir( <cDirectory> ) --> nError|Create a new directory|ARGS:<cDirectory>:<cDirectory>  The name of the directory you want to create.
MemoRead|MemoRead( <cFileName> ) --> cString|Return the text file's contents as a character string|ARGS:<cFileName>:<cFileName> is the filename to read from disk. It must include the file extension. If file to be read lives in another d
MemoTran|MemoTran( <cString>, <cHard>, <cSoft> ) --> <cConvertedString>|Converts hard and soft carriage returns within strings.|ARGS:<cString>:<cString> is a string of chars to convert.; <cHard>:<cHard> is the character to replace hard returns with. If not specified defaults to semicolon.; <cSoft>:<cSoft> is the character to replace soft returns with. If not specified defaults to single space.
MemoWrit|MemoWrit( <cFileName>, <cString> ) --> lSuccess|Write a memo field or character string to a text file on disk|ARGS:<cFileName>:<cFileName> is the filename to be written to disk. It must include the file extension. If file to be read lives in anoth; <cString>:<cString>   Is the memo field or character string, to be write to; <cFile>:<cFile>.