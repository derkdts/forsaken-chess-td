```markdown
# Forsaken Chess (Tower Defense)

Изометрическая игра в жанре Tower Defense на базе **Tauri v2**, **Rust** и **TypeScript**. Защищайте линию обороны с помощью шахматных фигур, которые эволюционируют от Пешки до Ферзя на процедурно генерируемых картах.

## Возможности
- **Процедурная генерация уровней:** каждый этап создает уникальный маршрут и возвышенности для башен.
- **Система эволюции:** призывайте фигуры за энергию и улучшайте их по цепочке `Пешка ➔ Конь ➔ Слон ➔ Ладья ➔ Ферзь`.
- **Управление скоростью:** переключение темпа игры (`x1` / `x2`) в реальном времени.
- **Нативная производительность:** игровая логика и тики симуляции работают в асинхронном потоке Rust.

## Установка и запуск

### Требования
- [Node.js](https://nodejs.org/) (v20+)
- [Rust & Cargo](https://www.rust-lang.org/) (stable)
- Системные зависимости Tauri v2 (для Linux: `libwebkit2gtk-4.1-dev`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`)

### Локальная разработка
```bash
npm install
npm run tauri dev
```

### Сборка релиза
```bash
npm run tauri build
```

---

# Forsaken Chess (Tower Defense) (English)

An isometric Tower Defense game built with **Tauri v2**, **Rust**, and **TypeScript**. Defend your lane using chess pieces that evolve from a Pawn to a Queen across procedurally generated maps.

## Features
- **Procedural Map Generation:** Every stage generates a unique enemy path and elevated platforms for towers.
- **Evolution System:** Spend energy to summon pieces and upgrade them through `Pawn ➔ Knight ➔ Bishop ➔ Rook ➔ Queen`.
- **Speed Controls:** Toggle game simulation speed (`x1` / `x2`) on the fly.
- **Native Performance:** Core game loop and physics ticks run asynchronously in Rust.

## Setup & Running

### Prerequisites
- [Node.js](https://nodejs.org/) (v20+)
- [Rust & Cargo](https://www.rust-lang.org/) (stable)
- Tauri v2 system dependencies (on Linux: `libwebkit2gtk-4.1-dev`, `libxdo-dev`, `libssl-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`)

### Local Development
```bash
npm install
npm run tauri dev
```

### Production Build
```bash
npm run tauri build
```
```