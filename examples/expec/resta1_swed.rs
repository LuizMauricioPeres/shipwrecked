// SWed (Shipwrecked) — versão funcional corrigida de resta1.prg
//
// Correções aplicadas sobre a saída bruta do transpilador:
//   • `while HbValue::Logical(true)` → `loop`
//   • `if (!posicaovalida(...))` → `.is_truthy()` (Not retorna HbValue, não bool)
//   • `posicaovalida` retornava `bool` numa função `-> HbValue` → `HbValue::Logical(...)`
//   • `hb_setcolor()` sem args → `hb_setcolor(HbValue::Nil)`
//   • `desenhatabuleiro(atabuleiro)` move o tabuleiro no loop → `.clone()` no call site
//   • `iniciatabuleiro`: inner loop rodava DEPOIS de mover `alinha` p/ atabuleiro → ordem corrigida
//   • `hb_range(..., 1)` literal i32 em param HbValue → `HbValue::Integer(1)`
//   • `possocomerpeca` mutava cópia local → parâmetro alterado para `&mut HbValue`
#![allow(non_snake_case, unused_mut, unused_variables)]

use swed_rt::{
    HbValue, HbArray,
    hb_array, hb_chr, hb_inkey, hb_lastkey, hb_max, hb_min,
    hb_range, hb_replicate, hb_setcolor, hb_space,
};

// Constantes simbólicas do .prg original (#define)
const PECA:     i64 = 1;
const VAZIO:    i64 = 2;
const INVALIDO: i64 = 3;

fn main() {
    let mut nkey = HbValue::Nil;
    let mut nlinha  = HbValue::Integer(5);
    let mut ncoluna = HbValue::Integer(5);
    let mut alinha: HbValue;
    let mut ccor = HbValue::String("R/W,N/GR*,,,N/W*".into());
    let mut apecaselecionada = HbValue::Nil;
    let mut atabuleiro = iniciatabuleiro();

    print!("\x1B[2J\x1B[H");
    let _ = std::io::Write::flush(&mut std::io::stdout());

    loop {
        desenhatabuleiro(atabuleiro.clone());
        alinha = atabuleiro.hb_get_val(nlinha.clone());
        desenha(alinha.clone(), nlinha.clone(), ncoluna.clone(), ccor.clone());
        let _ = std::io::Write::flush(&mut std::io::stdout());

        nkey = hb_inkey(HbValue::Integer(0));

        if hb_lastkey() == HbValue::Integer(27) {
            break;
        }

        if nkey.clone() == HbValue::Integer(5) {           // K_UP
            nlinha -= HbValue::Integer(1);
            nlinha = hb_max(HbValue::Integer(1), nlinha.clone());
            if !posicaovalida(nlinha.clone(), ncoluna.clone()).is_truthy() {
                nlinha += HbValue::Integer(1);
            }
        } else if nkey.clone() == HbValue::Integer(24) {   // K_DOWN
            nlinha += HbValue::Integer(1);
            nlinha = hb_min(HbValue::Integer(7), nlinha.clone());
            if !posicaovalida(nlinha.clone(), ncoluna.clone()).is_truthy() {
                nlinha -= HbValue::Integer(1);
            }
        } else if nkey.clone() == HbValue::Integer(19) {   // K_LEFT
            ncoluna -= HbValue::Integer(1);
            ncoluna = hb_max(HbValue::Integer(1), ncoluna.clone());
            if !posicaovalida(nlinha.clone(), ncoluna.clone()).is_truthy() {
                ncoluna += HbValue::Integer(1);
            }
        } else if nkey.clone() == HbValue::Integer(4) {    // K_RIGHT
            ncoluna += HbValue::Integer(1);
            ncoluna = hb_min(HbValue::Integer(7), ncoluna.clone());
            if !posicaovalida(nlinha.clone(), ncoluna.clone()).is_truthy() {
                ncoluna -= HbValue::Integer(1);
            }
        } else if nkey.clone() == HbValue::Integer(13) {   // K_RETURN
            if ccor.clone() == HbValue::String("R/W,N/GR*,,,N/W*".into()) {
                ccor = HbValue::String("G/W,N/GR*,,,N/W*".into());
                let mut __a = HbArray::new();
                __a.hb_aadd(nlinha.clone());
                __a.hb_aadd(ncoluna.clone());
                apecaselecionada = HbValue::Array(__a);
            } else {
                possocomerpeca(
                    &mut atabuleiro,
                    nlinha.clone(),
                    ncoluna.clone(),
                    apecaselecionada.clone(),
                );
                ccor = HbValue::String("R/W,N/GR*,,,N/W*".into());
            }
        }
    }
}

// Retorna HbValue::Logical — a saída original retornava `bool` diretamente,
// o que falha em `-> HbValue`.
fn posicaovalida(nlinha: HbValue, ncoluna: HbValue) -> HbValue {
    HbValue::Logical(
        ((nlinha.clone() > HbValue::Integer(2)) && (nlinha < HbValue::Integer(6)))
            || ((ncoluna.clone() > HbValue::Integer(2)) && (ncoluna < HbValue::Integer(6))),
    )
}

fn iniciatabuleiro() -> HbValue {
    let mut atabuleiro = hb_array(HbValue::Integer(7));
    for nlinha in hb_range(HbValue::Integer(1), atabuleiro.hb_len(), HbValue::Integer(1)) {
        // Correção: preenche alinha ANTES de movê-la para atabuleiro.
        // A versão original chamava hb_set_val(nlinha, alinha) primeiro, depois
        // tentava iterar sobre alinha — que já tinha sido movida.
        let mut alinha = hb_array(HbValue::Integer(7));
        for ncoluna in hb_range(HbValue::Integer(1), alinha.hb_len(), HbValue::Integer(1)) {
            if posicaovalida(nlinha.clone(), ncoluna.clone()).is_truthy() {
                alinha.hb_set_val(ncoluna, HbValue::Integer(PECA));
            } else {
                alinha.hb_set_val(ncoluna, HbValue::Integer(INVALIDO));
            }
        }
        atabuleiro.hb_set_val(nlinha, alinha);
    }
    // Centro do tabuleiro começa vazio
    atabuleiro.hb_set_nested(
        HbValue::Integer(4),
        HbValue::Integer(4),
        HbValue::Integer(VAZIO),
    );
    atabuleiro
}

fn desenhatabuleiro(atabuleiro: HbValue) {
    // Correção: hb_setcolor() sem args não existe — passa Nil para indicar getter.
    let ccor = hb_setcolor(HbValue::Nil);
    for nlinha in hb_range(HbValue::Integer(1), atabuleiro.hb_len(), HbValue::Integer(1)) {
        let alinha = atabuleiro.hb_get_val(nlinha.clone());
        for ncoluna in hb_range(HbValue::Integer(1), alinha.hb_len(), HbValue::Integer(1)) {
            desenha(alinha.clone(), nlinha.clone(), ncoluna.clone(), ccor.clone());
        }
    }
}

fn desenha(alinha: HbValue, nlinha: HbValue, ncoluna: HbValue, _ccor: HbValue) {
    let mut adesenha = hb_array(HbValue::Integer(3));
    adesenha.hb_set_val(HbValue::Integer(INVALIDO), hb_space(HbValue::Integer(1)));
    adesenha.hb_set_val(HbValue::Integer(PECA),     hb_chr(HbValue::Integer(219)));
    adesenha.hb_set_val(HbValue::Integer(VAZIO),    hb_chr(HbValue::Integer(176)));

    let cell_type = alinha.hb_get_val(ncoluna.clone());
    let glyph     = adesenha.hb_get_val(cell_type);
    let rendered  = hb_replicate(glyph, HbValue::Integer(3));

    // @ nLinha * 2, nColuna * 6 SAY rendered COLOR cCor
    let row = match &nlinha  { HbValue::Integer(n) => n * 2, _ => 0 };
    let col = match &ncoluna { HbValue::Integer(n) => n * 6, _ => 0 };
    print!("\x1B[{};{}H{}", row, col, rendered);
}

// Correção: assinatura alterada para &mut HbValue para que as mutações
// no tabuleiro sejam visíveis no chamador.  A versão original recebia por
// valor — todas as atribuições hb_set_nested eram perdidas.
fn possocomerpeca(
    atabuleiro: &mut HbValue,
    nlinha: HbValue,
    ncoluna: HbValue,
    apecaselecionada: HbValue,
) {
    let peca  = HbValue::Integer(PECA);
    let vazio = HbValue::Integer(VAZIO);
    let sel_linha  = apecaselecionada.hb_get_val(HbValue::Integer(1));
    let sel_coluna = apecaselecionada.hb_get_val(HbValue::Integer(2));

    // Movimento para a DIREITA: destino = sel_col + 2
    if nlinha.clone() == sel_linha.clone()
        && (ncoluna.clone() - sel_coluna.clone()) == HbValue::Integer(2)
    {
        if atabuleiro.hb_get_val(nlinha.clone()).hb_get_val(ncoluna.clone()) == vazio.clone()
            && atabuleiro.hb_get_val(nlinha.clone())
                .hb_get_val(ncoluna.clone() - HbValue::Integer(1)) == peca.clone()
            && atabuleiro.hb_get_val(nlinha.clone())
                .hb_get_val(ncoluna.clone() - HbValue::Integer(2)) == peca.clone()
        {
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone(),                            peca.clone());
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone() - HbValue::Integer(1), vazio.clone());
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone() - HbValue::Integer(2), vazio.clone());
        }

    // Movimento para a ESQUERDA: destino = sel_col - 2
    } else if nlinha.clone() == sel_linha.clone()
        && (ncoluna.clone() - sel_coluna.clone()) == -HbValue::Integer(2)
    {
        if atabuleiro.hb_get_val(nlinha.clone()).hb_get_val(ncoluna.clone()) == vazio.clone()
            && atabuleiro.hb_get_val(nlinha.clone())
                .hb_get_val(ncoluna.clone() + HbValue::Integer(1)) == peca.clone()
            && atabuleiro.hb_get_val(nlinha.clone())
                .hb_get_val(ncoluna.clone() + HbValue::Integer(2)) == peca.clone()
        {
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone(),                            peca.clone());
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone() + HbValue::Integer(1), vazio.clone());
            atabuleiro.hb_set_nested(nlinha.clone(), ncoluna.clone() + HbValue::Integer(2), vazio.clone());
        }

    // Movimento para BAIXO: destino = sel_linha + 2
    } else if ncoluna.clone() == sel_coluna.clone()
        && (nlinha.clone() - sel_linha.clone()) == HbValue::Integer(2)
    {
        if atabuleiro.hb_get_val(nlinha.clone()).hb_get_val(ncoluna.clone()) == vazio.clone()
            && atabuleiro.hb_get_val(nlinha.clone() - HbValue::Integer(1))
                .hb_get_val(ncoluna.clone()) == peca.clone()
            && atabuleiro.hb_get_val(nlinha.clone() - HbValue::Integer(2))
                .hb_get_val(ncoluna.clone()) == peca.clone()
        {
            atabuleiro.hb_set_nested(nlinha.clone(),                            ncoluna.clone(), peca.clone());
            atabuleiro.hb_set_nested(nlinha.clone() - HbValue::Integer(1), ncoluna.clone(), vazio.clone());
            atabuleiro.hb_set_nested(nlinha.clone() - HbValue::Integer(2), ncoluna.clone(), vazio.clone());
        }

    // Movimento para CIMA: destino = sel_linha - 2
    } else if ncoluna.clone() == sel_coluna.clone()
        && (nlinha.clone() - sel_linha.clone()) == -HbValue::Integer(2)
    {
        if atabuleiro.hb_get_val(nlinha.clone()).hb_get_val(ncoluna.clone()) == vazio.clone()
            && atabuleiro.hb_get_val(nlinha.clone() + HbValue::Integer(1))
                .hb_get_val(ncoluna.clone()) == peca.clone()
            && atabuleiro.hb_get_val(nlinha.clone() + HbValue::Integer(2))
                .hb_get_val(ncoluna.clone()) == peca.clone()
        {
            atabuleiro.hb_set_nested(nlinha.clone(),                            ncoluna.clone(), peca.clone());
            atabuleiro.hb_set_nested(nlinha.clone() + HbValue::Integer(1), ncoluna.clone(), vazio.clone());
            atabuleiro.hb_set_nested(nlinha.clone() + HbValue::Integer(2), ncoluna.clone(), vazio.clone());
        }
    }
}
