# Battle Sim

## Features

* Random character generation
* Automatic turn-based combat simulation with a log of every turn
* Spell system
* Battle results saved to a PostgreSQL database
* Battle history with pagination, sorting, and search
* Detail view and replay of individual battles
* API request rate limiting
* Docker containerization with separate dev and prod environments

## Game Rules

Each battle takes place between two randomly generated characters, who alternate taking attack turns.

**Turn sequence:**

* The attacker deals damage equal to the difference between their strength and the opponent's effective defense (minimum 0)
* With a given probability, the attacker also casts their spell
* The battle ends when one character's HP drops to zero

### Spells

| Spell | Effect |
|---|---|
| **Hard Hit** | Randomly deals between 5-10 damage
| **Ice Bullet** | Deals 8 damage
| **Shield** | Permanently raises defense randomly by 1-3
| **Aura** | Raises defense by 5 for the next turn
| **Critical Hit** | 1/10 chance of landing a critical that ends the fight
| **Nature Voice** | 1/10 chance of restoring full HP
| **Rage** | 1/5 chance for the attacker to attack next turn
| **Reincarnation** | Randomly restores between 5-10 HP (cannot exceed max HP)

Every battle is saved along with a full turn-by-turn log, so it can later be replayed in the history view.

## Tech Stack

**Backend**

* Rust 2024
* Axum - HTTP framework
* SQLx - PostgreSQL access
* PostgreSQL 18
* tower-governor - rate limiting

**Frontend**

* Svelte 5
* Vite
* Tailwind CSS 4
* DataTables.net 3

## Running the Project

### Production version

```bash
docker compose -f docker-compose.prod.yml up --build
```

The frontend will be available at `http://localhost` (the backend is exposed internally and accessible via `/api/`).

### Development version

```bash
docker compose -f docker-compose.dev.yml up --build
```

The backend runs with hot-reload (`cargo watch`), and frontend runs with the Vite dev server:

* Frontend: `http://localhost:5173`
* Backend API: `http://localhost:3000`
* PostgreSQL: `localhost:5432`

### [!] Database
The default database credentials in `docker-compose.prod.yml` and `docker-compose.dev.yml` files (`POSTGRES_USER`, `POSTGRES_PASSWORD`) are
sample values and **must be changed before deploying to production** (ideally passed via an `.env` file or secrets).

## License

The project is available under the **MIT** license - see the [LICENSE](./LICENSE) file for details.
