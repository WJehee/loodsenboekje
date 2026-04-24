# Authelia / LDAP integration

Tracks: [#55](https://github.com/WJehee/loodsenboekje/issues/55)

## Goal

Replace (or augment) the local username/password flow with an SSO login backed
by [Authelia](https://www.authelia.com/). Authelia already speaks LDAP to an
upstream directory, so the Loodsenboekje app itself does not need to talk to
LDAP directly — the reverse proxy authenticates the request and forwards the
identity as trusted headers.

## Why Authelia + trusted headers (not OIDC, not LDAP direct)

- **Authelia-in-front / trusted headers** — tiny code change (read a few
  headers), reuses the reverse proxy we already have, single sign-on across
  apps. Trade-off: only safe behind a proxy that strips client-supplied
  `Remote-*` headers.
- **OIDC inside the app** — would require adding `openidconnect`, handling
  redirects, storing tokens; more code for no benefit when Authelia already
  sits in front.
- **LDAP inside the app** — pointless given Authelia already binds to LDAP.
  Would just reimplement half of Authelia.

## Scope vs. PocketBase

The issue also raises [PocketBase](https://pocketbase.io/) as an alternative —
replace the custom Rust backend entirely with a shared self-hosted backend
serving multiple apps. That is a much larger migration (different data model,
different client, different deployment) and is **out of scope** here. The
Authelia work below is additive and gated behind an env var, so it does not
foreclose a later PocketBase move.

## Headers the app will read

Standard Authelia forward-auth headers:

| Header          | Meaning                                         |
| --------------- | ----------------------------------------------- |
| `Remote-User`   | Unique username (stable identifier)             |
| `Remote-Name`   | Display name                                    |
| `Remote-Email`  | Email address                                   |
| `Remote-Groups` | Comma-separated list of groups                  |

Group → role mapping (configurable via env):

| `UserType` | Env var (default)                                        |
| ---------- | -------------------------------------------------------- |
| `Admin`    | `AUTHELIA_GROUP_ADMIN`  (default: `loodsenboekje-admin`) |
| `Writer`   | `AUTHELIA_GROUP_WRITER` (default: `loodsenboekje-writer`)|
| `Reader`   | `AUTHELIA_GROUP_READER` (default: `loodsenboekje-reader`)|

If a user has no matching group, they are treated as `Inactive`.

## Configuration

New env vars:

- `AUTH_MODE` — `local` (default, existing behavior) or `authelia`.
- `AUTHELIA_GROUP_ADMIN` / `_WRITER` / `_READER` — group names to map.
- `READ_PASSWORD` / `WRITE_PASSWORD` / `ADMIN_PASSWORD` — only required when
  `AUTH_MODE=local`.

The NixOS module exposes `services.loodsenboekje.auth.mode` and the group
option set; see `nix/module.nix`.

## Runtime behavior in `authelia` mode

1. Every request carries `Remote-User` (guaranteed by Authelia). The app's
   `user()` helper:
   - Reads `Remote-User`, `Remote-Groups`.
   - Looks up the user in the `users` table by username; if missing, inserts a
     new row (empty password, `user_type` derived from the group mapping).
   - On every request, re-derives `user_type` from `Remote-Groups` so that
     group changes in Authelia propagate immediately without re-login.
2. `/login` and `/register` pages are hidden; the `Login` / `Register` /
   `Logout` server functions become no-ops that redirect to `/`. Logout is
   delegated to Authelia's own `/logout` endpoint (admins configure their
   reverse proxy to expose it).
3. The in-memory session store stays as a cheap per-request cache but is no
   longer the source of truth.

## Reverse-proxy setup (nginx example)

```nginx
# Authelia forward-auth endpoint
location = /authelia {
    internal;
    proxy_pass http://authelia:9091/api/verify;
    proxy_pass_request_body off;
    proxy_set_header Content-Length "";
    proxy_set_header X-Original-URL $scheme://$http_host$request_uri;
    proxy_set_header X-Forwarded-Method $request_method;
}

server {
    server_name loodsen.example.org;

    # CRITICAL: strip any client-supplied Remote-* headers before forwarding
    proxy_set_header Remote-User   "";
    proxy_set_header Remote-Groups "";
    proxy_set_header Remote-Name   "";
    proxy_set_header Remote-Email  "";

    location / {
        auth_request /authelia;

        # Copy Authelia's response headers onto the upstream request
        auth_request_set $user   $upstream_http_remote_user;
        auth_request_set $groups $upstream_http_remote_groups;
        auth_request_set $name   $upstream_http_remote_name;
        auth_request_set $email  $upstream_http_remote_email;
        proxy_set_header Remote-User   $user;
        proxy_set_header Remote-Groups $groups;
        proxy_set_header Remote-Name   $name;
        proxy_set_header Remote-Email  $email;

        # On 401 from Authelia, redirect to its portal
        error_page 401 = @authelia_redirect;

        proxy_pass http://127.0.0.1:1744;
    }

    location @authelia_redirect {
        return 302 https://auth.example.org/?rd=$scheme://$http_host$request_uri;
    }
}
```

## Security notes

- **The app must not be directly reachable.** Trusted-header auth is only
  secure when requests go through the reverse proxy. Bind the app to
  `127.0.0.1` on the host, or isolate it on an internal network.
- **The proxy must strip incoming `Remote-*` headers** before `auth_request`
  runs — otherwise a client can impersonate anyone by sending their own
  `Remote-User`. The example config above does this.
- Consider adding a shared-secret header (e.g. `X-Auth-Secret`) as defense in
  depth if the proxy config cannot be trusted. Not implemented in the initial
  PR.

## Migration plan

1. **This PR** — scaffolding behind `AUTH_MODE`. Local auth is still the
   default; opt in with `AUTH_MODE=authelia`.
2. Deploy to staging with Authelia, verify group mapping and auto-provisioning.
3. Flip production to `AUTH_MODE=authelia`; keep `local` available for a
   release or two as escape hatch.
4. **Follow-up PR** — remove local auth, make `password` column nullable,
   delete `/register` entirely, drop `bcrypt` and the registration passwords.
5. **Separate issue** — evaluate PocketBase as a shared backend for future
   apps (tracks the other half of #55).

## Out of scope

- Direct LDAP binding inside the Rust app.
- OIDC-as-client inside the Rust app.
- Automatic group creation in Authelia.
- PocketBase migration.
