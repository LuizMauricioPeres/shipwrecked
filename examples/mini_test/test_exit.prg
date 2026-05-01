#define K_ESC 27

function main()
    local nKey := 0
    while .T.
        nKey := inkey(0)
        if LastKey() == K_ESC
            exit
        endif
    enddo
return 0
