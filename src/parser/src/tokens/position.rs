#[derive(Copy, Clone)]
#[derive(Debug)]
pub struct Position {
    pub start: usize,
    pub end: usize,
}

impl Position {
    pub fn new(start: usize, end: usize) -> Self {
        Position { start, end }
    }

    // Nuevo método para obtener la línea de inicio contando saltos de línea en el input
    pub fn start_line(&self, input: &str) -> usize {
        input[..self.start].chars().filter(|&c| c == '\n').count() + 1
    }
}
