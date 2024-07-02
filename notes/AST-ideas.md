# Ideas for a nice AST API

```rust
let mat = MaterialX::from_str("..")?;

// all returns iterator over nodes matching specified type
let first_material = mat.all<SurfaceMaterial>().first().unwrap();
// get fetches node by name and casts it to specified type
let surface = mat.get<StandardSurface>(first_material.get<Input>("surfaceshader").nodename);
// or: index operator returning most generic node type
// might be clever on Vec<Input> to return Input instead of Node
// decide: panic or return dummy node
let surface = mat.get<StandardSurface>(first_material["surfaceshader"].nodename);
```

Ideally, we'd be able to have as much of the parsed as type-safe as possible.
Currently we try this using quick-xml and serde,
but we might need to be more dynamic to support the full MaterialX spec
while providing a nice API.
