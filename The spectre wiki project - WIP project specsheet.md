# What is the spectre wiki project?
The spectre wiki project is a software engineering project aiming for next-generation, fast, advanced and modular wiki technology.

> [!WARNING]
> Spectre wiki, and this specsheet is not complete and may change.

## Tech stack
- Main language - Rust
- Database - `Turso`
- Web framework - `Axum`
- logging - `Tracing` - https://crates.io/crates/tracing
- Modules sandboxing - WASMtime - https://github.com/bytecodealliance/wasmtime
- Password hashing - `argon2`
- Media handling - `ffmpeg` (subprocess) / `symphonia`, served via `tower-http` range requests
- Real-time collab (future) - `yrs` (Yjs port), if/when CRDT mode is added
- uses **AGPL 3.0**

> [!NOTE]
> **Why AGPLv3 instead of GPLv3:** GPLv3's copyleft obligations trigger on *distribution* of the binary. Since a core project goal is running a nonprofit Wiki-as-a-Service, GPLv3 would let a competing host fork Spectre, modify it, and never share those changes back — they're never "distributing" anything, just running a service. AGPLv3 closes this by requiring source disclosure to users interacting with a modified version over a network. This protects the commons from exactly the kind of competitor most likely to appear, at the cost of some corporate adopters with blanket "no AGPL" policies avoiding self-hosted deployment.

# Project key goals
1. Create a modern wiki technology
2. Make this technology extremely fast
3. Make it modular via modules
4. Make it safe by default
5. Make it advanced, but dumbed down for casual users

## Additional goals
- Make spectre wiki into a Wiki-as-a-service provider via a non-profit, only paying for specialty hosting for a small fee.
- Reinvest all money from hosting back into development of the project.

# Feature list
- **(fundamental feature)** rule based Access control - server, namespace, and page level
	- fine-grained control (edit, read, conflict resolution perms, etc.)
	- permissions table includes a `scope` column (`server` / `namespace` / `page`) from Stage 1, even if only `server`-scoped rows are written initially — avoids a breaking migration when granular scopes are added later
- **(fundamental feature)** edit difference history, aka diff history
- **(fundamental feature)** conflict resolution — see [[#Conflict resolution]]
- ([[#Module system]]) High-level modular plugin system
- Image + audio playback support
    - integrates with existing tooling (`ffmpeg`/`symphonia`) rather than building codec/transcoding infrastructure in-house
    - optional caching client option (uses temp files)
    - optional pre-transcoded quality tiers at upload time, selected at serve time based on client hints (not full adaptive bitrate streaming)
    - HTTP range request support for scrubbing, via `tower-http`
- universal banner system (to be used by plugins and system)
	- Banner system can change the body color, add an icon, change size and add links
- Full markdown support using `pulldown-cmark`
- GitHub repository connection with automatic issue linking. Used for documenting fixes for issues.
- WYSIWYG live rendering markdown editor
- full web client style customization
     - upon setup, ask if they want to pre-install and pre-enable or not
     - add cool styles like carbon fiber, galaxy, and Y2K, stuff like that
- Google maps object with coordinate and/or address
- standard markdown embed support
- use WASMtime to sandbox WASM plugins for security and control
- (QoL) Easy to setup podman/docker containers
- (QoL) Interactive setup
	- Advanced and simplified system modes (for power users and novices)
- Advanced admin panel
- Full-Text Search (FTS) using turso's FTS5 module
- one-click structured markdown export of the entire wiki — also doubles as the backup and GDPR/PIPEDA data-export mechanism (see [[#Data handling and compliance]])
- Authentication system — see [[#Authentication]]

## Conflict resolution
Deferred to the Beta stage, but the storage model is chosen up front so Stage 1 doesn't need to be redesigned later.

- **Stage 1/2 approach: three-way merge** (MediaWiki-style). Uses the last-common-ancestor revision plus both diverging edits, run through a diff3-style merge algorithm. This fits naturally on top of the diff-history feature already planned — revisions are stored either way, so this is mostly a merge algorithm layered on existing data.
- **Considered for later: CRDTs** (e.g. via `yrs`) for real-time, Google-Docs-style simultaneous editing. This is a much larger commitment: it changes the storage format (CRDT ops/state rather than markdown snapshots), requires the WYSIWYG editor to speak the CRDT protocol, and needs a sync layer. Not required for a functional wiki (Wikipedia itself has no real-time collab) — treated as an optional Stage 3+ mode, not a prerequisite.

## Authentication
- **Local accounts** (username/email + password) as the always-available baseline, hashed with `argon2`. Kept in core rather than delegated to modules, since auth is too security-critical to fully hand to third-party WASM code by default.
- **OIDC/OAuth2 (GitHub, Google, etc.) as pluggable modules**, not baked into core. Each provider is a module that declares its required permissions (session creation, profile read) through the existing module declaration/permission system — consistent with how other modules work.
- **Sessions are server-side**, stored in Turso, rather than self-contained JWTs — allows instant revocation (banned users, compromised accounts) without needing a separate revocation-list system bolted on afterward.

## Module system
When creating plugins or modules, we need to create a fast and secure system.

Modules will be compiled in WASM, and run via a WASMtime process. This allows Spectre to control and directly manage its modules without exposure to the host's device or container.

```mermaid
flowchart LR

n1["Engine"] --> n2["WASMtime process"]

n2 --> n3["WASM modules"]

  

n1@{ shape: proc}

n2@{ shape: subproc}

n3@{ shape: procs}
```

This system maintains control over performance, run states with hot-restart capability, and allows potentially malicious code to be completely isolated.

### Installation
These WASM plugins will be uploaded to the server via spectre's dashboard, or can be pasted into the instance's module's folder, although that may not be recommended.

### Module development
To develop these plugins, we will develop a faux-API program that will allow the developer to manually trigger API messages in a simulated environment the same as spectre's WASMtime sandboxing.

### API
Spectre uses an **internal REST-shaped host API**, not a networked REST API. Host functions are exposed to the WASM guest via WASM's normal import/export ABI (e.g. bound through `wasmtime::Linker`), mimicking REST semantics — verb + resource path + JSON payload/response, passed through shared linear memory — without opening any actual socket inside the sandbox. This gives module developers a familiar request/response mental model while keeping each "endpoint" individually permission-gated as a host function call, with no real network attack surface from within the sandbox.

### Security
Upon startup, spectre will ping a module for its "module declaration." This declaration will contain the module's name, description, version, required permissions, and other details.

Spectre will have permissions for each of its API calls, but for API calls to function, the module is required to announce its permissions. This "permissionless unless required" system will allow the user to see any module's capabilities.

### API versioning
When changes are needed to the module API, add the new API commands and keep the legacy commands until a major version. Once the major update is ready, remove all legacy API commands.

# Current systems
## Serving pages
In spectre, we use templates to serve pages. These templates are served alongside a context package that is rendered into the HTML. These context packages are insertable bits of data, for example; The page's HTML content fetched from the database.

```mermaid
flowchart LR

n1["Page template"] --> n2["client"]
n3["Context data, aka page contents"] --> n1
```

### Data holding
The page data will be saved in 2 formats; markdown, and HTML. The raw data will be handled by the `pulldown-cmark` parser, then rendered into HTML data that is saved in the database. This HTML data is what will be served in the context data when serving pages.

### Database consistency notes
Turso (libSQL) is SQLite-based with primary/replica embedded replicas; writes typically go to one primary and replicas may lag. If multi-region reads are used for speed, this needs to be reconciled with conflict resolution and edit-history correctness — flagged here as an open design question, not yet resolved.

# Data handling and compliance
Relevant specifically to the nonprofit Wiki-as-a-Service hosting offering (not self-hosted instances, where the instance admin is the data controller).

- **Content moderation / CSAM scanning**: required by law in most jurisdictions once third-party image/audio uploads are hosted at scale, regardless of nonprofit status. Plan to integrate automated hash-matching against known-CSAM databases at upload time (e.g. PhotoDNA or Thorn's Safer, both offer free access for qualifying nonprofits/NGOs) as a baseline, plus a publicly documented abuse/DMCA takedown process before launch.
- **Backups & data portability**: the one-click structured markdown export feature doubles as the backup and data-export mechanism rather than building a second system. Plan for automated periodic backups (self-hosted: to the instance admin's own storage; hosted: per documented retention policy).
- **Data deletion**: hosted users need a documented, user-triggerable "export and delete my data" flow to meet GDPR/PIPEDA-type timely-deletion requirements.
- A full Trust & Safety / data policy document is out of scope for this technical specsheet but should exist before the hosting service launches.

# Development plan
## Stage 1 - Alpha/prototype 0.1.0-Alpha.XX
- Integrate turso
- establish routers
- establish basic slug editing
- establish page creation and editing
- establish full edit history - major / minor / drafts
- establish server sided role based access control (schema includes scope column for future namespace/page-level ACL)
- establish local authentication (argon2 + server-side sessions)
- create basic UI

## Stage 2 - Beta 0.1.0-Beta.XX
- add image + video storage and playback (ffmpeg/symphonia integration, range-request serving)
- add basic WYSIWYG markdown editor
- add three-way merge conflict resolution
- add OIDC/OAuth2 module(s) for third-party login

## Stage 3+ / Future
- namespace and page-level ACL scopes activated
- CRDT-based real-time collaborative editing (optional mode)
- Trust & Safety tooling (CSAM hash-matching, moderation workflows) ahead of public hosting launch

# Development notes
## Modules API changes
When we need to make changes to the module API, add the new API commands and keep the legacy commands UNTIL a major version. Once the major update is ready, we remove all of the legacy API commands.
