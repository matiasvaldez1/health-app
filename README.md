# Health Tracker

App local de seguimiento de salud para macOS, con acceso desde el navegador.

## Funcionalidades

- **Registro de peso** — seguimiento diario con gráficos de tendencia
- **Registro de meditación** — sesiones con fecha, hora, duración y calendario mensual
- **Diario de estado de ánimo** — entradas de texto libre con puntaje de ánimo (1-10)
- **Tips de salud con IA** — tips personalizados generados por Claude basados en tus datos
- **Dashboard** — resumen con métricas, tendencias, racha y acciones rápidas
- **Meta de peso** — configurá un objetivo y seguí tu progreso
- **Timer de meditación** — cronómetro integrado que autocompleta la duración
- **Exportar datos** — descargá tus datos en JSON o CSV

## Stack

- **Desktop**: [Tauri v2](https://v2.tauri.app/) (Rust)
- **Frontend**: [Svelte 5](https://svelte.dev/) + [Tailwind CSS v4](https://tailwindcss.com/)
- **Base de datos**: SQLite (local, WAL mode)
- **Servidor HTTP**: [Axum](https://github.com/tokio-rs/axum) (embebido, sirve frontend + API REST en `:3333`)
- **IA**: API de Claude (Anthropic) para tips de salud

## Requisitos

- Node.js 20+ (usar `nvm use 20`)
- Rust (instalar via [rustup](https://rustup.rs/))
- pnpm (`npm install -g pnpm`)
- Xcode Command Line Tools (`xcode-select --install`)

## Setup

```bash
git clone git@github-personal:matiasvaldez1/health-app.git
cd health-app
nvm use 20
pnpm install
```

## Desarrollo

```bash
pnpm tauri dev
```

Esto abre la app nativa + el servidor HTTP en `http://localhost:3333`.

## Build

```bash
pnpm tauri build
```

Genera:
- `src-tauri/target/release/bundle/macos/Health Tracker.app`
- `src-tauri/target/release/bundle/dmg/Health Tracker_0.1.0_aarch64.dmg`

Para instalar: copiar el `.app` a `/Applications/`.

## Arquitectura

```
Tauri v2 (ventana nativa macOS)
├── Svelte 5 Frontend (SPA)
│   └── Comunicación via Tauri IPC (desktop) o HTTP fetch (browser)
│
└── Rust Backend
    ├── Tauri IPC commands
    ├── Axum HTTP server (:3333) ← acceso desde el navegador
    ├── SQLite (compartido, modo WAL)
    └── Cliente API de Claude (reqwest)
```

Desktop y navegador comparten la misma base de datos — no necesita sincronización.

## Datos

Los datos se guardan localmente en:

```
~/Library/Application Support/com.matiasvaldez.health-tracker/health-tracker.db
```

La clave de API se guarda en:

```
~/Library/Application Support/com.matiasvaldez.health-tracker/config.json
```

## Estructura del proyecto

```
health-tracker/
├── src/                      # Frontend (Svelte 5)
│   ├── lib/
│   │   ├── api.ts            # Capa API unificada (Tauri IPC vs HTTP)
│   │   ├── types.ts          # Interfaces TypeScript
│   │   └── utils.ts          # Helpers de fecha/hora
│   └── routes/
│       ├── +page.svelte      # Dashboard
│       ├── weight/            # Registro de peso
│       ├── meditation/        # Registro de meditación
│       ├── feelings/          # Diario de estado de ánimo
│       ├── tips/              # Tips de salud con IA
│       └── settings/          # Configuración
├── src-tauri/                # Backend (Rust)
│   ├── src/
│   │   ├── lib.rs            # Entry point
│   │   ├── db.rs             # Pool SQLite + migraciones
│   │   ├── models.rs         # Structs y DTOs
│   │   ├── services.rs       # Lógica de negocio (CRUD)
│   │   ├── commands.rs       # Comandos Tauri IPC
│   │   ├── handlers.rs       # Handlers REST (Axum)
│   │   ├── server.rs         # Router y servidor HTTP
│   │   └── claude.rs         # Integración API de Claude
│   └── migrations/
│       └── 001_init.sql      # Schema de la base de datos
└── README.md
```
