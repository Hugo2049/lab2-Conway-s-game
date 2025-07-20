use minifb::{Key, Window, WindowOptions};
use rand::Rng;
use std::thread;
use std::time::Duration;

// Configuración de la ventana y framebuffer
const WINDOW_WIDTH: usize = 800;
const WINDOW_HEIGHT: usize = 600;
const GRID_WIDTH: usize = 100;
const GRID_HEIGHT: usize = 75;

// Colores
const WHITE: u32 = 0xFFFFFF; // Célula viva
const BLACK: u32 = 0x000000; // Célula muerta
const GRAY: u32 = 0x333333;  // Fondo opcional

struct Framebuffer {
    buffer: Vec<u32>,
    width: usize,
    height: usize,
}

impl Framebuffer {
    fn new(width: usize, height: usize) -> Self {
        Self {
            buffer: vec![BLACK; width * height],
            width,
            height,
        }
    }

    fn point(&mut self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index] = color;
        }
    }

    fn get_color(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            self.buffer[index]
        } else {
            BLACK
        }
    }

    fn clear(&mut self, color: u32) {
        self.buffer.fill(color);
    }

    // Escalar el framebuffer pequeño a la ventana grande
    fn scale_to_window(&self) -> Vec<u32> {
        let mut scaled_buffer = vec![BLACK; WINDOW_WIDTH * WINDOW_HEIGHT];
        
        let scale_x = WINDOW_WIDTH as f32 / self.width as f32;
        let scale_y = WINDOW_HEIGHT as f32 / self.height as f32;

        for y in 0..WINDOW_HEIGHT {
            for x in 0..WINDOW_WIDTH {
                let src_x = (x as f32 / scale_x) as usize;
                let src_y = (y as f32 / scale_y) as usize;
                
                if src_x < self.width && src_y < self.height {
                    let color = self.get_color(src_x, src_y);
                    scaled_buffer[y * WINDOW_WIDTH + x] = color;
                }
            }
        }

        scaled_buffer
    }
}

struct GameOfLife {
    current_state: Vec<Vec<bool>>,
    next_state: Vec<Vec<bool>>,
    width: usize,
    height: usize,
    framebuffer: Framebuffer,
}

impl GameOfLife {
    fn new(width: usize, height: usize) -> Self {
        let current_state = vec![vec![false; width]; height];
        let next_state = vec![vec![false; width]; height];
        let framebuffer = Framebuffer::new(width, height);

        Self {
            current_state,
            next_state,
            width,
            height,
            framebuffer,
        }
    }

    // Inicializar con patrones conocidos
    fn initialize_with_patterns(&mut self) {
        // Limpiar el estado
        for y in 0..self.height {
            for x in 0..self.width {
                self.current_state[y][x] = false;
            }
        }

        // === PATRONES EN DIFERENTES ÁREAS ===
        
        // Área superior izquierda - Still lifes
        self.add_block(5, 5);
        self.add_beehive(12, 8);
        self.add_loaf(5, 15);
        self.add_boat(15, 18);
        self.add_tub(8, 25);

        // Área superior central - Oscillators
        self.add_blinker(35, 5);
        self.add_toad(40, 10);
        self.add_beacon(45, 15);
        self.add_pulsar(32, 25);

        // Área superior derecha - Spaceships
        self.add_glider(70, 8);
        self.add_lightweight_spaceship(65, 15);
        self.add_middleweight_spaceship(60, 25);

        // === ÁREA INFERIOR DERECHA - PATRONES ESPECIALES ===
        self.add_heavyweight_spaceship(self.width - 25, self.height - 15);
        self.add_pentadecathlon(self.width - 15, self.height - 25);
        self.add_gosper_glider_gun(self.width - 40, self.height - 20);
        
        // Algunos patrones adicionales dispersos
        self.add_r_pentomino(25, 50);
        self.add_diehard(15, 40);
        
        // Agregar algunas células aleatorias para variedad
        let mut rng = rand::thread_rng();
        for _ in 0..30 {
            let x = rng.gen_range(5..self.width-5);
            let y = rng.gen_range(35..50);
            self.current_state[y][x] = true;
        }

        self.update_framebuffer();
    }

    // Patrones del Game of Life
    fn add_glider(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, false],
            vec![false, false, true],
            vec![true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_blinker(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true],
            vec![true],
            vec![true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_block(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true, true],
            vec![true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_beehive(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![false, true, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_toad(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, true, true],
            vec![true, true, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    // === STILL LIFES ADICIONALES ===
    
    fn add_loaf(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, true, false],
            vec![true, false, false, true],
            vec![false, true, false, true],
            vec![false, false, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_boat(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true, true, false],
            vec![true, false, true],
            vec![false, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_tub(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, false],
            vec![true, false, true],
            vec![false, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    // === OSCILLATORS ADICIONALES ===
    
    fn add_beacon(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true, true, false, false],
            vec![true, true, false, false],
            vec![false, false, true, true],
            vec![false, false, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_pulsar(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true, true, true, false, false, false, true, true, true, false, false],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![true, false, false, false, false, true, false, true, false, false, false, false, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, true, true, true, false, false, false, true, true, true, false, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_pentadecathlon(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true, true, true],
            vec![true, false, true],
            vec![true, false, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, true, true],
            vec![true, false, true],
            vec![true, false, true],
            vec![true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    // === SPACESHIPS ADICIONALES ===
    
    fn add_lightweight_spaceship(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![true, false, false, true, false],
            vec![false, false, false, false, true],
            vec![true, false, false, false, true],
            vec![false, true, true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_middleweight_spaceship(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, false, true, false, false, false],
            vec![true, false, false, false, true, false],
            vec![false, false, false, false, false, true],
            vec![true, false, false, false, false, true],
            vec![false, true, true, true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_heavyweight_spaceship(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, false, true, true, false, false, false],
            vec![true, false, false, false, false, true, false],
            vec![false, false, false, false, false, false, true],
            vec![true, false, false, false, false, false, true],
            vec![false, true, true, true, true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    // === PATRONES ESPECIALES ===
    
    fn add_gosper_glider_gun(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true],
            vec![false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, true, true],
            vec![true, true, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![true, true, false, false, false, false, false, false, false, false, true, false, false, false, true, false, true, true, false, false, false, false, true, false, true, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, true, false, false, false, false, false, true, false, false, false, false, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
            vec![false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_r_pentomino(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, true, true],
            vec![true, true, false],
            vec![false, true, false],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_diehard(&mut self, start_x: usize, start_y: usize) {
        let pattern = vec![
            vec![false, false, false, false, false, false, true, false],
            vec![true, true, false, false, false, false, false, false],
            vec![false, true, false, false, false, true, true, true],
        ];
        self.add_pattern(start_x, start_y, &pattern);
    }

    fn add_pattern(&mut self, start_x: usize, start_y: usize, pattern: &[Vec<bool>]) {
        for (dy, row) in pattern.iter().enumerate() {
            for (dx, &cell) in row.iter().enumerate() {
                let x = start_x + dx;
                let y = start_y + dy;
                if x < self.width && y < self.height {
                    self.current_state[y][x] = cell;
                }
            }
        }
    }

    // Contar vecinos vivos de una célula
    fn count_neighbors(&self, x: usize, y: usize) -> usize {
        let mut count = 0;
        
        // Verificar los 8 vecinos
        for dy in -1i32..=1 {
            for dx in -1i32..=1 {
                // Saltar la célula central
                if dx == 0 && dy == 0 {
                    continue;
                }
                
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                
                // Verificar límites
                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    if self.current_state[ny as usize][nx as usize] {
                        count += 1;
                    }
                }
            }
        }
        
        count
    }

    // Actualizar el estado del juego según las reglas de Conway
    fn update(&mut self) {
        // Calcular el próximo estado
        for y in 0..self.height {
            for x in 0..self.width {
                let neighbors = self.count_neighbors(x, y);
                let current_cell = self.current_state[y][x];
                
                self.next_state[y][x] = match (current_cell, neighbors) {
                    // Célula viva con menos de 2 vecinos muere (underpopulation)
                    (true, n) if n < 2 => false,
                    // Célula viva con 2 o 3 vecinos sobrevive
                    (true, 2) | (true, 3) => true,
                    // Célula viva con más de 3 vecinos muere (overpopulation)
                    (true, n) if n > 3 => false,
                    // Célula muerta con exactamente 3 vecinos vive (reproduction)
                    (false, 3) => true,
                    // Cualquier otro caso mantiene el estado actual
                    (state, _) => state,
                };
            }
        }

        // Intercambiar los estados
        std::mem::swap(&mut self.current_state, &mut self.next_state);
        
        // Actualizar el framebuffer
        self.update_framebuffer();
    }

    // Actualizar el framebuffer con el estado actual
    fn update_framebuffer(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let color = if self.current_state[y][x] {
                    WHITE
                } else {
                    BLACK
                };
                self.framebuffer.point(x, y, color);
            }
        }
    }

    fn get_scaled_buffer(&self) -> Vec<u32> {
        self.framebuffer.scale_to_window()
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut window = Window::new(
        "Game of Life - Conway",
        WINDOW_WIDTH,
        WINDOW_HEIGHT,
        WindowOptions::default(),
    )?;

    // Limitar FPS
    window.limit_update_rate(Some(std::time::Duration::from_millis(100)));

    let mut game = GameOfLife::new(GRID_WIDTH, GRID_HEIGHT);
    game.initialize_with_patterns();

    
    println!("Atajos");
    println!("ESC - Salir");
    println!("R - Reinicializar con nuevos patrones");
    println!();
    
    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Manejar input
        if window.is_key_pressed(Key::R, minifb::KeyRepeat::No) {
            game.initialize_with_patterns();
        }

        // Actualizar el juego
        game.update();

        // Renderizar
        let scaled_buffer = game.get_scaled_buffer();
        window.update_with_buffer(&scaled_buffer, WINDOW_WIDTH, WINDOW_HEIGHT)?;

        // Pequeño delay para poder visualizar mejor la animación
        thread::sleep(Duration::from_millis(50));
    }

    Ok(())
}
