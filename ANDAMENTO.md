# SWed — Relatório de Andamento

**Data:** 30 de Abril de 2026  
**Versão do Documento:** 1.0  
**Status Geral:** Desenvolvimento ativo — ~95% da transpilação principal funcional

---

## 📋 Resumo Executivo

**SWed** é um transpilador de código-fonte **Harbour/xBase → Rust (2021)**. Objetivo: preservar décadas de software legado com segurança de memória moderna.

| Métrica | Valor |
|---|---|
| **Componentes Principais** | 10 crates (2 binários) |
| **Arquivos Fonte** | ~50+ |
| **Testes Automatizados** | 36 em `swed_testgen` |
| **Pipeline Transpilação** | Lexer → Parser → Semantic → Codegen |
| **Completude Pipeline** | ✅ ~95% |

---

## 🏗️ Arquitetura de Módulos

```
┌─────────────────────────────────────────────────────┐
│          swed (binary) — Transpilador               │
├─────────────────────────────────────────────────────┤
│  Lexer → Parser → Semantic → [swed_nm] → Codegen   │
└─────────────────────────────────────────────────────┘
                         ↓
        ┌────────────────┴────────────────┐
        ↓                                 ↓
   Runtime (Produção)          Dev Tools (Compilação)
   ┌─────────────────────┐     ┌──────────────────┐
   │  swed_rt (lib)      │     │ swed_nm (lib)    │
   │  + swed_bf          │     │ swed_kn (lib)    │
   │  + swed_db          │     │ swed_mkh (bin)   │
   │  + swed_io          │     └──────────────────┘
   │  + swed_ui (opt)    │
   └─────────────────────┘
           ↑
      swed_co (lib)
   [tipos + traits]
```

### Crates e Responsabilidades

| Crate | Linhas | Status | Função |
|---|---|---|---|
| **swed_co** | ~500 | ✅ | Core types: `HbType`, `SwedError`, `ErrorInterceptor`, `NativeFunction` traits |
| **swed_rt** | ~2000 | ✅ | Runtime: `HbValue`, aritmética, datas, variáveis PUBLIC, unwrap seguro |
| **swed_bf** | ~1500 | ✅ | Built-in Functions: String, Date, Numeric (Left, AllTrim, Chr, Month, etc.) |
| **swed_db** | ~1200 | ✅ | RDD/DBF: `DbfHandler`, `WorkArea`, `Row`, field_get/field_set |
| **swed_io** | ~400 | ✅ | File I/O + Encoding CP1252 → UTF-8 |
| **swed_ui** | ~1200 | ✅ | TUI/Ratatui: `AppState`, `GetElement`, widgets (char_input, date_input, etc.) |
| **swed_mkh** | ~1800 | ✅ | Manifesto `.mkh`: analisador de símbolos, test generator, emitter |
| **swed** (bin) | ~2000 | ✅ | Transpilador: lexer, parser, semantic, codegen |
| **swed_kn** | ~600 | ⚠️ | Knife Tools: debug, dump hex (parcialmente implementado) |
| **swed_nm** | — | ❌ | Semantic Normalizer (planejado, não iniciado) |

---

## ✅ Funcionalidades Implementadas

### Transpilação (Lexer → Codegen)

- [x] Tokenização completa via `logos` crate
- [x] Parser recursivo descendente — `Statement`, `Expression`, `Clause`
- [x] Análise semântica com diagnosticador
- [x] Codegen → Rust idiomático
  - [x] `PROCEDURE Main()` → `fn main()`
  - [x] `FUNCTION f()` → `fn f() -> HbValue`
  - [x] `LOCAL x := v` → `let mut x = v;`
  - [x] `STATIC x := v` → `thread_local!` com `RefCell`
  - [x] `PUBLIC nVar` → `public_store().write().unwrap().set(...)`
  - [x] `FOR i := 1 TO n` → `for i in hb_range(1, n, 1)`
  - [x] `DO WHILE` / `IF / ELSEIF / ELSE` → estruturas Rust equivalentes

### Sistema de Tipo (HbValue)

- [x] `HbValue::Nil`, `::Logical`, `::Integer`, `::Float`, `::String`, `::Array`, `::Object`
- [x] Operações aritméticas (`+`, `-`, `*`, `/`, `%`)
- [x] Comparações (`==`, `!=`, `<`, `>`, `<=`, `>=`)
- [x] Operações em arrays (`[i]`, `LEN()`, `AAdd()`)
- [x] Operações em strings (Case conversion, trimming, substring)
- [x] Manipulação de datas (Month, Year, Day, Date aritmética)
- [x] `IIF()` e funções condicionais

### Variáveis e Escopo

- [x] PUBLIC — armazenadas em `publics_var.rs` thread-local store
- [x] STATIC — `thread_local!` com `RefCell`
- [x] LOCAL — stack frame via `let`
- [x] MEMVAR (macro `m->varname`) — resolução em runtime
- [x] FIELD (alias `ALIAS->fieldname`) — acesso a campos de DBF

### Banco de Dados (DBF/RDD)

- [x] `DbfHandler` — leitura/escrita de arquivos DBF
- [x] `WorkArea` — seleção de área de trabalho
- [x] `Row` — representação de registro
- [x] `field_get` / `field_set` — acesso aos campos
- [x] Suporte a múltiplas áreas de trabalho (alias)

### Interface de Usuário (TUI/Ratatui)

- [x] `AppState` — estado centralizado
- [x] `GetElement` trait — polimorfismo para widgets
- [x] Widgets:
  - [x] `CharInput` — entrada de caracteres
  - [x] `NumericInput` — entrada numérica
  - [x] `DateInput` — entrada de data
  - [x] `LogicalToggle` — toggle booleano
- [x] `GetList` / `GetWidget` — sistema de leitura estruturada
- [x] Codegen de `@..SAY` / `@..GET` / `READ` para blocos TUI

### Manifesto de Símbolos (`.mkh`)

- [x] Parser two-pass (definições + usos)
- [x] Classificação de símbolos (Function, Procedure, Class, Public, Static, etc.)
- [x] Emitter para arquivos `.mkh` em `./cache_maker/`
- [x] Test Generator (`swed_testgen` binary) — 36 testes passando
- [x] Análise de visibilidade (Exported, Hidden, Protected)

### Codificação

- [x] Suporte Windows-1252 → UTF-8 via `encoding_rs`
- [x] Leitura automática com conversão no `file_io.rs`

### Ferramentas de Desenvolvimento

- [x] `swed_testgen` — gerador de testes a partir de `.mkh`
- [x] Diagnosticador integrado (warnings, errors)
- [x] Dump de estruturas internas (AST, tokens)

---

## ⚠️ Funcionalidades Pendentes

### 1. Operadores Compostos em `HbValue`

**Status:** ❌ Não implementado  
**Impacto:** Codegen não pode emitir `+=`, `-=`, `*=`, `/=` diretamente

```rust
// Pendente: implementar traits em swed_rt/src/value.rs
impl AddAssign for HbValue { ... }
impl SubAssign for HbValue { ... }
impl MulAssign for HbValue { ... }
impl DivAssign for HbValue { ... }
```

**Prioridade:** Média  
**Esforço:** ~1-2 horas

---

### 2. Comparação Exata (`hb_eq()` / `hb_exact_eq()`)

**Status:** ❌ Não implementado  
**Impacto:** Semântica de `==` vs `EXACT()` não diferenciada

```rust
// Pendente em swed_rt/src/value.rs
fn hb_eq(&self, other: &HbValue) -> HbValue { ... }
fn hb_exact_eq(&self, other: &HbValue) -> HbValue { ... }
```

**Prioridade:** Média  
**Esforço:** ~2-3 horas

---

### 3. Semantic Normalizer (`swed_nm`)

**Status:** ❌ Não iniciado  
**Objetivo:** Passe de reescrita de AST *antes* do codegen

**Responsabilidades:**
- Desambiguação de operadores (ex: `++i` vs `i++`)
- Expansão de `++` / `--` para lógica equivalente
- Detecção de chamadas de funções nativas vs user-defined
- Emissão de diagnósticos via `ErrorInterceptor`

**Prioridade:** Alta  
**Esforço:** ~4-6 horas

---

### 4. Dual-File Codegen

**Status:** ⚠️ Parcialmente implementado (simplificado)  
**Status:** De-priorizado

Originalmente planejado:
- `<nome>.rs` — código gerado
- `<nome>_module.rs` — metadata (função, tipo, etc.)

**Status Atual:** Single-file output (gerado em uma única `.rs`)

---

### 5. Master Maker (`swed_mkm`)

**Status:** ❌ Não iniciado  
**Objetivo:** Orquestração de build baseada em `.mkh`

- Leitura de manifesto `.mkh`
- Construção de grafo de dependências
- Injeção de diretivas de compilação

**Prioridade:** Baixa  
**Esforço:** ~6-8 horas

---

### 6. ErrorInterceptor em `swed_kn`

**Status:** ⚠️ Parcialmente implementado  
**Implementado:** Dump hexadecimal de dados  
**Pendente:** Sugestão automática de patches em erros `Critical`

**Prioridade:** Baixa  
**Esforço:** ~2-3 horas

---

## 📊 Métricas de Qualidade

| Métrica | Valor |
|---|---|
| **Status de Compilação** | ✅ Compila com sucesso (`cargo check`) |
| **Testes Unitários** | 36 (via `swed_testgen`) |
| **Taxa de Cobertura** | ~70% (estimated) |
| **Clippy Warnings** | 0 ✅ |
| **Dead Code Warnings** | 5 (campos não utilizados em AST) ⚠️ |
| **Doc Warnings** | 3 (módulos `swed_ui` sem documentação) ⚠️ |
| **Unsafe Code** | Mínimo (apenas em `unwrap()` necessários) |
| **Documentação** | ~85% de funções públicas com doc comments |

---

## 🎯 Próximos Passos (Roadmap)

### Curto Prazo (1-2 sprints)

1. **Implementar `AddAssign` / `SubAssign` / `MulAssign` / `DivAssign`**
   - [ ] Adicionar traits em `swed_rt/src/value.rs`
   - [ ] Atualizar codegen para emitir `+=`, `-=`, etc.
   - [ ] Adicionar testes

2. **Implementar `hb_eq()` / `hb_exact_eq()`**
   - [ ] Adicionar métodos em `HbValue`
   - [ ] Diferenciar semântica Harbour vs Rust
   - [ ] Testes de regressão

3. **Iniciar `swed_nm` (Semantic Normalizer)**
   - [ ] Estrutura base do crate
   - [ ] Parser de operadores
   - [ ] Reescrita de AST

### Médio Prazo (3-4 sprints)

4. **Completar `swed_nm`**
   - [ ] Desambiguação completa
   - [ ] Integração com diagnosticador
   - [ ] Testes de normalização

5. **Expandir `swed_kn` (Knife Tools)**
   - [ ] Sugestões de patch automáticas
   - [ ] Análise de stack traces

6. **Teste de Compatibilidade Harbour**
   - [ ] Reproduzir projeto xBase real
   - [ ] Validar transpilação ponta-a-ponta

### Longo Prazo (5+ sprints)

7. **VS Code Extension (SWed LSP)**
   - [ ] Go-to-Definition via `.mkh`
   - [ ] Inferência de tipos
   - [ ] Highlight de erros em tempo real

8. **Master Maker (`swed_mkm`)**
   - [ ] Orquestração de build
   - [ ] Grafo de dependências

9. **Dual-File Codegen** (se necessário)
   - [ ] Revisitar necessidade
   - [ ] Implementar se confirmado

---

## 🔗 Targets Confirmados

| Target | Status | Prioridade |
|---|---|---|
| CLI/TUI (transpilação) | ✅ Funcional | ✅ Alta |
| Runtime puro (produção) | ✅ Funcional | ✅ Alta |
| VS Code Extension (LSP) | 🎯 Planejado | ⚠️ Média |
| Web IDE (WASM) | 📋 Futuro | 🔵 Baixa |

---

## 📝 Notas de Desenvolvimento

### Convenções

- **Nomenclatura Húngara:** Preservada via `HbType` (ex: `n` = numérico, `c` = caractere, `a` = array)
- **Módulos de Traits:** `swed_co::traits::*`
- **Padrão de Injeção:** `FunctionResolver`, `ErrorInterceptor` via traits
- **Thread-Safety:** `thread_local!` para STATIC, `Arc<RwLock<>>` para store compartilhado

### Problemas Conhecidos

1. **Operadores `++`/`--`**
   - Harbour permite tanto prefixo quanto sufixo com semântica diferentes
   - Rust só permite um padrão
   - Solução: `swed_nm` deve normalizar antes do codegen

2. **Macros `m->varname`**
   - Resolução em runtime via `memvar_get()` / `memvar_set()`
   - Sem type-checking em compile-time
   - Esperado comportamento — compatível com xBase

3. **Type Coercion**
   - Harbour permite operações implícitas entre tipos
   - `HbValue` retorna `Result` — usuário deve tratar erros
   - Codegen pode usar `?` operator para propagação

---

## 📚 Referências Importantes

| Arquivo | Conteúdo |
|---|---|
| [README.md](README.md) | Visão geral, arquitetura, mapping Harbour→Rust |
| [deepcontext.md](deepcontext.md) | Contexto profundo, decisões arquiteturais |
| [hbdocs.json](hbdocs.json) | Assinatura de funções Harbour (lexer+parser) |
| [swed/src/](swed/src/) | Transpilador (lexer, parser, semantic, codegen) |
| [swed_rt/src/value.rs](swed_rt/src/value.rs) | Core de `HbValue` |
| [swed_mkh/src/](swed_mkh/src/) | Manifesto e testgen |

---

## 👥 Responsáveis

- **Conceito & Arquitetura:** Inspirado por Barry Rebell e Brian Russell (Clipper)
- **Implementação Atual:** Desenvolvimento ativo

---

**Documento gerado:** 2026-04-30  
**Próxima atualização recomendada:** Quando fases do roadmap completarem
