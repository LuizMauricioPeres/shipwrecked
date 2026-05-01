#define K_ESC 27

function main()
    local nKey, nLinha, nColuna, aLinha, cCor, aPecaSelecionada
    local aTabuleiro := IniciaTabuleiro()
    STORE 5 TO nLinha, nColuna
    CLS
    cCor := "R/W,N/GR*,,,N/W*"
    while .T.
        nKey := inkey(0)
        if LastKey() == K_ESC
            exit
        endif
    enddo
return 0
