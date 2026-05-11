# Agent Instructions

Read `/home/li/primary/AGENTS.md` first.

This repository is a Signal contract repository. Before editing, read:

- `/home/li/primary/skills/contract-repo.md`
- `/home/li/primary/skills/rust-discipline.md`
- `/home/li/primary/skills/nix-discipline.md`
- `/home/li/primary/skills/naming.md`

This crate owns Persona origin-context vocabulary. It must not define a
Persona-specific in-band proof type. Filesystem permissions and process
spawn context are the local trust boundary; this crate only names the
typed provenance records passed after ingress.
