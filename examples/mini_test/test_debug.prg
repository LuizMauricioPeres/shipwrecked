


















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
               , cCor