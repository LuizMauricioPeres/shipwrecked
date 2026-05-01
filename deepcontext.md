# SWed — Deep Context

> Documento de contexto consolidado para compartilhamento de lógica entre sessões e agentes.
> Gerado em: 2026-04-30 | Atualizado: 2026-04-30 (rev 3 — fixes lexer/parser + compound assignment)

---

## 1. O que é o SWed

SWed é um **transpilador Harbour/xBase → Rust (edição 2021)**.

**Motivação:** Preservar décadas de software de negócio legado (Harbour/Clipper) com segurança de memória Rust e ferramental moderno (LSP, testes automáticos de regressão).

**Pipeline atual:** `Lexer → Parser → Semantic → [swed_nm] → Codegen`

**Workspace Cargo:**
```
swed          (binary — compilador/transpilador)
swed_co       (library — tipos e Traits compartilhados, DNA do sistema)
swed_rt       (library — runtime HbValue, aritmética, datas, builtins)
swed_bf       (library — Basic Functions: Date, Left, AllTrim, etc.)
swed_mkh      (library + binary — manifesto .mkh, analisador, testgen)
swed_nm       (library — Semantic Normalizer: reescrita de AST antes do codegen)
swed_kn       (library — Knife Tools: debug, dump hexadecimal, patch suggestions)
swed_db       (library — RDD, DBF, WorkArea, Row, field_get/set)
swed_ui       (library — TUI/Ratatui, AppState, GetElement, widgets)
swed_io       (library — File System, encoding Win-1252)
```

---

## 2. Arquitetura de Módulos

### Grafo de dependências (runtime path)

```
usuário final:
  swed_rt  ←  swed_co   (traits + tipos)
  swed_bf  ←  swed_co + swed_rt
  swed_db  ←  swed_co + swed_rt
  swed_ui  ←  swed_co + swed_rt
  (opcional) swed_kn  ←  swed_co + swed_rt
  (dev only) swed_nm  ←  swed_co + swed_rt   (compilação apenas)
  (dev only) swed     ←  tudo
```

**Regra de ouro:** Runtime puro (`swed_rt` + `swed_bf`) é deployável sem debug tools ou compilador. `swed_kn`, `swed_nm` e `swed` são ferramentas de desenvolvimento — não devem ser dependências transitivas do produto final.

### Contratos entre crates

| Crate | Papel |
|---|---|
| `swed_co` | Apenas tipos e Traits `pub`. Sem lógica. Todas as outras crates dependem dela. |
| `swed_rt` | Implementa as Traits de `swed_co`. `HbValue`, aritmética, datas, `publics_var`. |
| `swed_bf` | Funções padrão Harbour. Trait `NativeFunction` definida em `swed_co`, implementada aqui. |
| `swed_db` | RDD (Replaceable Database Drivers), isolação de DBF. Owns `DbfHandler`, `WorkArea`, `Row`, `field_get`, `field_set`. |
| `swed_ui` | TUI/Ratatui. `AppState`, `GetElement` trait, widgets — renderização isolada da lógica. |
| `swed_nm` | Semantic Normalizer. Reescreve AST antes do codegen. Emite diagnósticos via `ErrorInterceptor`. Dev only. |
| `swed_kn` | Implementa `ErrorInterceptor`. Dump hexadecimal, sugestões de patch em erros `Critical`. Dev only. |
| `swed_mkh` | Manifesto `.mkh`, análise de símbolos, geração de testes (`testgen`). |
| `swed_io` | File System, encoding Windows-1252. |

### Fases de modularização

**Fase 1 — Fundação**
- `swed_co`: tipos `HbType` (notação húngara), `SeverityLevel`, `SwedError`
- `swed_rt`: `HbValue`, todas as ops retornam `Result<HbValue, SwedError>`
- `swed_bf`: funções padrão via `NativeFunction`

**Fase 2 — Injeção de comportamentos**
- Trait `ModuleComponent` (`on_init`, `on_shutdown`) — módulos como `swed_kn` injetam no ciclo de vida sem alterar `swed_rt`
- Trait `FunctionResolver` — código transpilado recebe Provider de funções (Injeção de Dependência)

**Fase 3 — Interceptação de erros**
- Trait `ErrorInterceptor` — padrão Observer para erros `Critical`
- `swed_kn`: dump hex + sugestão de patch ("vedação")

**Fase 4 — Normalização semântica**
- `swed_nm`: passe de reescrita de AST entre Semantic e Codegen; desambigua operadores, reescreve crimes de `++`/`--`, emite diagnósticos via `swed_kn`

**Fase 5 — Automação de build**
- `swed_mkm` (Master Maker): lê `.mkh`, monta grafo de dependências, injeta diretivas de compilação

---

## 3. Targets Confirmados

| Target | Detalhe |
|---|---|
| CLI/TUI | Ratatui com trait `GetElement` para sistema de READ interativo — **implementado** |
| VS Code Extension | SWed como pré-compilador LSP — Go-to-Definition e inferência de tipos via `.mkh` |

---

## 4. Estado Atual de Implementação

| Feature | Status |
|---|---|
| Lexer / Parser / Semantic / Codegen | ✅ ~95% |
| `HbValue`, `HbArray`, builtins | ✅ |
| `pub_declare/pub_get/pub_set/memvar_assign/memvar_get` em `swed_rt` | ✅ `swed_rt/src/publics_var.rs` |
| Parser: `m->varname` → `ExprKind::Macro`, `ALIAS->field` → `ExprKind::Field` | ✅ `swed/src/parser.rs` |
| Codegen: PUBLIC/MEMVAR via funções livres; FIELD → `field_get/field_set` | ✅ `swed/src/codegen.rs` |
| Codegen: `m->` emite `memvar_get()` | ✅ |
| `swed_rt` runtime puro — sem dependência de `ratatui` | ✅ |
| `.mkh` manifesto (`swed_mkh/` — analyser + emitter + types) | ✅ |
| `cache_maker/` dir (criado automaticamente por `write_mkh()`) | ✅ |
| Encoding CP1252 (`fs::read()` + `encoding_rs::WINDOWS_1252`) | ✅ |
| Test generation (`swed_mkh/src/testgen.rs` + binary `swed_testgen`) | ✅ 36 testes passando |
| `Index<&HbValue>` + `hb_len_as_i64()` em `HbValue` | ✅ (`swed_rt/src/value.rs:165`) |
| `hb_len() → HbValue` + `hb_get_val()` em `HbValue` | ✅ (`value.rs:245-258`) |
| DBF / WorkArea / Row — migrado para `swed_db` | ✅ (`swed_db/src/dbf/`) |
| `field_get` / `field_set` / `field_get_alias` / `field_set_alias` | ✅ (`swed_db/src/lib.rs:32-47`) |
| `swed_ui`: AppState system (`AppState`, `traits::GetElement`, widgets) | ✅ (`swed_ui/src/`) |
| `swed_ui`: GetList system (`GetWidget`, `GetList`, `hb_read`, `GETLIST_ACTIVE`) | ✅ (`swed_ui/src/{get_element,get_list,read}.rs`) |
| Codegen `@..SAY` / `@..GET` / `READ` — blocos TUI | ✅ (`swed/src/codegen.rs`) |
| `%` Mod e `^` Pow — lexer `Percent`/`Caret`, parser, codegen, `impl Rem` + `pow_hb()` | ✅ confirmado |
| **FIX: `*` mid-line tratado como comentário de linha** | ✅ `lexer.rs` — strip em `extract_defines()`, regex removido |
| **FIX: nested array indexing `a[b[i]]`** | ✅ `parser.rs` — `balance_bracket_content()` |
| **Compound assignment `+=` `-=` `*=` `/=`** | ✅ desugar no parser → `Assign(x, BinOp(op, x, rhs))` |
| `AddAssign`/`SubAssign`/`MulAssign`/`DivAssign` em `HbValue` | ❌ Pendente — necessário para `++/--` no codegen (B5) |
| `hb_eq()` / `hb_exact_eq()` em `HbValue` | ❌ Pendente (ver seção 8.1) |
| `swed_nm` crate (Semantic Normalizer) | ❌ Não implementado (ver seção 9) |
| Dual-File Codegen (`<nome>.rs` + `<nome>_module.rs`) | ❌ De-priorizado — `generate_with_module` simplificado |

---

## 5. Manifesto `.mkh` — Analisador de Símbolos

**Referência:** `/home/barruga/src/MakerHeaderGenerate` (repo do Gerardo)

**Função:** Varre diretórios `.prg`, classifica símbolos e gera cache em `./cache_maker/<stem>.mkh`.

**Tipos-chave:**
```
Manifest { source_path, md5, symbols: Vec<Symbol>, usages: Vec<Usage> }
Symbol   { name, kind: SymbolKind, scope, line, attributes, conditional }
SymbolKind: Function | Procedure | Method | Public | Static | Memvar
            | ClassVar { visibility } | Access | Assign | Class
Visibility: Exported | Hidden | Protected
```

**Parser two-pass:**
1. `pass_definitions()` — captura símbolos declarados
2. `pass_usages()` — detecta chamadas por `ident(`

**Emitter:**
- `write_mkh()` → `cache_maker/`
- `render_stdout()` → verbose

**Formato de linha:**
```
[SYMBOL] -> [TIPO] -> Nome | Escopo | Linha | Atributos
[+] NOME | { [Linha:X, Coluna:Y], ... }    ← funções de usuário
```

**Notação húngara:** Antes de converter token para UPPERCASE, detectar `char[0]` minúsculo + `char[1]` maiúsculo → extrair tipo e guardar em `SymbolDef`.

---

## 6. Geração de Testes Automatizados

**Implementação:** `swed_mkh/src/testgen.rs`, função pública `generate_tests(prg_path, manifest, out_dir)`

**Binary:** `swed_mkh/src/bin/swed_testgen.rs` — `swed_testgen <arquivo.prg> [--out-dir <dir>]`

### Estrutura de um teste gerado

```rust
#[cfg(test)]
mod tests {
    use crate::publics_var;
    use crate::swed_rt::HbValue;
    use super::*;

    #[test]
    fn test_calc_imposto_regressao() {
        // 1. Setup de globais identificadas no .mkh
        //    Sempre em bloco {} para soltar o lock antes da execução
        {
            let mut globals = publics_var::globals().write().unwrap();
            globals.n_empresa = HbValue::Integer(1);
            globals.c_usuario = HbValue::String("TEST_RUNNER".into());
        }

        // 2. Execução da função transpilada
        let resultado = calc_imposto(HbValue::Float(100.0), HbValue::Float(15.0));

        // 3. Assert
        assert_eq!(resultado, HbValue::Float(115.0));
    }

    #[test]
    fn test_nil_invalido() {
        // Semântica Harbour: NIL + N → NIL (não panic)
        let resultado = calc_imposto(HbValue::Nil, HbValue::Integer(10));
        assert_eq!(resultado, HbValue::Nil);
    }
}
```

### Heurísticas de inferência de tipos

| Prefixo húngaro | Valor default gerado |
|---|---|
| `N`, `F` | `HbValue::Float(0.0)` |
| `C` | `HbValue::String("".into())` |
| `L` | `HbValue::Logical(false)` |
| Qualquer outro | `HbValue::Nil` |

**Flag `[+]` no `.mkh`** = função de usuário (não nativa Harbour) — essas são as candidatas a teste.

---

## 7. `HbValue` — API de Acesso a Arrays (implementado)

`swed_rt/src/value.rs`

### `Index<&HbValue>` — linha 165

```rust
impl std::ops::Index<&HbValue> for HbValue {
    type Output = HbValue;
    fn index(&self, index: &HbValue) -> &Self::Output { ... }
}
```

Harbour é base-1, Rust base-0 — ajuste interno. Fora de bounds → `&HbValue::Nil`.

### Outros helpers

| Método | Linha | Descrição |
|---|---|---|
| `hb_len_as_i64()` | 235 | Comprimento como `i64` — arrays e strings |
| `hb_len()` | 245 | Comprimento como `HbValue::Integer` |
| `hb_get_val(index)` | 251 | Acesso indexado 1-based (recebe `HbValue` por valor) |

### Padrão de codegen — sem clone interno

```rust
fn maxscore(aarr: &HbValue) -> HbValue {
    let mut nmax = &aarr[&HbValue::Integer(1)];
    for i in 2..=(aarr.hb_len_as_i64()) {
        let idx = HbValue::Integer(i);
        let cur = &aarr[&idx];
        if cur > nmax { nmax = cur; }
    }
    nmax.clone() // clone apenas no retorno final
}
```

### Regras para o codegen

| Situação | Emitir |
|---|---|
| Acesso a elemento de array | `arr[&idx]` |
| Loop sobre array Harbour | `2..=(arr.hb_len_as_i64())` |
| Clone dentro do loop | **Proibido** |
| Clone no `return` final | Permitido |

---

## 8. Operadores Harbour — Semântica e Mapeamento Rust

### 8.1 Operadores de comparação

| Harbour | Semântica | Método em `HbValue` |
|---|---|---|
| `=` (em expressão) | Fuzzy: numérico coerce `Integer↔Float`; string afetada por `SET EXACT OFF` (left-anchored) | `hb_eq(&self, other: &HbValue) -> bool` ❌ pendente |
| `==` | Exato: string full-match; numérico igual ao fuzzy | `hb_exact_eq(&self, other: &HbValue) -> bool` ❌ pendente |
| `<>` / `!=` / `#` | Negação de `hb_eq` | `!self.hb_eq(other)` |
| `!(val==x)` | Negação de `hb_exact_eq` | `!self.hb_exact_eq(other)` |

**Atenção:** `=` em início de statement com LHS `Ident` é **atribuição** — o parser resolve pelo contexto, não o codegen.

**Gap atual:** `PartialEq` derivado em `HbValue` faz `Integer(10) != Float(10.0)`. Em Harbour `10 = 10.0` é `.T.`. O `PartialEq` derivado é correto para `==` Rust interno, mas **não** para codegen de `=`/`==` Harbour.

```rust
// Pendente em swed_rt/src/value.rs
impl HbValue {
    pub fn hb_eq(&self, other: &HbValue) -> bool {
        match (self, other) {
            (HbValue::Integer(a), HbValue::Integer(b)) => a == b,
            (HbValue::Float(a),   HbValue::Float(b))   => a == b,
            (HbValue::Integer(a), HbValue::Float(b))   => (*a as f64) == *b,
            (HbValue::Float(a),   HbValue::Integer(b)) => *a == (*b as f64),
            (HbValue::String(a),  HbValue::String(b))  => a.starts_with(b.as_str()), // SET EXACT OFF
            (HbValue::Logical(a), HbValue::Logical(b)) => a == b,
            (HbValue::Nil,        HbValue::Nil)         => true,
            _ => false,
        }
    }

    pub fn hb_exact_eq(&self, other: &HbValue) -> bool {
        match (self, other) {
            (HbValue::String(a), HbValue::String(b)) => a == b, // full-match
            _ => self.hb_eq(other),
        }
    }
}
```

### 8.2 Operadores de mutação

**Compound assignment `+=` `-=` `*=` `/=`** — implementado via desugar no parser (2026-04-30).

`x += e` é transformado em `StmtKind::Assign(target=x, value=BinOp(Add, x, e))` durante o parse. O codegen não precisa saber da diferença — emite `x = (x.clone() + e);`. Usa `Add` (implementado), não `AddAssign` (pendente). Isso é intencional.

Harbour **não tem** `%=` nem `^=` — não implementados.

**`AddAssign`/`SubAssign` em `HbValue`** — ainda necessários exclusivamente para `++/--` no codegen (ver B5). Quando implementados, usar `mem::replace` para evitar clone:

```rust
// Pendente em swed_rt/src/value.rs — necessário para ++/--
macro_rules! impl_assign_op {
    ($trait:ident, $method:ident, $op:tt) => {
        impl std::ops::$trait for HbValue {
            fn $method(&mut self, rhs: HbValue) {
                *self = std::mem::replace(self, HbValue::Nil) $op rhs;
            }
        }
    };
}
impl_assign_op!(AddAssign, add_assign, +);
impl_assign_op!(SubAssign, sub_assign, -);
impl_assign_op!(MulAssign, mul_assign, *);
impl_assign_op!(DivAssign, div_assign, /);
```

**`++`/`--`** — Rust não tem esses operadores. O codegen sempre expande:

| Operando | Contexto | Rust gerado |
|---|---|---|
| `Ident` — statement | `n++` / `++n` | `n += HbValue::Integer(1);` |
| `Ident` — expressão postfix | `n++` | `{ let _t = n.clone(); n += HbValue::Integer(1); _t }` |
| `Ident` — expressão prefix | `++n` | `{ n += HbValue::Integer(1); n.clone() }` |
| `IndexExpr` — `arr[i]++` | expressão | `{ let _t = arr[i].clone(); arr[i] += HbValue::Integer(1); _t }` |
| `CallExpr` — `fn()++` | qualquer | só `fn()` + Warning — sem dupla chamada |
| `PCol()++` em `@..SAY` | coordenada | valor bruto + Warning com sugestão `PCol() + 1` |

### 8.3 Crimes conhecidos de `++`/`--`

```harbour
nValor := UltimoRegistro()++        // Crime A: rvalue — ++ descartado silenciosamente
@ PRow(), PCol()++ @ say "TEXTO"    // Crime B: cursor não avança — ++ sem efeito
```

Regra: `++`/`--` só é válido quando operando é `Ident` ou `IndexExpr`. Qualquer outro nó AST → `SeverityLevel::Warning` via `swed_kn`.

---

## 8.4 Lexer — Tratamento do `*` comentário Harbour

`*` é comentário **somente** quando é o primeiro char não-espaço de uma linha. Antes do fix, o regex `\*[^\n]*` com `priority=3` consumia qualquer `*` (incluindo multiplicação mid-line). Fix: strip de linhas-comentário feito em `extract_defines()` antes da tokenização, regex removido do lexer.

```rust
// lexer.rs — extract_defines()
if trimmed.starts_with('*') {
    code_lines.push("");  // blank preserva numeração de linha
    continue;
}
```

Invariante garantido: `Token::Star` é **exclusivamente** multiplicação. `//` cobre comentários de estilo C também suportados por Harbour.

---

## 8.5 Parser — Nested Array Index `a[b[i]]`

O regex `BracketString` `\[[^\]]*\]` para no primeiro `]`. Para `aArray[aPosicao[1]]` o lexer emite `BracketString(" APOSICAO[ 1 ")` + `RBracket`.

Fix: `balance_bracket_content()` em `parser.rs` conta `[` vs `]` no content e consome `RBracket` extras do stream principal, recompondo o content completo antes do sub-parse. Funciona recursivamente.

```rust
// parser.rs — balance_bracket_content()
let open  = content.chars().filter(|&c| c == '[').count();
let close = content.chars().filter(|&c| c == ']').count();
let mut needed = open.saturating_sub(close);
while needed > 0 && matches!(self.peek(), Some(Token::RBracket)) {
    self.advance();
    content.push(']');
    needed -= 1;
}
```

Aplicado tanto em `parse_postfix` (leitura `a[b[i]]`) quanto em `parse_ident_stmt` (escrita `a[b[i]] := val`).

---

## 8.6 Bugs Abertos — Identificados via `resta1.prg` (2026-04-30)

| # | Arquivo | Descrição | Impacto |
|---|---|---|---|
| B3 | `parser.rs:411` | Cláusula `COLOR cCor` após `@..SAY` não consumida — vaza como statement | Parser crash |
| B4 | `codegen.rs:690` | `__EXPR_STMT__` não mapeado em `render_call` — `n++`/`n--` como statement emite `__expr_stmt__(...)` | `rustc` undefined fn |
| B5 | `swed_rt/src/value.rs` | `AddAssign`/`SubAssign` ausentes — `++n`/`--n` geram `n += HbValue::Integer(1)` | `rustc` cannot use `+=` |
| B6 | `codegen.rs:690` | Builtins sem mapeamento: `Chr`→`hb_chr`, `Replicate`→`hb_replicate`, `Space`→`hb_space`, `Max`→`hb_max`, `Min`→`hb_min`, `Array(n)`→sem equivalente, `SetColor`/`Inkey`/`LastKey`→não implementados | `rustc` undefined fns |
| B7 | `codegen.rs:463` | `STORE` emite var em maiúsculas (ex: `NLINHA`) — deve ser `to_snake_case(var)` | `rustc` undefined var |
| B8 | `codegen.rs:179` | `function main()` gera `fn main() -> HbValue` — Rust só aceita `fn main()` sem retorno | `rustc` invalid main |

**Fix B3:** adicionar `try_consume_color()` em `parse_at_stmt` análogo ao `try_consume_picture()`.

**Fix B4:** adicionar `"__EXPR_STMT__"` em `render_call` para emitir apenas a expressão interna.

**Fix B6:** adicionar mapeamentos em `render_call`: `"CHR"→hb_chr`, `"REPLICATE"→hb_replicate`, `"SPACE"→hb_space`, `"MAX"→hb_max`, `"MIN"→hb_min`, `"ARRAY"→...` (sem equivalente direto no runtime — precisa de `HbArray::new_with_nils(n)`).

**Fix B7:** `self.out.push_str(&format!("{pad}{} = {value}.clone();\n", to_snake_case(var)));`

**Fix B8:** em `emit_func`, checar `if f.name.to_ascii_uppercase() == "MAIN"` e gerar `fn main()` sem retorno.

---

## 9. `swed_nm` — Semantic Normalizer

Crate novo. Passe de reescrita de AST que senta entre `Semantic` e `Codegen`.

```
Parser → AST → Semantic → [swed_nm] → AST Normalizado → Codegen → Rust
                               ↓
                         swed_kn (diagnósticos + patch suggestions)
```

### Dependências

```toml
[dependencies]
swed_co = { path = "../swed_co" }  # SwedError, SeverityLevel, ErrorInterceptor
swed_rt = { path = "../swed_rt" }  # HbValue (tipos de diagnóstico)
```

Não depende de `swed_kn` — `swed_kn` é plugin injetável via `&dyn ErrorInterceptor<HbValue>`.

### Responsabilidades

| # | Responsabilidade | Detalhe |
|---|---|---|
| 1 | Rewrite `++`/`--` em rvalues | `PostfixIncr(CallExpr)` → safe fallback + Warning |
| 2 | Desambiguação de `=` | Statement raiz → `AssignExpr`; dentro de expressão → nó `HbEq` |
| 3 | Normalização de nós de comparação | `=` → `HbEq`, `==` → `HbExactEq`, `<>`/`!=`/`#` → `HbNe` |
| 4 | Detecção de `PCol()++`/`PRow()++` em `@..SAY` | Warning + sugestão de `PCol() + 1` |
| 5 | Elevação de `++` em tipo estático `Nil`/`String` | `Warning` → `Critical` quando tipo é inferível |
| 6 | **Detecção de funções nativas usadas na AST** | Mapeia quais `hb_*` de `swed_bf` são chamados — alimenta gerador de `_module.rs` |

---

## 10. `swed_ui` — TUI / Ratatui (implementado)

Dois sistemas coexistentes — ambos em `swed_ui/src/`.

### Sistema AppState (usado pelo codegen `@..SAY`/`@..GET`/`READ`)

```
swed_ui/src/
  traits.rs       — trait GetElement { render, handle_key, value, set_value, is_done, label }
  app_state.rs    — AppState + ReadResult::Confirmed(Vec<HbValue>) | Cancelled
  say.rs          — render de @..SAY sem READ (println!)
  widgets/
    char_input.rs, numeric_input.rs, date_input.rs, logical_toggle.rs
```

```rust
pub enum ReadResult { Confirmed(Vec<HbValue>), Cancelled }
pub struct AppState { fields: Vec<Box<dyn GetElement>> }
// AppState::new(fields).run() → io::Result<ReadResult>
```

### Sistema GetList (PICTURE real, commit/cancel — mais fiel ao Harbour)

```
swed_ui/src/
  get_element.rs  — trait GetElement { handle_char, handle_backspace, commit, cancel, edit_buf, ... }
                    + GetWidget (PICTURE masks: 9 A L X @!)
  get_list.rs     — GetList { set_active, next, prev, commit_all, cancel_all, get_value }
  read.rs         — hb_read(&mut GetList) → io::Result<bool>  (ratatui::DefaultTerminal)
  lib.rs          — GETLIST_ACTIVE: thread_local RefCell + active_get_value()
```

```rust
pub fn hb_read(list: &mut GetList) -> io::Result<bool>
// true = confirmado (Enter no último), false = cancelado (ESC)
```

**Nota:** Os dois módulos têm um trait chamado `GetElement` — em módulos distintos, sem conflito. O codegen usa o AppState system. O GetList system é para uso futuro / direto.

### Mapeamento PICTURE → widget AppState (codegen)

| Prefixo húngaro / PICTURE | Widget gerado |
|---|---|
| `C` / `PICTURE "A"` etc. | `CharInput::new(label, max_len)` |
| `N` / `F` + `PICTURE "9..."` | `NumericInput::new(label, width, decimals)` |
| `D` | `DateInput::new(label)` |
| `L` | `LogicalToggle::new(label, false)` |

---

## 11. Codegen `@..SAY` / `@..GET` / `READ` (implementado)

`swed/src/codegen.rs`

### Pipeline de emit

```
emit_stmts()
  → detecta grupo AtSay/AtGet até AtRead
  → chama emit_read_block(&stmts[i..end])
      → find_say_label(get, &says) — infere label do SAY na mesma linha
      → render_widget_ctor(get, label) — constrói ctor do widget
      → emite bloco scoped com AppState
  → @..SAY sem READ → println! simples
```

### Padrão do bloco gerado

```rust
{
    // swed_ui importado apenas no escopo do bloco
    use swed_ui::{AppState, ReadResult, GetElement,
                  widgets::numeric_input::NumericInput,
                  widgets::char_input::CharInput};
    use swed_ui::widgets::char_input::Picture as HbPicture;

    let __fields: Vec<Box<dyn GetElement>> = vec![
        Box::new(NumericInput::new("Código:", 6, 0)),
        Box::new(CharInput::new("Nome:", 40)),
    ];
    match AppState::new(__fields).run() {
        Ok(ReadResult::Ok)     => { /* field_set para cada GET */ }
        Ok(ReadResult::Cancel) => {}
        Err(e)                 => eprintln!("UI error: {e}"),
    }
}
```

**`field_get`/`field_set` emitidos como:** `use swed_db::{field_get, field_set};` — migrado de `swed_rt` para `swed_db`.

---

## 12. `swed_db` — Camada DBF / RDD (implementado)

### Migração de `swed_rt`

Commit `c09a428`: `DbfHandler`, `WorkArea`, `Row` movidos de `swed_rt/src/` para `swed_db/src/dbf/`.

### Estrutura

```
swed_db/src/
  lib.rs                   — field_get, field_set, field_get_alias, field_set_alias
  dbf/
    mod.rs
    handler.rs             — DbfHandler (leitura/escrita de .dbf)
    work_area.rs           — WorkArea (alias, posição de cursor)
    row.rs                 — Row (campos, tipos dBASE)
  sql/mod.rs               — stub SQL RDD
```

### API pública de `field_*`

```rust
pub fn field_get(name: &str) -> HbValue
pub fn field_set(name: &str, val: HbValue)
pub fn field_get_alias(alias: &str, name: &str) -> HbValue
pub fn field_set_alias(alias: &str, name: &str, val: HbValue)
```

---

## 13. `swed_bf` — Basic Functions (implementado)

### API pública atual

```rust
// date
pub use date::{hb_date, hb_day, hb_dtos, hb_month, hb_stod, hb_year};
// misc
pub use misc::hb_type;
// numeric
pub use numeric::{hb_chr, hb_ntos, hb_str, hb_strzero};
// string
pub use string::{hb_padc, hb_padl, hb_padr};
```

**Pendentes para Dual-File Codegen:** `hb_range`, `hb_left`, `hb_right`, `hb_alltrim` — necessários para transpilação de código Harbour mais rico.

Assinaturas aceitam `HbValue` por valor. **Pendente (seção 14):** migrar para `impl Into<HbValue>` para permitir chamadas com literais diretos.

---

## 14. Dual-File Codegen — `<nome>.rs` + `<nome>_module.rs` (planejado)

**Problema:** funções nativas (`hb_range`, `hb_str`, etc.) geradas pelo Codegen falham por estarem fora de escopo no Rust compilado.

### Estrutura por PRG

Para cada `<nome>.prg` o Codegen emitirá dois arquivos:

| Arquivo | Conteúdo |
|---|---|
| `<nome>.rs` | Lógica transpilada. Apenas `use crate::<nome>_module::*;` no topo. |
| `<nome>_module.rs` | Prelude: `pub use` de `HbValue`, funções `swed_bf` detectadas, `publics_var`. |

### Conteúdo do `_module.rs`

```rust
pub use crate::swed_rt::HbValue;
pub use crate::swed_bf::{hb_str, hb_range, /* apenas funções na AST */};
pub use crate::publics_var;
```

`swed_nm` é responsável por detectar quais funções nativas são referenciadas na AST e informar o Codegen — apenas essas entram no `pub use`.

### Topo do `<nome>.rs`

```rust
use crate::<nome>_module::*; // puxa todo o ambiente preparado
```

### Ergonomia — `impl Into<HbValue>`

```rust
// swed_bf — assinaturas alvo
pub fn hb_str(num: impl Into<HbValue>, len: impl Into<HbValue>, dec: impl Into<HbValue>) -> HbValue

// codegen emite
hb_str(val, 10.into(), 0.into())
// ou, com assinaturas genéricas
hb_str(val, 10, 0)
```

**Regra de ouro:** `_module.rs` é oficial de ligação apenas. Zero duplicação de lógica.

---

## 15. Variáveis PUBLIC / MEMVAR — `publics_var`

Singleton thread-safe em `swed_rt/src/publics_var.rs`. Um único `HashMap<String, HbValue>` para PUBLIC e MEMVAR/PRIVATE (ambos mapeiam para o mesmo armazém global no código transpilado).

**Nomes armazenados em UPPERCASE.**

### API de funções livres (emitidas pelo codegen)

```rust
// Importado automaticamente pelo preamble do código gerado:
use swed_rt::{pub_declare, pub_get, pub_set, memvar_assign, memvar_get};

pub fn pub_declare(name: &str, init: HbValue)   // PUBLIC x := val
pub fn pub_get(name: &str) -> HbValue           // leitura PUBLIC
pub fn pub_set(name: &str, val: HbValue)        // atribuição PUBLIC
pub fn memvar_assign(name: &str, val: HbValue)  // MEMVAR x ou x := val
pub fn memvar_get(name: &str) -> HbValue        // leitura MEMVAR / m->x
```

**Prioridade de resolução no Codegen:** FIELD > PUBLIC > MEMVAR > LOCAL

```rust
// FIELD → field_get/field_set (via swed_db — import condicional dentro da função)
// PUBLIC → pub_get / pub_set / pub_declare
// MEMVAR → memvar_get / memvar_assign
// LOCAL  → variável Rust local (snake_case)
```

O `use swed_db::{field_get, field_set}` é emitido **dentro do corpo da função** apenas quando há `FIELD` declarations — projetos sem DBF não veem o import.

### 15.1 Macro `m->` (MEMVAR Access)

```harbour
m->varname   // Acessa variável dinâmica (PUBLIC/PRIVATE)
m->nCounter  // Ignora LOCAL nCounter mesmo que exista
```

**Parser:** `m->varname` → `ExprKind::Macro("VARNAME")` (distinguido de `ALIAS->FIELD` → `ExprKind::Field`).

**Codegen:** `m->foo` sempre emite `memvar_get("FOO")` — ignora escopo local, ignora FIELD.

```rust
// Harbour: m->nCounter  →  Rust gerado:
memvar_get("NCOUNTER")
```

Diferença crítica:
```harbour
LOCAL nX := 10
PUBLIC nX := 20
? nX     // 10 — LOCAL tem prioridade
? m->nX  // 20 — força PUBLIC
```

### 15.2 `publics_project.rs` — Override por Projeto (planejado)

Arquivo **opcional** gerado pelo Codegen para projetos com PUBLIC globais explícitas.

```
projeto/
├── src/
│   ├── publics_project.rs  ← Gerado se houver globais no .prg
│   ├── main.rs
│   └── ...
└── Cargo.toml
```

**Hierarquia de resolução no Codegen:**
1. Procura em `publics_project.rs` (símbolos específicos do projeto)
2. Fallback para `public_store()` de `publics_var.rs` (runtime)
3. Busca dinâmica em memvars

**Thread safety:** `OnceLock<RwLock<T>>` — inicialização lazy única, múltiplos leitores simultâneos, escrita exclusiva. O lock deve ser solto antes de chamar funções que possam tentar adquiri-lo novamente.

---

## 16. `HbValue` — Semântica Detalhada

### Display — Float sem zeros à direita

```rust
// 3.00 → "3" | 3.50 → "3.5" | 3.14 → "3.14"
let s = format!("{:.10}", f);
let s = s.trim_end_matches('0');
let s = s.trim_end_matches('.');
```

### Divisão por zero

```rust
Integer(a) / Integer(0) → HbValue::Nil   // Harbour behavior — não panic
Float(a)   / Float(0.0) → HbValue::Nil
```

### Coerção numérica (promoção)

```rust
Integer(3) + Float(1.5) → Float(4.5)   // qualquer Float → resultado Float
```

### Aritmética de Datas — Algoritmo Howard Hinnant

`HbValue::Date(i32)` = dias desde 01/01/1970. Implementação pura em `swed_rt/src/value.rs` (sem `chrono`).

```rust
fn days_to_ymd(days: i32) -> (i32, u32, u32) { /* Hinnant civil date */ }
fn ymd_to_days(y: i32, m: u32, d: u32) -> i32 { /* inverso */ }
```

**Formato de saída:** DD/MM/YYYY (brasileiro).

**Operações:**
- `Date(d) + Integer(n)` → nova data (+n dias)
- `Date(d1) - Date(d2)` → `Integer` (diferença em dias)
- `Date(d1) < Date(d2)` → comparação direta (via dias)

### Concatenação de Strings

```rust
(HbValue::String(a), HbValue::String(b)) => HbValue::String(a + &b)  // operador +
```

### `hb_str_format` — STR() do Harbour

Espelha `STR(val, width, dec)`: alinhamento à direita, preenchido com espaços.

```rust
pub fn hb_str_format(value: HbValue, width: HbValue, dec: HbValue) -> HbValue
// hb_str_format(Integer(123), Integer(5), Integer(0)) → String("  123")
// hb_str_format(Float(3.14),  Integer(8), Integer(2)) → String("    3.14")
```

### Funções Numéricas (`swed_bf`)

- `hb_trunc(n, dec)` — trunca para `dec` casas sem arredondar
- `hb_round(n, dec)` — arredonda para `dec` casas (banker's rounding Harbour)

---

## 17. VS Code / LSP (planejado)

### Formatos de arquivo suportados

| Extensão | Conteúdo |
|---|---|
| `.mkp` | Procedures (Harbour puro) |
| `.mks` | Scripts / utilities |
| `.mkc` | Classes |

### Funcionalidades LSP

**Go to Definition:**
- Funções (locais e globais)
- Variáveis PUBLIC mapeadas em `publics_var.rs` / `publics_project.rs`
- Classes e métodos

**Hover:** `ValType()` + valor inicial se declarado em `.ch` headers.

**Autocomplete globais:** LSP lê `publics_var.rs` em indexação; Codegen gera `.deepindex` (JSON) com símbolos públicos.

### Arquitetura LSP

```
┌─────────────────────────────────┐
│   VS Code Extension             │
├─────────────────────────────────┤
│   LSP Client (native/WASM)      │
└────────────────┬────────────────┘
                 ↓
┌─────────────────────────────────┐
│   LSP Server (Rust / stdio)     │
├─────────────────────────────────┤
│ • Lexer + Parser (.mkp/.mks)    │
│ • Symbol Table (scope.rs)       │
│ • publics_var.rs Reader         │
│ • .deepindex Generator          │
└────────────────┬────────────────┘
                 ↓
        SWed Transpiler Core
        (reutiliza modules)
```

### Schema `.deepindex`

```json
{
  "functions": [
    { "name": "MyFunction", "file": "src/utils.mkp", "line": 42,
      "params": ["cName", "nValue"], "returnType": "Logical" }
  ],
  "variables": [
    { "name": "nCounter", "scope": "PUBLIC", "file": "publics_var.rs",
      "line": 10, "type": "Integer", "initialValue": "0" }
  ],
  "classes": [
    { "name": "MyClass", "file": "src/classes.mkc", "line": 5,
      "methods": ["new", "execute"] }
  ]
}
```
