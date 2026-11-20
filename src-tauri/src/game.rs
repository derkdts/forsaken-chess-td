use crate::generator::{generate_map, GRID_SIZE};
use rand::seq::SliceRandom;
use serde::Serialize;
const SUMMON_COST: u32 = 40;
const MAX_HERO_LEVEL: u8 = 5;
#[derive(Clone, Serialize)]
pub struct Hero {
    pub gx: usize,
    pub gy: usize,
    pub level: u8,
    pub cooldown: f32,
    pub max_cd: f32,
    pub range: f32,
    pub dmg: f32,
    pub color: String,
    pub symbol: String,
    pub title: String,
}
impl Hero {
    fn stats_for_level(level: u8) -> (&'static str, &'static str, &'static str, f32, f32, f32) {
        match level {
            1 => ("Пешка", "♟", "#90caf9", 2.2, 0.95, 25.0),
            2 => ("Конь", "♞", "#69f0ae", 2.6, 0.65, 42.0),
            3 => ("Слон", "♝", "#ffd740", 3.4, 0.80, 68.0),
            4 => ("Ладья", "♜", "#ff8a65", 3.0, 1.05, 130.0),
            _ => ("Ферзь", "♛", "#e040fb", 3.9, 0.55, 195.0),
        }
    }
    pub fn new(gx: usize, gy: usize) -> Self {
        let (title, symbol, color, range, max_cd, dmg) = Self::stats_for_level(1);
        Self {
            gx,
            gy,
            level: 1,
            cooldown: 0.0,
            max_cd,
            range,
            dmg,
            color: color.to_string(),
            symbol: symbol.to_string(),
            title: title.to_string(),
        }
    }
    pub fn upgrade(&mut self) {
        if self.level < MAX_HERO_LEVEL {
            self.level += 1;
            let (title, symbol, color, range, max_cd, dmg) = Self::stats_for_level(self.level);
            self.title = title.to_string();
            self.symbol = symbol.to_string();
            self.color = color.to_string();
            self.range = range;
            self.max_cd = max_cd;
            self.dmg = dmg;
        }
    }
}
#[derive(Clone, Serialize)]
pub struct Enemy {
    pub gx: f32,
    pub gy: f32,
    pub wp_index: usize,
    pub hp: f32,
    pub max_hp: f32,
    pub speed: f32,
    pub reward: u32,
}
#[derive(Clone, Serialize)]
pub struct Projectile {
    pub from_gx: f32,
    pub from_gy: f32,
    pub to_gx: f32,
    pub to_gy: f32,
    pub color: String,
    pub life: f32,
}
#[derive(Clone, Serialize, PartialEq)]
pub enum MatchStatus {
    Playing,
    Victory,
    Defeat,
}
#[derive(Clone, Serialize)]
pub struct GameState {
    pub stage: u32,
    pub lost: u32,
    pub max_lost: u32,
    pub win: u32,
    pub win_target: u32,
    pub mana: u32,
    pub speed: u32,
    pub grid_size: usize,
    pub grid: Vec<Vec<u8>>,
    #[serde(skip)]
    pub waypoints: Vec<(f32, f32)>,
    pub heroes: Vec<Hero>,
    pub enemies: Vec<Enemy>,
    pub projectiles: Vec<Projectile>,
    #[serde(skip)]
    pub spawn_timer: f32,
    pub status: MatchStatus,
}
impl GameState {
    pub fn new(stage: u32) -> Self {
        let (grid, waypoints) = generate_map(stage);
        Self {
            stage,
            lost: 0,
            max_lost: 10,
            win: 0,
            win_target: 12 + stage * 3,
            mana: 100 + (stage - 1) * 20,
            speed: 1,
            grid_size: GRID_SIZE,
            grid,
            waypoints,
            heroes: Vec::new(),
            enemies: Vec::new(),
            projectiles: Vec::new(),
            spawn_timer: 0.5,
            status: MatchStatus::Playing,
        }
    }
    pub fn summon_or_upgrade(&mut self, gx: Option<usize>, gy: Option<usize>) {
        if self.mana < SUMMON_COST || self.status != MatchStatus::Playing {
            return;
        }
        let target = match (gx, gy) {
            (Some(x), Some(y)) => Some((x, y)),
            _ => {
                let free: Vec<(usize, usize)> = (0..GRID_SIZE)
                    .flat_map(|y| (0..GRID_SIZE).map(move |x| (x, y)))
                    .filter(|&(x, y)| {
                        self.grid[y][x] == 2
                            && !self.heroes.iter().any(|h| h.gx == x && h.gy == y)
                    })
                    .collect();
                free.choose(&mut rand::thread_rng()).copied()
            }
        };
        let Some((tx, ty)) = target else { return };
        if ty >= GRID_SIZE || tx >= GRID_SIZE || self.grid[ty][tx] != 2 {
            return;
        }
        if let Some(hero) = self.heroes.iter_mut().find(|h| h.gx == tx && h.gy == ty) {
            if hero.level < MAX_HERO_LEVEL {
                self.mana -= SUMMON_COST;
                hero.upgrade();
            }
        } else {
            self.mana -= SUMMON_COST;
            self.heroes.push(Hero::new(tx, ty));
        }
    }
    pub fn tick(&mut self, raw_dt: f32) {
        if self.status != MatchStatus::Playing {
            return;
        }
        let dt = raw_dt * (self.speed as f32);
        self.spawn_timer -= dt;
        let total_spawned = self.win + self.lost + (self.enemies.len() as u32);
        if self.spawn_timer <= 0.0 && total_spawned < self.win_target + self.max_lost {
            let start = self.waypoints[0];
            let base_hp = 60.0 + (self.stage as f32) * 35.0;
            self.enemies.push(Enemy {
                gx: start.0,
                gy: start.1,
                wp_index: 1,
                hp: base_hp,
                max_hp: base_hp,
                speed: 1.3 + ((self.stage as f32) * 0.08).min(1.2),
                reward: 15,
            });
            self.spawn_timer = (1.8 - (self.stage as f32) * 0.08).max(0.6);
        }
        let mut i = self.enemies.len();
        while i > 0 {
            i -= 1;
            let wp_idx = self.enemies[i].wp_index;
            if wp_idx >= self.waypoints.len() {
                self.enemies.remove(i);
                self.lost += 1;
                if self.lost >= self.max_lost {
                    self.status = MatchStatus::Defeat;
                }
                continue;
            }
            let (tx, ty) = self.waypoints[wp_idx];
            let dx = tx - self.enemies[i].gx;
            let dy = ty - self.enemies[i].gy;
            let dist = dx.hypot(dy);
            if dist < 0.05 {
                self.enemies[i].gx = tx;
                self.enemies[i].gy = ty;
                self.enemies[i].wp_index += 1;
            } else {
                self.enemies[i].gx += (dx / dist) * self.enemies[i].speed * dt;
                self.enemies[i].gy += (dy / dist) * self.enemies[i].speed * dt;
            }
        }
        for hero in &mut self.heroes {
            hero.cooldown -= dt;
            if hero.cooldown <= 0.0 {
                let hx = hero.gx as f32;
                let hy = hero.gy as f32;
                let mut best_idx = None;
                let mut min_dist = hero.range;
                for (idx, enemy) in self.enemies.iter().enumerate() {
                    let d = (enemy.gx - hx).hypot(enemy.gy - hy);
                    if d <= min_dist {
                        min_dist = d;
                        best_idx = Some(idx);
                    }
                }
                if let Some(idx) = best_idx {
                    hero.cooldown = hero.max_cd;
                    self.enemies[idx].hp -= hero.dmg;
                    self.projectiles.push(Projectile {
                        from_gx: hx,
                        from_gy: hy,
                        to_gx: self.enemies[idx].gx,
                        to_gy: self.enemies[idx].gy,
                        color: hero.color.clone(),
                        life: 0.14,
                    });
                    if self.enemies[idx].hp <= 0.0 {
                        let dead = self.enemies.remove(idx);
                        self.win += 1;
                        self.mana += dead.reward;
                        if self.win >= self.win_target {
                            self.status = MatchStatus::Victory;
                        }
                    }
                }
            }
        }
        self.projectiles.retain_mut(|p| {
            p.life -= dt;
            p.life > 0.0
        });
    }
}