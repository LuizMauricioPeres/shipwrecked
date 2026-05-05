# Template: Implementação de Função Harbour (`swed_bf`)

## Visão geral

Use este template ao pedir para a IA implementar ou revisar uma função Harbour no crate `swed_bf`.
Preencha os campos marcados com `[...]` antes de enviar.

---

## 1. Identificação

| Campo | Valor |
|-------|-------|
| Função Harbour | `[NOME_DA_FUNCAO]` |
| Módulo (`swed_bf/src/`) | `[modulo].rs` |
| Módulo de docs | `[Modulo]_README_IA.md` |
| Harbour signature | `[ex: LEFT( <cString>, <nLen> ) --> cReturn]` |

---

## 2. Contexto obrigatório para a IA

Cole o(s) bloco(s) relevantes do `README_IA.md` do módulo correspondente
**antes** do pedido de implementação. Exemplo para `LEFT`:

```
Left|Left( <cString>, <nLen> ) --> cReturn|Extract the leftmost substring of a character expression|ARGS:<cString>:<cString> Main character to be parsed; <nLen>:<nLen> Number of bytes to return beginning at the leftmost position
```

> **Por quê?** Sem isso a IA infere a semântica por analogia com outras linguagens
> e erra em edge cases (retorno em `nLen=0`, comportamento com `Nil`, etc.).

---

## 3. Assinatura Rust correta

Funções `swed_bf` **sempre** recebem slice de argumentos — Harbour é dinâmico
e `PCount()` deve funcionar em qualquer função:

```rust
// CORRETO
pub fn hb_[nome](args: &[HbValue]) -> HbValue { ... }

// ERRADO — assinatura posicional fixa não funciona com PCount()
pub fn hb_left(val: HbValue, count: HbValue) -> HbValue { ... }
```

Extração dos argumentos dentro da função:

```rust
let s   = args.get(0).cloned().unwrap_or(HbValue::Nil);
let cnt = args.get(1).cloned().unwrap_or(HbValue::Nil);
```

---

## 4. Checklist semântico por tipo de retorno

### Funções de String (`cString → cString`)
- [ ] Se o argumento principal não for `HbValue::String` → retornar `HbValue::Nil`
- [ ] Slicing **sempre** via `.chars().take(n)` ou índice de bytes seguro — nunca indexação direta
- [ ] `nLen < 0` → tratar como `0` (comportamento Harbour, não pânico)
- [ ] `nLen > len(string)` → retornar a string inteira (sem truncar nem pânico)
- [ ] Resultado vazio → `HbValue::String(String::new())`, **não** `HbValue::Nil`

### Funções Numéricas (`nNumber → nResult`)
- [ ] Se o argumento não for `Integer` nem `Float` → retornar `HbValue::Nil`
- [ ] Divisão / raiz de negativo / log(0) → retornar `HbValue::Nil` (sem `panic!`)
- [ ] Arredondamento: usar `hb_round(n, dec)` do `swed_rt` — **nunca** `.round()` nativo do Rust (semântica diferente para .5)
- [ ] Resultado inteiro vs float: preservar o tipo do input quando possível

### Funções de Data (`dDate → xResult`)
- [ ] Verificar `HbValue::Date(_)` antes de operar
- [ ] Aritmética de datas usa dias desde epoch (i32) — **não** converter para `chrono`
- [ ] Retorno de string de data: formato `DD/MM/YYYY`

### Funções de Array (`aArray → xResult`)
- [ ] Índices são **1-based** em Harbour → subtrair 1 ao acessar `HbArray`
- [ ] Índice fora dos limites → `HbValue::Nil` (não pânico)
- [ ] Modificações in-place devem clonar apenas quando necessário (ver regra de clone)

---

## 5. Regras de erro e resultado

```rust
// Erros de runtime retornam Result — use SwedError de swed_co
// Para funções que apenas retornam Nil em caso de tipo errado:
pub fn hb_left(args: &[HbValue]) -> HbValue {
    let HbValue::String(ref s) = args.get(0).unwrap_or(&HbValue::Nil) else {
        return HbValue::Nil;
    };
    // ...
}

// Para funções que podem falhar com contexto (ex: I/O, rede):
pub fn hb_memoread(args: &[HbValue]) -> Result<HbValue, SwedError> {
    // ...
}
```

**Nunca use:**
- `unwrap()` ou `expect()` em código gerado/runtime
- `as f64` / `as i64` direto em `HbValue` — use os operadores implementados em `swed_rt`
- `.clone()` dentro de loops — prefira `&HbValue`

---

## 6. Padrão de código completo (exemplo: `LEFT`)

```rust
// swed_bf/src/string.rs

pub fn hb_left(args: &[HbValue]) -> HbValue {
    let s = match args.get(0) {
        Some(HbValue::String(s)) => s,
        _ => return HbValue::Nil,
    };
    let count = match args.get(1) {
        Some(HbValue::Integer(n)) => (*n).max(0) as usize,
        Some(HbValue::Float(f))   => (*f).max(0.0) as usize,
        _ => return HbValue::Nil,
    };
    HbValue::String(s.chars().take(count).collect())
}
```

---

## 7. Testes obrigatórios

Todo PR de função nova deve incluir ao menos:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use swed_rt::HbValue;

    #[test]
    fn test_[nome]_caso_tipico() {
        // assert comportamento normal
    }

    #[test]
    fn test_[nome]_tipo_errado_retorna_nil() {
        let result = hb_[nome](&[HbValue::Integer(1)]);
        assert_eq!(result, HbValue::Nil);
    }

    #[test]
    fn test_[nome]_arg_ausente_retorna_nil() {
        let result = hb_[nome](&[]);
        assert_eq!(result, HbValue::Nil);
    }

    // Para funções numéricas: incluir caso de divisão/log/raiz inválida
    // Para funções de array: incluir caso de índice fora dos limites
    // Para funções de data: incluir caso com data inválida ou Nil
}
```

---

## 8. Prompt completo para a IA

```
[Cole aqui o bloco README_IA.md do módulo relevante]

Implemente a função Harbour `[ASSINATURA_COMPLETA]` no crate `swed_bf`,
arquivo `swed_bf/src/[modulo].rs`.

Regras:
- Assinatura: `pub fn hb_[nome](args: &[HbValue]) -> HbValue`
- [descreva o comportamento esperado para cada argumento e edge case]
- Retorne `HbValue::Nil` para tipo errado ou argumento ausente
- Não use unwrap(), expect(), as f64 direto, nem .clone() em loop
- Inclua os testes unitários mínimos listados no template

Referência de semântica Harbour: [cole linha do README_IA.md]
```

---

## 9. Módulos e arquivos de referência rápida

| Necessidade | Arquivo |
|-------------|---------|
| Tipos e operadores | `swed_rt/src/value.rs` |
| Arredondamento correto | `hb_round`, `hb_trunc` em `swed_rt/src/value.rs` |
| Erros de runtime | `swed_co/src/error.rs` (`SwedError`, `SeverityLevel`) |
| Builtins existentes | `swed_bf/src/` |
| Docs de semântica Harbour | `[Modulo]_README_IA.md` |
| Conversão de nomes | `to_snake_case` (disponível no projeto) |
