# hbdocs — Índice de Módulos

Documentação das funções Harbour usadas pelo transpiler SWed.
Gerado automaticamente a partir de `hbdocs.json`.

| Módulo | Funções | Descrição |
|--------|--------:|-----------|
| [Array](./Array_README.md) | 15 | Manipulação de arrays dinâmicos 1-based |
| [Compat](./Compat_README.md) | 11 | Compatibilidade Clipper — evite uso direto |
| [Core](./Core_README.md) | 13 | Tipos, eval de blocos, parâmetros, controle de fluxo essencial |
| [Database](./Database_README.md) | 55 | Work areas, navegação DBF, filtros, lock e estrutura de tabelas |
| [DateTime](./DateTime_README.md) | 15 | Conversão, extração de componentes, diferença entre datas/horas |
| [Debug](./Debug_README.md) | 3 | Call stack: nome do procedimento, arquivo e linha |
| [File](./File_README.md) | 25 | I/O de baixo nível, diretórios, memo e disco |
| [Hash](./Hash_README.md) | 25 | Tabelas hash (dicionários chave-valor) |
| [I18N](./I18N_README.md) | 5 | Seleção de idioma/codepage e mensagens localizadas |
| [IO](./IO_README.md) | 5 | stdout/stderr, execução de comandos, encerramento de processo |
| [Index](./Index_README.md) | 11 | Índices: criação, escopo, condição, chave e ordem de navegação |
| [Keyboard](./Keyboard_README.md) | 13 | Leitura de teclas, ações e buffer de teclado |
| [Math](./Math_README.md) | 9 | Arredondamento, raiz, log, abs, min/max |
| [MemVar](./MemVar_README.md) | 11 | Variáveis PRIVATE/PUBLIC e pool de memvars |
| [Misc](./Misc_README.md) | 1 | Não categorizadas |
| [Network](./Network_README.md) | 41 | Sockets TCP/UDP: criação, conexão, envio/recebimento |
| [OOP](./OOP_README.md) | 15 | Introspecção de objetos: métodos, dados, herança |
| [Runtime](./Runtime_README.md) | 6 | GC, idle loop e tratamento de erros matemáticos |
| [String](./String_README.md) | 36 | Busca, fatiamento, padding, formatação e verificações de tipo de char |
| [System](./System_README.md) | 10 | SO, variáveis de ambiente, settings globais, mensagens runtime |
| [UI](./UI_README.md) | 18 | Tela, cursor, cor, mouse, menus e browser de dados |
| **Total** | **343** | |

## Versões

- `*_README.md` — documentação legível por humanos
- `*_README_IA.md` — formato denso para contexto de IA (menor token count)

## Estrutura dos arquivos IA

Cada linha no `README_IA.md` segue o formato:

```
NOME|label completo|descrição curta|ARGS:param:doc; param:doc
```