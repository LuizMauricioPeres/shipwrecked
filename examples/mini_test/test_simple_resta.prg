#define PECA     1
#define VAZIO    2
#define INVALIDO 3

#define K_UP                    5
#define K_DOWN                  24
#define K_LEFT                  19
#define K_RIGHT                 4
#define K_HOME                  1
#define K_END                   6
#define K_PGUP                  18
#define K_PGDN                  3
#define K_ENTER                 13
#define K_INTRO                 13
#define K_RETURN                13
#define K_SPACE                 32
#define K_ESC                   27


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
