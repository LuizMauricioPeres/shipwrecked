#define PECA     1
#define VAZIO    2
#define INVALIDO 3

#define K_UP                    5     /*   Up arrow, Ctrl-E              */
#define K_DOWN                  24    /*   Down arrow, Ctrl-X            */
#define K_LEFT                  19    /*   Left arrow, Ctrl-S            */
#define K_RIGHT                 4     /*   Right arrow, Ctrl-D           */
#define K_HOME                  1     /*   Home, Ctrl-A                  */
#define K_END                   6     /*   End, Ctrl-F                   */
#define K_PGUP                  18    /*   PgUp, Ctrl-R                  */
#define K_PGDN                  3     /*   PgDn, Ctrl-C                  */
#define K_ENTER                 13    /*   Enter, Ctrl-M                 */
#define K_INTRO                 13    /*                                 */
#define K_RETURN                13    /*   Return, Ctrl-M                */
#define K_SPACE                 32    /*   Space bar                     */
#define K_ESC                   27    /*   Esc, Ctrl-[                   */


function main()
    local nKey, nLinha, nColuna, aLinha, cCor, aPecaSelecionada
    local aTabuleiro := IniciaTabuleiro()
    STORE 5 TO nLinha, nColuna
    CLS
    cCor := "R/W,N/GR*,,,N/W*"
    while .T.
        DesenhaTabuleiro( aTabuleiro )
        aLinha := aTabuleiro[ nLinha ]
        Desenha( aLinha;
               , nLinha;
               , nColuna;
               , cCor ;
               )
        nKey := inkey(0)
        if LastKey() == K_ESC
            exit
        endif

        if nKey == K_UP
            nLinha := Max( 1, --nLinha )
            if !PosicaoValida( nLinha, nColuna )
                nLinha++
            endif
        elseif nKey == K_DOWN
            nLinha := Min( 9, ++nLinha )
            if !PosicaoValida( nLinha, nColuna )
                nLinha--
            endif
        elseif nKey == K_LEFT
            nColuna := Max( 1, --nColuna )
            if !PosicaoValida( nLinha, nColuna )
                nColuna++
            endif
        elseif nKey == K_RIGHT
            nColuna := Min( 9, ++nColuna )
            if !PosicaoValida( nLinha, nColuna )
                nColuna--
            endif
        elseif nKey == K_RETURN
            if cCor == "R/W,N/GR*,,,N/W*"
                cCor := "G/W,N/GR*,,,N/W*"
                aPecaSelecionada := { nLinha, nColuna }
            else
                PossoComerPeca( aTabuleiro, nLinha, nColuna, aPecaSelecionada )
                cCor :="R/W,N/GR*,,,N/W*"
            endif
        endif

    enddo
return 0

***************************************************************************************************
static function PosicaoValida( nLinha, nColuna )
return ( nLinha > 2 .and. nLinha < 6 ) .or.;
       ( nColuna > 2 .and. nColuna < 6 )

***************************************************************************************************
static function IniciaTabuleiro()
    local aTabuleiro, aLinha, nLinha, nColuna
    aTabuleiro := Array( 7 )
    for nLinha := 1 To Len( aTabuleiro )
        aLinha := Array( 7 )
        aTabuleiro[ nLinha ] := aLinha
        for nColuna := 1 To Len( aLinha )
            if PosicaoValida( nLinha, nColuna )
                aLinha[  nColuna  ] := PECA
            else
                aLinha[ nColuna   ] := INVALIDO
            endif
       next
   next
   aTabuleiro[ 4, 4 ] := VAZIO
return aTabuleiro

***************************************************************************************************
static procedure DesenhaTabuleiro( aTabuleiro )
    local aLinha, nLinha, nColuna, cCor
    cCor := SetColor()
    for nLinha := 1 To Len( aTabuleiro )
        aLinha := aTabuleiro[ nLinha ]
        for nColuna := 1 To Len( aLinha )
            Desenha( aLinha;
                   , nLinha;
                   , nColuna;
                   , cCor;
                   )
        next
    next
return

***************************************************************************************************
static procedure Desenha( aLinha;
                        , nLinha;
                        , nColuna;
                        , cCor;
                        )
    local aDesenha
    aDesenha := Array( 3 )
    aDesenha[ INVALIDO ] := Space( 1 )
    aDesenha[ PECA     ] := Chr( 219 )
    aDesenha[ VAZIO    ] := Chr( 176 )
    @ nLinha * 2, nColuna * 6 say Replicate( aDesenha[ aLinha[ nColuna ] ], 3 ) COLOR cCor
return

static procedure possoComerPeca( aTabuleiro, nLinha, nColuna, aPecaSelecionada )
    if nLinha == aPecaSelecionada[ 1 ] .and. ( nColuna - aPecaSelecionada[ 2 ] ) == 2
        if aTabuleiro[ nLinha, nColuna - 0 ] == VAZIO .and.;
           aTabuleiro[ nLinha, nColuna - 1 ] == PECA .and.;
           aTabuleiro[ nLinha, nColuna - 2 ] == PECA
           aTabuleiro[ nLinha, nColuna - 0 ] := PECA
           aTabuleiro[ nLinha, nColuna - 1 ] := VAZIO
           aTabuleiro[ nLinha, nColuna - 2 ] := VAZIO
        endif
    elseif nLinha == aPecaSelecionada[ 1 ] .and. ( nColuna - aPecaSelecionada[ 2 ] ) == -2
        if  aTabuleiro[ nLinha, nColuna + 0 ] == VAZIO .and.;
            aTabuleiro[ nLinha, nColuna + 1 ] == PECA .and.;
            aTabuleiro[ nLinha, nColuna + 2 ] == PECA
            aTabuleiro[ nLinha, nColuna + 0 ] := PECA
            aTabuleiro[ nLinha, nColuna + 1 ] := VAZIO
            aTabuleiro[ nLinha, nColuna + 2 ] := VAZIO
        endif
    elseif nColuna == aPecaSelecionada[ 2 ]  .and. ( nLinha - aPecaSelecionada[ 1 ] ) == 2
        if aTabuleiro[ nLinha - 0, nColuna ] == VAZIO .and.;
           aTabuleiro[ nLinha - 1, nColuna ] == PECA .and.;
           aTabuleiro[ nLinha - 2, nColuna ] == PECA
           aTabuleiro[ nLinha - 0, nColuna ] := PECA
           aTabuleiro[ nLinha - 1, nColuna ] := VAZIO
           aTabuleiro[ nLinha - 2, nColuna ] := VAZIO
        endif
    elseif nColuna == aPecaSelecionada[ 2 ]  .and. ( nLinha - aPecaSelecionada[ 1 ] ) == -2
        if aTabuleiro[ nLinha + 0, nColuna ] == VAZIO .and.;
           aTabuleiro[ nLinha + 1, nColuna ] == PECA .and.;
           aTabuleiro[ nLinha + 2, nColuna ] == PECA
           aTabuleiro[ nLinha + 0, nColuna ] := PECA
           aTabuleiro[ nLinha + 1, nColuna ] := VAZIO
           aTabuleiro[ nLinha + 2, nColuna ] := VAZIO
        endif
    endif
return