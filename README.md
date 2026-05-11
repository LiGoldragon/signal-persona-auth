# signal-persona-auth

`signal-persona-auth` is the Persona Signal contract for ingress
context and route identity.

It does not prove identity. Local proof happens before a component
accepts a socket, through daemon-created sockets, filesystem
permissions, inherited process context, and operating-system identity.
After that boundary, this crate carries typed provenance such as
`ConnectionClass`, `MessageOrigin`, and `IngressContext`.

