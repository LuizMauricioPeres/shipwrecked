/// Resta 1 — implementação idiomática em Rust puro.
///
/// Tradução direta de resta1.prg sem nenhuma dependência do swed_rt.
/// Usa crossterm para I/O de terminal (raw mode, posicionamento de cursor,
/// cores ANSI) e tipos nativos do Rust (enum + Vec).
use crossterm::{
    cursor,
    event::{self, Event, KeyCode},
    execute, queue,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
};
use std::io::{self, Write};

// ---------------------------------------------------------------------------
// Tipos do domínio
// ---------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Debug)]
enum Cell {
    Peca,
    Vazio,
    Invalido,
}

struct Game {
    /// Tabuleiro 7×7, indexado em base-0.
    board: [[Cell; 7]; 7],
    /// Posição atual do cursor (linha, coluna) em base-0.
    cursor: (usize, usize),
    /// Peça selecionada pelo primeiro Enter (Some) ou nenhuma (None).
    selected: Option<(usize, usize)>,
}

// ---------------------------------------------------------------------------
// Lógica do jogo
// ---------------------------------------------------------------------------

/// Equivale a `PosicaoValida(nLinha, nColuna)` do .prg original.
/// Converte de base-0 para base-1 para manter a mesma semântica.
fn is_valid(row: usize, col: usize) -> bool {
    let r = row + 1; // base-1
    let c = col + 1;
    (r > 2 && r < 6) || (c > 2 && c < 6)
}

impl Game {
    /// Equivale a `IniciaTabuleiro()`.
    fn new() -> Self {
        let mut board = [[Cell::Invalido; 7]; 7];
        for row in 0..7 {
            for col in 0..7 {
                if is_valid(row, col) {
                    board[row][col] = Cell::Peca;
                }
            }
        }
        // Centro vazio (posição [4][4] em base-1 = [3][3] em base-0)
        board[3][3] = Cell::Vazio;

        // Cursor inicial: (5,5) base-1 → (4,4) base-0
        Game { board, cursor: (4, 4), selected: None }
    }

    /// Move o cursor em (dr, dc); rejeita posições inválidas ou fora do tabuleiro.
    fn move_cursor(&mut self, dr: i32, dc: i32) {
        let (r, c) = self.cursor;
        let nr = (r as i32 + dr).clamp(0, 6) as usize;
        let nc = (c as i32 + dc).clamp(0, 6) as usize;
        if is_valid(nr, nc) {
            self.cursor = (nr, nc);
        }
    }

    /// Equivale ao bloco `K_RETURN` em `main()` do .prg original.
    ///
    /// - Primeiro Enter: seleciona a peça na posição atual.
    /// - Segundo Enter: tenta executar o salto e desseleciona.
    fn select_or_move(&mut self) {
        let (row, col) = self.cursor;
        match self.selected {
            None => {
                if self.board[row][col] == Cell::Peca {
                    self.selected = Some((row, col));
                }
            }
            Some((sr, sc)) => {
                self.try_eat(sr, sc, row, col);
                self.selected = None;
            }
        }
    }

    /// Equivale a `possoComerPeca(aTabuleiro, nLinha, nColuna, aPecaSelecionada)`.
    ///
    /// `(from_row, from_col)` = peça selecionada (aPecaSelecionada).
    /// `(to_row,   to_col)`   = destino do salto   (nLinha, nColuna).
    fn try_eat(&mut self, from_row: usize, from_col: usize, to_row: usize, to_col: usize) {
        let b = &self.board;

        // Direita: nColuna - sel_coluna == 2
        if from_row == to_row && to_col == from_col + 2 {
            if b[to_row][to_col]       == Cell::Vazio
                && b[from_row][from_col + 1] == Cell::Peca
                && b[from_row][from_col]     == Cell::Peca
            {
                self.board[to_row][to_col]       = Cell::Peca;
                self.board[from_row][from_col + 1] = Cell::Vazio;
                self.board[from_row][from_col]     = Cell::Vazio;
                return;
            }
        }

        // Esquerda: nColuna - sel_coluna == -2  (i.e. from_col == to_col + 2)
        if from_row == to_row && from_col >= 2 && from_col == to_col + 2 {
            if b[to_row][to_col]           == Cell::Vazio
                && b[from_row][from_col - 1]   == Cell::Peca
                && b[from_row][from_col]        == Cell::Peca
            {
                self.board[to_row][to_col]         = Cell::Peca;
                self.board[from_row][from_col - 1] = Cell::Vazio;
                self.board[from_row][from_col]     = Cell::Vazio;
                return;
            }
        }

        // Baixo: nLinha - sel_linha == 2
        if from_col == to_col && to_row == from_row + 2 {
            if b[to_row][to_col]           == Cell::Vazio
                && b[from_row + 1][from_col]   == Cell::Peca
                && b[from_row][from_col]        == Cell::Peca
            {
                self.board[to_row][to_col]         = Cell::Peca;
                self.board[from_row + 1][from_col] = Cell::Vazio;
                self.board[from_row][from_col]     = Cell::Vazio;
                return;
            }
        }

        // Cima: nLinha - sel_linha == -2  (i.e. from_row == to_row + 2)
        if from_col == to_col && from_row >= 2 && from_row == to_row + 2 {
            if b[to_row][to_col]           == Cell::Vazio
                && b[from_row - 1][from_col]   == Cell::Peca
                && b[from_row][from_col]        == Cell::Peca
            {
                self.board[to_row][to_col]         = Cell::Peca;
                self.board[from_row - 1][from_col] = Cell::Vazio;
                self.board[from_row][from_col]     = Cell::Vazio;
            }
        }
    }

    /// Conta peças restantes no tabuleiro.
    fn piece_count(&self) -> usize {
        self.board.iter().flatten().filter(|&&c| c == Cell::Peca).count()
    }
}

// ---------------------------------------------------------------------------
// Renderização
// ---------------------------------------------------------------------------

/// Equivale a `DesenhaTabuleiro` + `Desenha` do .prg original.
///
/// O .prg usava `@ nLinha*2, nColuna*6 SAY ... COLOR cCor`.
/// Aqui reproduzimos o mesmo layout com crossterm.
fn draw(game: &Game, out: &mut impl Write) -> io::Result<()> {
    queue!(out, cursor::MoveTo(0, 0), terminal::Clear(ClearType::All))?;

    for (row, line) in game.board.iter().enumerate() {
        for (col, &cell) in line.iter().enumerate() {
            // Layout: linha*2, coluna*6  (base-1 → shift +1 para base-0 visual)
            let screen_row = ((row + 1) * 2) as u16;
            let screen_col = ((col + 1) * 6) as u16;
            queue!(out, cursor::MoveTo(screen_col, screen_row))?;

            let is_cursor   = (row, col) == game.cursor;
            let is_selected = game.selected == Some((row, col));

            match cell {
                Cell::Invalido => {
                    queue!(out, Print("   "))?;
                }
                Cell::Peca => {
                    // Verde = selecionada; Vermelho = cursor sobre peça; branco = normal
                    let color = if is_selected {
                        Color::Green
                    } else if is_cursor {
                        Color::Red
                    } else {
                        Color::White
                    };
                    queue!(out, SetForegroundColor(color), Print("███"), ResetColor)?;
                }
                Cell::Vazio => {
                    // Vermelho = cursor sobre vazio; cinza = normal
                    if is_cursor {
                        queue!(out, SetForegroundColor(Color::Red), Print("░░░"), ResetColor)?;
                    } else {
                        queue!(out, SetForegroundColor(Color::DarkGrey), Print("░░░"), ResetColor)?;
                    }
                }
            }
        }
    }

    // Linha de status abaixo do tabuleiro
    queue!(out, cursor::MoveTo(0, 18))?;
    let pieces = game.piece_count();
    match game.selected {
        Some((r, c)) => {
            queue!(
                out,
                SetForegroundColor(Color::Green),
                Print(format!("Peça ({},{}) selecionada — mova o cursor e pressione Enter ", r + 1, c + 1)),
                ResetColor,
            )?;
        }
        None => {
            queue!(
                out,
                Print(format!(
                    "Peças: {:2}  │  ↑↓←→ mover  Enter selecionar/mover  ESC sair  ",
                    pieces
                )),
            )?;
        }
    }

    out.flush()
}

// ---------------------------------------------------------------------------
// Ponto de entrada
// ---------------------------------------------------------------------------

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();

    terminal::enable_raw_mode()?;
    execute!(stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

    let mut game = Game::new();
    let mut running = true;

    while running {
        draw(&game, &mut stdout)?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc           => running = false,
                KeyCode::Up            => game.move_cursor(-1,  0),
                KeyCode::Down          => game.move_cursor( 1,  0),
                KeyCode::Left          => game.move_cursor( 0, -1),
                KeyCode::Right         => game.move_cursor( 0,  1),
                KeyCode::Enter | KeyCode::Char(' ') => game.select_or_move(),
                _ => {}
            }
        }
    }

    execute!(stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
    terminal::disable_raw_mode()?;

    println!("Peças restantes: {}", game.piece_count());
    Ok(())
}
