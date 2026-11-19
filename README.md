# crowdsec-local-ui

Lightweight single-binary local dashboard for CrowdSec (Rust + Axum + Preact, embedded via `rust-embed`).

## Data Schema (`crowdsec.db` Read-Only WAL)
- `decisions`: `id`, `alert_id`, `origin`, `type`, `scope`, `value`, `scenario`, `until`, `created_at`
- `alerts`: `id`, `scenario`, `source_ip`, `source_scope`, `events_count`, `started_at`, `stopped_at`

## Local API Endpoints
- `GET /api/stats` — Active bans, top triggered scenarios, and 24h alert summary
- `GET /api/decisions` — Paginated active decisions with IP/origin search
- `POST /api/decisions` — Create manual IP/range ban via local LAPI (`127.0.0.1:8080/v1/decisions`)
- `DELETE /api/decisions/:id` — Revoke ban decision via local LAPI
- `POST /api/false-positive` — Anonymize raw log sample and open a GitHub issue for automated Grok parser reproduction
