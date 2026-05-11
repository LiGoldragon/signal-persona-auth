# Repository Skills

- Keep this crate as a pure Signal contract vocabulary.
- Do not add daemon code, socket handling, tokio runtime code, redb
  tables, or actors here.
- Do not introduce a Persona-specific in-band proof type. The local trust
  boundary is outside this crate.
- Prefer closed enums for known Persona components and provenance
  classes. Use string newtypes only for names whose universe is outside
  this crate.
