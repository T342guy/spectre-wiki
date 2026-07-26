# What is the spectre wiki project?
The spectre wiki project is a software engineering project aiming for next-generation, fast, advanced and modular wiki technology.
## Tech stack
- Main language - Rust
- UI - Leptos: in SSR mode
- Database - spacetimeDB - first supported tech, add more with [[#Database stacks]].
- web framework - Axum
### Quality of life
- Easy to setup podman/docker containers
- Interactive setup
	- Advanced and simplified system modes (for power users and novices)
- Advanced admin panel 
## framework

# Feature list
## Fundamental features
(Features that have already existed for a long time in other wiki tech and are essential)
- **(fundamental feature)** rule based Access control - server level
	- fine grained control (edit, read, conflict resolution perms, ext.)
- **(fundamental feature)** edit difference history, aka diff history
- **(fundamental feature)** conflict resolution
## Engine features
- Real time collaborative editing
- High-level modular plugin system
 - version-safe updating (when the API changes)
    - config files outline version names, and update accordingly so they work seamlessly in newer versions of spectre
### Uncertain features
- Multi-database compatibility
## Client features
- Full image + audio streaming system
    - optional caching client option (uses temp files)
    - optional quality limiter (for streaming and server sided storage concerns)
    - quality downgrading for low-bandwith clients
- universal banner system (to be used by plugins and system).
	- Banner system can change the body color, add an icon, change size and add links. 
- Full markdown support using `pulldown-cmark`.
- GitHub repository connection with automatic issue linking. Used for documenting fixes for issues.
- WYSIWYG live rendering markdown editor
- full web client style customization
     - upon setup, ask if they want to pre-install and pre-enable or not.
     - add cool ones like carbon fiber, galaxy, and Y2K, stuff like that
- Google maps object with coordinate and/or address
- standard markdown embed support
## Plugin system

# Development plan 
## Stage 1 - Alpha/prototype x.x.1
- Use basic database like turso (rust-rewrite of sqlite)
- establish basic slug editing
- establish reliable page creation and editing
- establish full edit history - major / minor / drafts
- establish server sided role based access control
- SSR rendering when a page is saved
- basic UI
- add conflict resolution (automatic conditions like newest or most lines, or only allow manual conflict resolution)
- role based access control
## Stage 2 - Beta 1 x.1.x
- integrate spacetimeDB usage
- add image + video storage and streaming
- add basic WYSIWYG markdown editor
- 