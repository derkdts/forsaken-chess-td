import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
interface Hero {
  gx: number;
  gy: number;
  level: number;
  range: number;
  color: string;
  symbol: string;
  title: string;
}
interface Enemy {
  gx: number;
  gy: number;
  hp: number;
  max_hp: number;
}
interface Projectile {
  from_gx: number;
  from_gy: number;
  to_gx: number;
  to_gy: number;
  color: string;
}
interface GameSnapshot {
  stage: number;
  lost: number;
  max_lost: number;
  win: number;
  win_target: number;
  mana: number;
  speed: number;
  grid_size: number;
  grid: number[][];
  heroes: Hero[];
  enemies: Enemy[];
  projectiles: Projectile[];
  status: 'Playing' | 'Victory' | 'Defeat';
}
const canvas = document.getElementById('gameCanvas') as HTMLCanvasElement;
const ctx = canvas.getContext('2d')!;
const TILE_W = 86;
const TILE_H = 43;
const PLATFORM_H = 24;
let state: GameSnapshot | null = null;
let hoveredTile: { gx: number; gy: number } | null = null;
function resize() {
  canvas.width = window.innerWidth;
  canvas.height = window.innerHeight;
}
window.addEventListener('resize', resize);
resize();
function getGridOrigin(gridSize: number) {
  return {
    originX: canvas.width / 2,
    originY: canvas.height / 2 - (gridSize * TILE_H) / 2 + 40,
  };
}
function gridToScreen(gx: number, gy: number, gridSize: number) {
  const { originX, originY } = getGridOrigin(gridSize);
  return {
    x: originX + (gx - gy) * (TILE_W / 2),
    y: originY + (gx + gy) * (TILE_H / 2),
  };
}
function screenToGrid(sx: number, sy: number, gridSize: number) {
  const { originX, originY } = getGridOrigin(gridSize);
  const dx = sx - originX;
  const dy = sy - originY;
  const gx = Math.floor((dx / (TILE_W / 2) + dy / (TILE_H / 2)) / 2);
  const gy = Math.floor((dy / (TILE_H / 2) - dx / (TILE_W / 2)) / 2);
  return { gx, gy };
}
function drawIsoTile(cx: number, cy: number, top: string, left: string, right: string, h: number) {
  const hw = TILE_W / 2;
  const hh = TILE_H / 2;
  ctx.beginPath();
  ctx.moveTo(cx - hw, cy);
  ctx.lineTo(cx, cy + hh);
  ctx.lineTo(cx, cy + hh + h);
  ctx.lineTo(cx - hw, cy + h);
  ctx.closePath();
  ctx.fillStyle = left;
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(cx + hw, cy);
  ctx.lineTo(cx, cy + hh);
  ctx.lineTo(cx, cy + hh + h);
  ctx.lineTo(cx + hw, cy + h);
  ctx.closePath();
  ctx.fillStyle = right;
  ctx.fill();
  ctx.beginPath();
  ctx.moveTo(cx, cy - hh);
  ctx.lineTo(cx + hw, cy);
  ctx.lineTo(cx, cy + hh);
  ctx.lineTo(cx - hw, cy);
  ctx.closePath();
  ctx.fillStyle = top;
  ctx.fill();
  ctx.strokeStyle = 'rgba(0,0,0,0.28)';
  ctx.stroke();
}
function drawChessPiece(x: number, y: number, hero: Hero) {
  ctx.beginPath();
  ctx.ellipse(x, y + 6, 17, 8, 0, 0, Math.PI * 2);
  ctx.fillStyle = 'rgba(0,0,0,0.45)';
  ctx.fill();
  ctx.beginPath();
  ctx.ellipse(x, y + 2, 14, 7, 0, 0, Math.PI * 2);
  ctx.fillStyle = hero.color;
  ctx.fill();
  ctx.strokeStyle = '#fff';
  ctx.lineWidth = 1.5;
  ctx.stroke();
  ctx.font = 'bold 30px "Segoe UI Symbol", serif';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillStyle = '#111';
  ctx.fillText(hero.symbol, x + 1, y - 13);
  ctx.fillStyle = '#fff';
  ctx.fillText(hero.symbol, x, y - 14);
  ctx.fillStyle = '#ffd740';
  ctx.font = 'bold 11px sans-serif';
  ctx.fillText(`${'★'.repeat(hero.level)} ${hero.title}`, x, y - 36);
}
function render() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
  if (!state) {
    requestAnimationFrame(render);
    return;
  }
  const N = state.grid_size;
  for (let y = 0; y < N; y++) {
    for (let x = 0; x < N; x++) {
      const cell = state.grid[y][x];
      if (cell === 0) continue;
      const pos = gridToScreen(x, y, N);
      const isHovered = hoveredTile?.gx === x && hoveredTile?.gy === y;
      if (cell === 1 || cell === 3 || cell === 4) {
        drawIsoTile(pos.x, pos.y, '#689f38', '#33691e', '#558b2f', 12);
        if (cell === 3) {
          ctx.beginPath();
          ctx.ellipse(pos.x, pos.y, 20, 10, 0, 0, Math.PI * 2);
          ctx.fillStyle = 'rgba(255, 23, 68, 0.8)';
          ctx.fill();
        } else if (cell === 4) {
          ctx.beginPath();
          ctx.ellipse(pos.x, pos.y, 20, 10, 0, 0, Math.PI * 2);
          ctx.fillStyle = 'rgba(124, 77, 255, 0.8)';
          ctx.fill();
        }
      } else if (cell === 2) {
        const isDark = (x + y) % 2 === 1;
        const top = isHovered ? '#b0bec5' : isDark ? '#546e7a' : '#78909c';
        drawIsoTile(pos.x, pos.y - PLATFORM_H, top, '#263238', '#37474f', PLATFORM_H + 12);
      }
      const hero = state.heroes.find((h) => h.gx === x && h.gy === y);
      if (hero) {
        if (isHovered) {
          ctx.beginPath();
          ctx.ellipse(
            pos.x,
            pos.y - PLATFORM_H,
            hero.range * TILE_W * 0.5,
            hero.range * TILE_H * 0.5,
            0,
            0,
            Math.PI * 2
          );
          ctx.fillStyle = 'rgba(255, 215, 64, 0.14)';
          ctx.fill();
          ctx.strokeStyle = hero.color;
          ctx.stroke();
        }
        drawChessPiece(pos.x, pos.y - PLATFORM_H - 4, hero);
      }
    }
  }
  for (const e of state.enemies) {
    const pos = gridToScreen(e.gx, e.gy, N);
    ctx.beginPath();
    ctx.arc(pos.x, pos.y - 10, 9, 0, Math.PI * 2);
    ctx.fillStyle = '#ff3d00';
    ctx.fill();
    const ratio = Math.max(0, e.hp / e.max_hp);
    ctx.fillStyle = '#111';
    ctx.fillRect(pos.x - 14, pos.y - 25, 28, 4);
    ctx.fillStyle = '#00e676';
    ctx.fillRect(pos.x - 14, pos.y - 25, 28 * ratio, 4);
  }
  for (const p of state.projectiles) {
    const a = gridToScreen(p.from_gx, p.from_gy, N);
    const b = gridToScreen(p.to_gx, p.to_gy, N);
    ctx.beginPath();
    ctx.moveTo(a.x, a.y - PLATFORM_H - 14);
    ctx.lineTo(b.x, b.y - 10);
    ctx.strokeStyle = p.color;
    ctx.lineWidth = 3;
    ctx.stroke();
  }
  requestAnimationFrame(render);
}
function syncUI(snap: GameSnapshot) {
  state = snap;
  document.getElementById('ui-stage')!.textContent = String(snap.stage);
  document.getElementById('ui-lost')!.textContent = `${snap.lost} / ${snap.max_lost}`;
  document.getElementById('ui-win')!.textContent = `${snap.win} / ${snap.win_target}`;
  document.getElementById('ui-mana')!.textContent = String(snap.mana);
  const overlay = document.getElementById('overlay')!;
  if (snap.status === 'Playing') {
    overlay.style.display = 'none';
  } else {
    overlay.style.display = 'flex';
    const title = document.getElementById('overlay-title')!;
    const desc = document.getElementById('overlay-desc')!;
    const btn = document.getElementById('btn-next')!;
    if (snap.status === 'Victory') {
      title.textContent = `ЭТАП ${snap.stage} ПРОЙДЕН!`;
      title.style.color = '#ffd740';
      desc.textContent = 'Процедурный генератор построит новую карту.';
      btn.textContent = `Этап ${snap.stage + 1} ➔`;
      btn.onclick = () => invoke('next_stage', { stage: snap.stage + 1 });
    } else {
      title.textContent = 'ПОРАЖЕНИЕ';
      title.style.color = '#ff5252';
      desc.textContent = 'Враги прорвали оборону. Попробуйте другую расстановку!';
      btn.textContent = 'Переиграть этап ↻';
      btn.onclick = () => invoke('next_stage', { stage: snap.stage });
    }
  }
}
canvas.addEventListener('mousemove', (e) => {
  if (!state) return;
  let cell = screenToGrid(e.clientX, e.clientY + PLATFORM_H, state.grid_size);
  if (state.grid[cell.gy]?.[cell.gx] !== 2) {
    cell = screenToGrid(e.clientX, e.clientY, state.grid_size);
  }
  hoveredTile = cell;
});
canvas.addEventListener('click', () => {
  if (hoveredTile) {
    invoke('summon_or_upgrade', { gx: hoveredTile.gx, gy: hoveredTile.gy });
  }
});
document.getElementById('btn-summon')!.addEventListener('click', () => {
  invoke('summon_or_upgrade', { gx: null, gy: null });
});
document.getElementById('btn-regen')!.addEventListener('click', () => {
  if (state) invoke('next_stage', { stage: state.stage });
});
document.getElementById('btn-speed')!.addEventListener('click', async (e) => {
  const newSpeed = await invoke<number>('toggle_speed');
  const btn = e.target as HTMLButtonElement;
  btn.textContent = `⏩ x${newSpeed}`;
  btn.classList.toggle('active', newSpeed === 2);
});
listen<GameSnapshot>('game_state', (event) => syncUI(event.payload));
requestAnimationFrame(render);