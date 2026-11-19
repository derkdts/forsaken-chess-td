use rand::Rng;

pub const GRID_SIZE: usize = 9;

pub fn generate_map(stage: u32) -> (Vec<Vec<u8>>, Vec<(f32, f32)>) {
    let mut rng = rand::thread_rng();
    let mut grid = vec![vec![0u8; GRID_SIZE]; GRID_SIZE];
    let mut waypoints = Vec::new();

    let mut cx: usize = 1;
    let mut cy: usize = rng.gen_range(1..4);
    grid[cy][cx] = 3; // Красный портал (Spawn)
    waypoints.push((cx as f32, cy as f32));

    while cx < GRID_SIZE - 2 {
        let max_right = (GRID_SIZE - 2 - cx).min(2);
        let steps_right = rng.gen_range(1..=max_right);
        for _ in 0..steps_right {
            cx += 1;
            grid[cy][cx] = 1;
            waypoints.push((cx as f32, cy as f32));
        }

        if cx >= GRID_SIZE - 2 {
            break;
        }

        let mut candidates = Vec::new();
        if cy > 2 {
            candidates.push(cy - 2);
        }
        if cy < GRID_SIZE - 3 {
            candidates.push(cy + 2);
        }

        if !candidates.is_empty() {
            let target_y = candidates[rng.gen_range(0..candidates.len())];
            while cy != target_y {
                if target_y > cy {
                    cy += 1;
                } else {
                    cy -= 1;
                }
                grid[cy][cx] = 1;
                waypoints.push((cx as f32, cy as f32));
            }
        }
    }

    cx += 1;
    grid[cy][cx] = 4; // Фиолетовый портал (Exit)
    waypoints.push((cx as f32, cy as f32));

    let dirs: [(isize, isize); 8] = [
        (-1, 0), (1, 0), (0, -1), (0, 1),
        (-1, -1), (1, 1), (-1, 1), (1, -1),
    ];
    let density = (0.75 - (stage as f32) * 0.03).max(0.45);

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if grid[y][x] != 0 {
                continue;
            }
            let near_path = dirs.iter().any(|(dx, dy)| {
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0 && nx < GRID_SIZE as isize && ny >= 0 && ny < GRID_SIZE as isize {
                    grid[ny as usize][nx as usize] == 1
                } else {
                    false
                }
            });

            if near_path && rng.gen::<f32>() < density {
                grid[y][x] = 2; // Возвышенность для шахматной башни
            }
        }
    }

    (grid, waypoints)
}