# Runtime — Harbour functions for SWed transpiler
# GC, idle loop and math error handling in the Harbour runtime.
# count:6

hb_gcAll|hb_gcAll()|Scans the memory and releases all garbage memory blocks.
hb_idleAdd|hb_idleAdd( <bAction> ) --> nHandle|Adds the background task.|ARGS:<bAction>:<bAction> is a codeblock that will be executed during idle states. There are no arguments passed to this codeblock durin
hb_idleDel|hb_idleDel( <nHandle> ) --> <bAction>|Removes the background task from the list of tasks.|ARGS:<nHandle>:<nHandle> is the identifier of the task returned by the hb_idleAdd() function.
hb_idleState|hb_idleState()|Evaluates a single background task and calls the garbage collector.
hb_matherBlock|hb_matherBlock( [<bNewBlock>] ) --> <bOldBlock>|Set/Get math error handling codeblock|ARGS:<bNewBlock>:<bNewBlock>
hb_matherMode|hb_matherMode( [<nNewMode>] ) --> <nOldMode>|Set/Get math error handling mode|ARGS:<nNumber>:[<nNumber>]   new math error handling mode, one of the following constants, defined in hbmath.ch: HB_MATH_ERRMODE_DEFAUL