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

Production is meant to run on a VPS, with certificates issued via Let's Encrypt (pre-configured to use [deSEC](https://desec.io/) as the DNS provider).

Before the first run:

1. Copy `docker/certbot/secrets/desec.ini.example` to `docker/certbot/secrets/desec.ini` and put your deSEC API token there.
2. Copy `.env.example` to `.env` and set `DOMAIN` to your actual domain (plus `DB_USER`, `DB_PASSWORD`, `DB_NAME`).
3. Run `chmod +x init-cert.sh` and then `./init-cert.sh` to issue the initial certificate.

Skipping any of these will make nginx keep restarting in a loop instead of starting.

```bash
docker compose -f docker-compose.prod.yml up --build
```

The frontend will be available at `https://<DOMAIN>` (the backend is exposed internally and accessible via `/api/`).

### Development version

```bash
docker compose -f docker-compose.dev.yml up --build
```

The backend runs with hot-reload (`cargo watch`), and frontend runs with the Vite dev server:

* Frontend: `http://localhost:5173`
* Backend API: `http://localhost:3000`
* PostgreSQL: `localhost:5432`

### [!] Database & secrets
The default values in `.env.example` (`DB_USER`, `DB_PASSWORD`, `DB_NAME`, `DOMAIN`) and in `docker/certbot/secrets/desec.ini.example` are sample values and **must be changed before deploying to production**. Copy each `.example` file, remove the `.example` suffix, and fill in real values.

## License

The project is available under the **MIT** license - see the [LICENSE](./LICENSE) file for details.
