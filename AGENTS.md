> **Purpose**: Defines how AI agents must operate in this repo.
> Be terse. Deliver code, not essays.

---

## 🧠 Core Principles

* Dioxus 0.7 only — no `cx`, `Scope`, `use_state`
* Components return `Element`
* State via `Signal`
* RSX for UI templates

```rust
#[component]
fn Hello() -> Element {
    rsx!( div { "Hello, world!" } )
}
```

```rust
#[component]
fn Counter() -> Element {
    let mut count = use_signal(|| 0);
    rsx!( button {
        onclick: move |_| *count.write() += 1,
        "Count: {count}"
    })
}
```

---

## 💬 Communication Rules

* **Be terse** — code > talk
* **Explain only when asked**
* **STOP command**

  * When told `STOP`: cease all output & edits, await next instruction

---

## 🧱 Project Packages

### `ui/` — Shared Components

* Cross-platform UI + client logic
* Contains components, hooks (`use_auth`), services (`auth_service.rs`)
* No platform deps
* Env flags: `api-dev`, `api-staging`, `api-prod`

### `web/` — Browser App

* Entry: `main.rs` → `dioxus::launch(App)`
* Web routes, Tailwind via `asset!`
* Handles OIDC callback in `use_effect`

### `desktop/` — Desktop App

* Entry: `main.rs`, minimal routes
* Uses `dioxus/desktop`

### `mobile/` — Mobile App

* Same as desktop; uses `dioxus/mobile`

### `api/main/` — Cloudflare Worker API

* Independent backend (no Dioxus server funcs)
* Uses `worker-rs`, Cloudflare D1 (SQLite)
* Auth: OIDC + JWT + password
* Folders:

  * `endpoints/` — routes
  * `services/` — logic
  * `common/` — utils

### `tailwind/` — CSS Build System

* Watches `ui/` + `web/` for classes
* Outputs to `web/assets/tailwind.css`
* Dark mode: `class`

**Dependency flow:**
`web | desktop | mobile → ui → (HTTP) → api/main`

---

## 📐 Spacing System

Standard spacing values for consistent layout:

### Standard Spacing Values

* **Container padding**: `px-4 sm:px-6 lg:px-8 py-6` (16px/24px/32px horizontal, 24px vertical)
* **Section margins**: `mb-6` (24px) for major sections
* **Card padding**: `p-5` (20px)
* **Form spacing**: `space-y-4` (16px)
* **List item spacing**: `py-4` (16px)
* **Small gaps**: `gap-3` (12px)
* **Medium gaps**: `gap-4` (16px)
* **Sidebar padding**: `px-6 py-6` (24px all around)
* **Main content padding**: `py-6` (24px top/bottom)

### Guidelines

* Never use `py-8` or `py-10` for main content containers
* Use `mb-6` instead of `mb-8` for section spacing
* Ensure consistent padding on all sides (use `py-6` not `pb-4` alone)
* Profile buttons should use `gap-x-3` with proper flex spacing (`min-w-0` on text containers)

---

## ⚙️ Adding Components

From `packages/ui`:

```bash
dx components add <name>
```

→ adds to Cargo.toml + correct dir.
Use for shared UI only.

---

## 🗺 Creating Pages

1. **Check if shared:**

   * Shared → `ui/`
   * Web-only → `web/src/views/`

2. **Shared page pattern:**

   * Component in `ui/`
   * Export in `lib.rs`
   * Imported by platform routes

3. **Web-only:**

   * Stay in `web/`, do not reuse elsewhere

---

## 🧩 Taskfile Rules

Use **Taskfile**, never raw shell commands.

### Common Tasks

```bash
task quality          # fmt + clippy + test + check
task run:web          # web dev server
task run:api          # API worker
task run:tailwind:dev # CSS watch
task build:web        # prod web build
task build:api        # prod API build
task deploy:db:migrate
```

### Agent Rules

1. Use existing tasks, not shell commands
2. Always run `task quality` before finishing
3. Use `task` for build, run, test, deploy, db ops

---

## 🧩 Error Fix Protocol

**Never guess.**
Follow 3 steps:

1. **Analyze:** read error, locate root cause, review related files
2. **Plan:**

   * Problem
   * Files to change
   * Fix strategy
   * Side effects
   * Verification
3. **Execute:** apply plan, test, `task quality`

---

## 📏 File Size Limit

* Max 250 lines per file
* If larger → split by component/service/util

---

## 🧹 Dead Code Policy

Delete all unused code immediately.

* No TODOs
* No commented blocks
* No “future use” code
* No unused imports, vars, or functions

> **Dead code = technical debt → delete it**

---

## 🧭 Agent Behavior Summary

<agent_behavior_summary>
  <rule name="Communication">Be terse, code-first</rule>
  <rule name="STOP">Halt immediately</rule>
  <rule name="DeadCode">Delete on sight</rule>
  <rule name="Errors">Analyze → Plan → Execute</rule>
  <rule name="FileSize">Refactor if >250 lines</rule>
  <rule name="Quality">Run task quality before finishing</rule>
  <rule name="Commands">Use Taskfile tasks only</rule>
  <rule name="NewPages">Decide shared vs platform-specific</rule>
  <rule name="SharedUI">Place in packages/ui</rule>
  <rule name="PlatformLogic">Place in platform crate (web, desktop, mobile)</rule>
  <rule name="Backend">Place in packages/api/main</rule>
  <rule name="Styling">Use Tailwind (tailwind/ package)</rule>
  <rule name="Spacing">Use standard spacing: py-6 for containers, mb-6 for sections, px-6 py-6 for sidebar</rule>
  <rule name="AddComponents">Run dx components add &lt;name&gt;</rule>
  <rule name="Auth">
    Client: ui/services/auth_service.rs
    Server: api/main/services/session.rs + api/main/endpoints/auth/
  </rule>
</agent_behavior_summary>

---

## ✅ Minimal Summary

> **Always:**
>
> * Be terse
> * Follow STOP
> * Use Taskfile
> * Run `task quality`
> * Keep files small
> * Delete dead code
> * Place code where it belongs
> * Plan before fixing

## LESSONS LEARNED

Reusable Lessons Learned
1. Signal types for component props
Pattern: When using Dioxus primitives components, props often expect ReadSignal<T>, not plain values.
Lesson: If a prop expects a ReadSignal<usize>, use use_signal(|| 0usize), not 0.
Common occurrence: Dropdown menus, select components, and other primitives that need reactive state.
2. String ownership in RSX
Pattern: Passing &String to attributes or RSX nodes often fails; Dioxus expects owned values.
Lesson: Clone strings when passing to attributes or displaying in RSX:
```
// ❌ Bad
href: to  // &String

// ✅ Good  
href: to.clone()  // String
```
Common occurrence: Any component that takes string props or displays string values.
3. Check component prop types before use
Pattern: Dioxus primitives components have specific prop requirements that may differ from expectations.
Lesson: When adding a new component via dx components add, check the generated component file to see what prop types it expects (especially for index, value, etc.).
Common occurrence: Every time you add a new component from the primitives library.
4. Router Links vs href links
Pattern: Shared components can't use router Link directly; they need to accept navigation as props or children.
Lesson: For shared components, accept navigation items as children/props. Use router Link in platform-specific wrappers.
Common occurrence: Any shared layout/navigation component that needs routing.