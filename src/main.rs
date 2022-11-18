use macroquad::prelude::*;

const PLAYER_SIZE: Vec2 = const_vec2!([150f32, 40f32]);
const PLAYER_SPEED: f32 = 700f32;
const BLOCK_SIZE: Vec2 = const_vec2!([100f32, 40f32]);
const BALL_SIZE: f32 = 50f32;
const BALL_SPEED: f32 = 400f32;

pub fn draw_title_text(text: &str, font: Font) {
    let dims = measure_text(text, Some(font), 50u16, 1.0f32);
    draw_text_ex(
        text,
        screen_width() * 0.5f32 - dims.width * 0.5f32,
        screen_height() * 0.5f32 - dims.height * 0.5f32,
        TextParams {
            font,
            font_size: 50u16,
            color: BLACK,
            ..Default::default()
        },
    );
}
pub enum GameState {
    Menu,
    Game,
    LevelCompleted,
    Dead,
}
struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(
                screen_width() * 0.5f32 - PLAYER_SIZE.x * 0.5f32,
                screen_height() - 100f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            ),
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut x_move = 0f32;
        if is_key_down(KeyCode::Left) {
            x_move -= 1f32;
        }
        if is_key_down(KeyCode::Right) {
            x_move += 1f32;
        }
        self.rect.x += x_move * dt * PLAYER_SPEED;

        if self.rect.x < 0f32 {
            self.rect.x = 0f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.rect.x = screen_width() - self.rect.w;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, BLUE);
    }
}

#[derive(PartialEq)]
pub enum BlockType {
    Regular,
    SpawnBallOnDeath,
}

struct Block {
    rect: Rect,
    lives: i32,
    block_type: BlockType,
}

impl Block {
    pub fn new(pos: Vec2, block_type: BlockType) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BLOCK_SIZE.x, BLOCK_SIZE.y),
            lives: 2,
            block_type,
        }
    }
    pub fn draw(&self) {
        let color = match self.block_type {
            BlockType::Regular => match self.lives {
                2 => WHITE,
                _ => ORANGE,
            },
            BlockType::SpawnBallOnDeath => GREEN,
        };
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
    }
}

struct Ball {
    rect: Rect,
    vel: Vec2,
}

impl Ball {
    pub fn new(pos: Vec2) -> Self {
        Self {
            rect: Rect::new(pos.x, pos.y, BALL_SIZE, BALL_SIZE),
            vel: vec2(rand::gen_range(-1f32, 1f32), 1f32).normalize(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * BALL_SPEED;
        self.rect.y += self.vel.y * dt * BALL_SPEED;
        if self.rect.x < 0f32 {
            self.vel.x = 1f32;
        }
        if self.rect.x > screen_width() - self.rect.w {
            self.vel.x = -1f32;
        }
        if self.rect.y < 0f32 {
            self.vel.y = 1f32;
        }
    }

    pub fn draw(&self) {
        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, DARKGRAY);
    }
}
// aabb collision with positional correction
fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &Rect) -> bool {
    // early exit
    let intersection = match a.intersect(*b) {
        Some(intersection) => intersection,
        None => return false,
    };
    let a_center = a.center();
    let b_center = b.center();
    let to = b_center - a_center;
    let to_signum = to.signum();
    match intersection.w > intersection.h {
        true => {
            // bounce on y
            a.y -= to_signum.y * intersection.h;
            match to_signum.y > 0f32 {
                true => vel.y = -vel.y.abs(),
                false => vel.y = vel.y.abs(),
            }
        }
        false => {
            // bounce on x
            a.x -= to_signum.x * intersection.h;
            match to_signum.x < 0f32 {
                true => vel.x = vel.x.abs(),
                false => vel.x = -vel.x.abs(),
            }
        }
    }
    true
}

fn reset_game(
    score: &mut i32,
    player_lives: &mut i32,
    blocks: &mut Vec<Block>,
    balls: &mut Vec<Ball>,
    player: &mut Player,
) {
    *player = Player::new();
    *score = 0;
    *player_lives = 3;
    balls.clear();
    blocks.clear();
    init_blocks(blocks);
}

fn init_blocks(blocks: &mut Vec<Block>) {
    let (width, height) = (6, 6);
    let padding = 5f32;
    let total_block_size = BLOCK_SIZE + vec2(padding, padding);
    let board_start_pos = vec2(
        (screen_width() - (total_block_size.x * width as f32)) * 0.5f32,
        50f32,
    );
    for i in 0..width * height {
        let block_x = (i % width) as f32 * total_block_size.x;
        let block_y = (i / width) as f32 * total_block_size.y;
        blocks.push(Block::new(
            board_start_pos + vec2(block_x, block_y),
            BlockType::Regular,
        ));
    }
    let mut last_num = 36;
    for _ in 0..3 {
        let rand_index = rand::gen_range(0, blocks.len());
        println!("--"); // seperator
        println!("{rand_index}"); // can be dupe index
        println!("{last_num}");
        if rand_index == last_num {
            let rand_index = rand::gen_range(0, blocks.len());
            blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
            println!("-- reroll"); // seperator
            println!("{rand_index}"); // can be dupe index
            println!("{last_num}");
            last_num = rand_index;
        } else {
            last_num = rand_index;
            blocks[rand_index].block_type = BlockType::SpawnBallOnDeath;
        }
    }
}

#[macroquad::main("breakout")]
async fn main() {
    set_pc_assets_folder("../res"); // requires ../ in helix

    let font = load_ttf_font("Heebo-VariableFont_wght.ttf").await.unwrap();
    let mut game_state = GameState::Menu;
    let mut score = 0;
    let mut player_lives = 3;

    let mut player = Player::new();
    let mut blocks = Vec::new();
    let mut balls = Vec::<Ball>::new();

    init_blocks(&mut blocks);

    loop {
        clear_background(RED); // non red background not working?
        player.draw();
        for block in blocks.iter() {
            block.draw();
        }
        for ball in balls.iter() {
            ball.draw();
        }

        match game_state {
            GameState::Menu => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Game;
                    // create ball on game start not before
                    balls.push(Ball::new(
                        player.rect.point()
                            + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32, -50f32),
                    ));
                }
                draw_title_text("Press SPACE to start", font);
            }
            GameState::Game => {
                player.update(get_frame_time());
                for ball in balls.iter_mut() {
                    ball.update(get_frame_time());
                }

                let mut spawn_later = vec![];
                for ball in balls.iter_mut() {
                    resolve_collision(&mut ball.rect, &mut ball.vel, &player.rect);
                    for block in blocks.iter_mut() {
                        if resolve_collision(&mut ball.rect, &mut ball.vel, &block.rect) {
                            block.lives -= 1;
                            if block.lives <= 0 {
                                score += 10;
                                if block.block_type == BlockType::SpawnBallOnDeath {
                                    // spawn a ball
                                    spawn_later.push(Ball::new(ball.rect.point()));
                                }
                            }
                        }
                    }
                }
                for ball in spawn_later.into_iter() {
                    balls.push(ball);
                }

                let balls_len = balls.len();
                balls.retain(|ball| ball.rect.y < screen_height());
                let removed_balls = balls_len - balls.len();
                if removed_balls > 0 && balls.is_empty() {
                    player_lives -= 1;
                    if player_lives >= 1 {
                        // only create ball if lives left
                        balls.push(Ball::new(
                            player.rect.point()
                                + vec2(player.rect.w * 0.5f32 + BALL_SIZE * 0.5f32, -50f32),
                        ))
                    };
                    if player_lives <= 0 {
                        game_state = GameState::Dead;
                    }
                }
                blocks.retain(|block| block.lives > 0);
                if blocks.is_empty() {
                    game_state = GameState::LevelCompleted;
                }
                let score_text = format!("score: {}", score);
                let score_text_dim = measure_text(&score_text, Some(font), 30u16, 1.0);
                draw_text_ex(
                    &format!("score: {}", score),
                    screen_width() * 0.5f32 - score_text_dim.width * 0.5f32,
                    40.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                draw_text_ex(
                    &format!("lives: {}", player_lives),
                    30.0,
                    40.0,
                    TextParams {
                        font,
                        font_size: 30u16,
                        color: BLACK,
                        ..Default::default()
                    },
                );
            }
            GameState::LevelCompleted => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut blocks,
                        &mut balls,
                        &mut player,
                    );
                }
                draw_title_text(&format!("you WIN! {} score", score), font);
            }
            GameState::Dead => {
                if is_key_pressed(KeyCode::Space) {
                    game_state = GameState::Menu;
                    reset_game(
                        &mut score,
                        &mut player_lives,
                        &mut blocks,
                        &mut balls,
                        &mut player,
                    );
                }
                draw_title_text(&format!("you DIED! {} score", score), font);
            }
        }
        next_frame().await
    }
}
