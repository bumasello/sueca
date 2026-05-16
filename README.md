# Sueca Online

Implementação online do jogo de cartas português **Sueca** — um jogo de vazas para 4 jogadores com naipe triunfo, divididos em duas duplas.

## Sobre o jogo

A Sueca é jogada com 40 cartas (baralho sem 8, 9 e 10), em duplas de 2. O naipe triunfo é sorteado no início de cada partida. A dupla que acumular mais de 60 pontos em vazas vence a rodada.

**Pontuação das cartas:**
| Carta | Pontos |
|-------|--------|
| Ás | 11 |
| 7 | 10 |
| Rei | 4 |
| Valete | 3 |
| Dama | 2 |
| Demais | 0 |

## Stack

- **Backend** — [Axum](https://github.com/tokio-rs/axum) + MongoDB Atlas
- **Frontend** — [Yew](https://yew.rs) (WebAssembly) + Tailwind CSS
- **Comum** — lógica do jogo compartilhada entre backend e frontend

## Estrutura

```
sueca/
├── common/     # lógica do jogo: cartas, naipes, distribuição, resolução de vazas
├── backend/    # servidor HTTP: autenticação, salas, fluxo da partida
└── frontend/   # interface web em WebAssembly
```

## Rodando localmente

### Pré-requisitos

- Rust (stable)
- `wasm32-unknown-unknown` target: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`
- MongoDB Atlas (crie um cluster gratuito em [mongodb.com](https://mongodb.com))

### Configuração

Crie um arquivo `.env` na raiz do projeto:

```env
MONGODB_URI=sua_connection_string
MONGODB_DB=sueca
MONGODB_COLLECTION=users
```

### Backend

```bash
cargo run -p backend
# Servidor disponível em http://localhost:3000
```

### Frontend

```bash
cd frontend
API_URL=http://localhost:3000 trunk serve
# Interface disponível em http://localhost:8080
```

> Use `http://localhost:8080` (não `127.0.0.1`) para que os cookies funcionem corretamente.

## Como jogar

1. Acesse a aplicação e crie um usuário
2. No lobby, crie ou entre em uma sala
3. A partida começa automaticamente quando 4 jogadores entrarem
4. Jogue suas cartas clicando nelas — o jogo enforça a regra de seguir o naipe puxado
5. A dupla com mais de 60 pontos vence

## Desenvolvimento

```bash
# Build completo
cargo build

# Testes
cargo test

# Lint
cargo clippy

# Formatação
cargo fmt
```
