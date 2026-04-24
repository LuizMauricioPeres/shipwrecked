/*
 * examples/demo.prg
 * Shipwrecked (SWed) demonstration source
 *
 * Run:  swed examples/demo.prg hbdocs.json
 *       rustc demo.rs --edition 2021   (after linking swed_rt)
 */

PROCEDURE Main( nValor, nLocal, nQuantidade, /* Teste de variavel errada porém comentada */ ERRADO  )

   LOCAL aNomes  := {}
   LOCAL aScores := {}
   LOCAL i       := 0
   LOCAL nTotal  := 0, aMax := { 1, 10, 15}
   LOCAL cNome   := "", GetList := {}

   * Populate arrays
   AAdd( aNomes,  [Alice]   )
   AAdd( aNomes,  [Bob]     )
   AAdd( aNomes,  [Charlie] )
   AAdd( aNomes,  "Blink" )

   AAdd( aScores, 92 )
   AAdd( aScores, 87 )
   AAdd( aScores, 95 )

   if nValor == 0
      nLocal := 5
      //nQuantidade+=1
      //nQuantidade++
      ERRADO = [TEXT]
   endif

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

//   @ 15, 30 say nTotal Picture "@E 9999.99" 
//   @ 15, 30 get nTotal Picture "@E 9999.99" valid MaxScore( @aMax ) > nTotal

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

PROCEDURE Proc( nValor, nLocal, nQuantidade, /* Teste de variavel errada porém comentada */ ERRADO  )

   LOCAL aNomes  := {}
   LOCAL aScores := {}
   LOCAL i       := 0
   LOCAL nTotal  := 0, aMax := { 1, 10, 15}
   LOCAL cNome   := "", GetList := {}

   * Populate arrays
   AAdd( aNomes,  [Alice]   )
   AAdd( aNomes,  [Bob]     )
   AAdd( aNomes,  [Charlie] )
   AAdd( aNomes,  "Blink" )

   AAdd( aScores, 92 )
   AAdd( aScores, 87 )
   AAdd( aScores, 95 )

   if nValor == 0
      nLocal := 5
      //nQuantidade+=1
      //nQuantidade++
      ERRADO = [TEXT]
   endif

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

//   @ 15, 30 say nTotal Picture "@E 9999.99" 
//   @ 15, 30 get nTotal Picture "@E 9999.99" valid MaxScore( @aMax ) > nTotal

RETURN
