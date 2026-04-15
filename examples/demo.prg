/*
 * examples/demo.prg
 * Shipwrecked (SWed) demonstration source
 *
 * Run:  swed examples/demo.prg hbdocs.json
 *       rustc demo.rs --edition 2021   (after linking swed_rt)
 */

PROCEDURE Main()

   LOCAL aNomes  := {}
   LOCAL aScores := {}
   LOCAL i       := 0
   LOCAL nTotal  := 0
   LOCAL cNome   := ""

   * Populate arrays
   AAdd( aNomes,  [Alice]   )
   AAdd( aNomes,  [Bob]     )
   AAdd( aNomes,  [Charlie] )

   AAdd( aScores, 92 )
   AAdd( aScores, 87 )
   AAdd( aScores, 95 )

   * Print header
   ? [=== Score Report ===]
   ? []

   * Loop and print each entry
   FOR i := 1 TO LEN( aNomes ) STEP 1
      cNome  := aNomes[i]
      nTotal := nTotal + aScores[i]
      ? cNome + [: ] + STR( aScores[i] )
   NEXT

   ? []
   ? [Average: ] + STR( nTotal / LEN( aNomes ) )

   * Conditional
   IF nTotal / LEN( aNomes ) >= 90
      ? [Grade: A]
   ELSEIF nTotal / LEN( aNomes ) >= 80
      ? [Grade: B]
   ELSE
      ? [Grade: C]
   ENDIF

RETURN


* ---------------------------------------------------------------------------
FUNCTION MaxScore( aArr )
* ---------------------------------------------------------------------------
   LOCAL nMax := aArr[1]
   LOCAL i    := 0

   FOR i := 2 TO LEN( aArr ) STEP 1
      IF aArr[i] > nMax
         nMax := aArr[i]
      ENDIF
   NEXT

RETURN nMax
