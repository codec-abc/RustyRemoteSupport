Tech stack:

1. Server: Rust server with Warp
2. Client: Rust wasm frontend with Seed and Bevy
3. Shared: Data structure definition

Bevy & seed interop:

1- In Seed init: create a bevy instance
2- When creating a bevy instance, create a plugin and ressource containing a ref to the Seed app
3- Keep a ref to the plugin in Seed in order to inject stuff into Bevy.