# Contribution Guidelines

General standards and expectations for the code contributed to Voxy.

This is intended to be an in-depth reference for both reviewers and 
contributors. Extensions to this standard should be submitted through an
[issue](https://github.com/BoringOrng/voxy/issues).

---

Licensed under either of

- Apache License, Version 2.0
  ([LICENSE-APACHE](../LICENSE-APACHE))
- MIT License
  ([LICENSE-MIT](../LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions.

---

### Index

- [Cargo.toml](#cargotoml)
  - [Adding Dependencies](#adding-dependencies)
  - [Removing Dependencies](#removing-dependencies)
  - [After Making Changes](#after-making-changes)

- [Crates](#crates)
  - [Adding a Crate](#adding-a-crate)
  - [Removing a Crate](#removing-a-crate)
  - [Formatting a Crate](#formatting-a-crate)
    - [Formatting a Library](#formatting-a-library)
    - [Formatting a Binary](#formatting-a-binary)

- [Modules](#modules)

- [Naming Conventions](#naming-conventions)
  - [Clarity Over Brevity](#clarity-over-brevity)
  - [Getters and Setters](#getters-and-setters)
  - [Public Fields](#public-fields)

---

## Cargo.toml

### Adding Dependencies

Dependencies should be carefully selected. In practice this means weighing the
pros and cons adding the library. To provide some examples:

---

You shouldn't add a library if:
- It is unmaintained;
- It is solely used to conform to *your* style;
- We already have a library that performs a similar-enough task.

---

You should add a library if:
- The feature it provides is too complex for us to write our own.
- The feature it provides cannot specialize to Voxy's architecture.
- The feature it provides is significantly faster than our own.

---

> [!Important]
> If you're working across crate boundaries, and you find that a library is
> needed for both, but is not included via workspace, you take responsibility
> for moving the library into the workspace.
>
> For example, given the files:
>
> ```toml
> # vx-foo
> [dependencies]
> bar = "x.x.x"
>
> # vx-baz
> [dependencies]
> bar = "x.x.x"
> ```
>
> You should convert it to:
>
> ```toml
> # Cargo.toml; in the workspace root
> [workspace.dependencies]
> bar = "x.x.x"
>
> # vx-foo
> [dependencies]
> bar.workspace = true
>
> # vx-baz
> [dependencies]
> bar.workspace = true
> ```
>
> Another thing worth noting, If a dependency is found like `bar` in the first
> example, but the versions are different, try to use the latest version in the
> workspace. If you get warnings, or it fails to compile, undo those changes,
> and create an [issue](https://github.com/BoringOrng/voxy/issues).

### Removing Dependencies

Removing dependencies should be done where possible, either by feature-trimming
or removing them entirely. More complex removals of dependencies, should first
involve the creation of an [issue](https://github.com/BoringOrng/voxy/issues).
To give an example, if we use a palette-compression library, but for one reason
or another it is found unsuitable, you should open an create an [issue
](https://github.com/BoringOrng/voxy/issues) that describes why the library is
unsuitable, and ask whether or not you can create your own `crates/vx-palette`
or `crates/vx-datastructure/palette`.

### After Making Changes

After making changes to a `Cargo.toml`, you must do two things. First involves
running the following command:

```bash
cargo sort -w
```

> [!Tip]
Instructions for installing `cargo-sort` can be found
[here](development.md#tooling).

---

The next step is verifying that the changed `Cargo.toml`s adhere to the
following general structure:

```toml
[package]
name              = "vx-foo"
version.workspace = true
authors.workspace = true
edition.workspace = true
license.workspace = true

[dependencies]
# dependencies in our workspace (excluding vx_*) go here.
bar.workspace  = true
bevy.workspace = true

# vx-* dependencies go here
vx-entity.workspace = true
vx-world.workspace  = true

# dependencies solely for this crate go here
baz    = "x.x.x"
# dependencies that enable more than one feature should use the following
# format
qux    = { version = "x.x.x", features = [
    "quux",
    "corge",
] }
grault = "x.x.x"

[lints]
workspace = true
```

> [!Important]
You may have noticed the alignment of the equal signs (`=`), that is
intentional, and the expected format.

---

## Crates

All crates must follow the naming convention `vx-*` and live in the `crates/` directory. All crates should follow `kebab-case` for their naming conventions.

### Adding a Crate

> [!Important]
Before adding a crate, it should be determined whether or not adding a new
crate is the best course of action via filing an [issue
](https://github.com/BoringOrng/voxy/issues).

When adding a new `vx-*` crate, it should be registered in the workspace
dependencies, as follows:

```toml
# Cargo.toml; in the workspace root
[workspace.dependencies]
vx-my-cool-crate.path = "crates/vx-my-cool-crate"
```

> [!Important]
Because you just edited a `Cargo.toml`, don't forget [what to do after making
changes](#after-making-changes).

> [!Tip]
If you're not sure what to do from here, see [how to format your crate
](#formatting-a-crate).

### Removing a Crate

There are very few instances in which it is determined the best option to
remove a crate. Any attempts to do so should first involve filing an [issue
](https://github.com/BoringOrng/voxy/issues). Unless sufficient grounds are
given, it can be expected that said request will be denied.

---

Sufficient grounds include, but are not limited to:
- A dependency exists now, that meets our needs.
- Benchmarking proves that a previously disregarded dependency functions 
better than our own implementation.
- A major refactor occurred, and what was once a crate makes more sense to 
exist as a module in another crate.

> [!Important]
All attempts to remove a crate, to replace it with a functionally similar 
crate, will only be considered can maintenance assurances be provided.

### Formatting a Crate

#### Formatting a Library

#### Formatting a Binary

---

## Modules

## Naming Conventions

### Clarity Over Brevity

However, that doesn't mean brevity should be ignored. Names like
`SuperSpecificNameForThisOneTypeIMade` are very clear, and would be considered
preferring clarity over brevity, but it ignores the concept of brevity. Clarity
and brevity should work hand-in-hand, building off of the context each provides.

Take for instance, `Id`, an exceedingly brief name, but what if I tell you that
this `Id` is the only `Id` ever exposed by the module's API? Brevity tells you
that means `Id` is perfect for the job, after all, there's only one, but clarity
tells you that `Id` is very brief, and doesn't give you the context you need. So,
let's call it `BlockId`, problem solved!

Except it's not solved. You have a new problem now, by focusing explicitly on the
type, you have entirely missed the forest for the trees. If we are to zoom out to
the big picture, say, the whole crate, we have just created the type,
`vx_world::block::BlockId`. While `BlockId` in isolation would be the best name,
given it's context, it is now *too* clear, thus we rollback to `Id`.

> [!Important]
Ambiguous names like `Id` should **always** be prefixed by their parent scope.
e.g. `block::Id`.

While I would love to leave things at that, there are a couple exceptions to this
rule. 

The first exception is if the ambiguous name is ambiguous in concept. When you
read a type like `Id`, you know it must be used to *identify*; this is because
it isn't ambiguous in concept. Now, what about a name like `Map`? Think about it,
`vx_world::chunk::Map` is perfect! It's just a `ChunkMap`! Or is it a geographic
map, maybe a functional map, perhaps a `HashMap`? It's just *too* ambiguous, that's
what it means to be ambiguous in concept.

> [!Important]
Ambiguity in this sense doesn't explicitly mean, "does a dictionary list more
than one definition for the word", it is more or less, "is there more than one
sense that's live in the domain a reader would reasonably expect". `Map` fails
because both "associative container" and "spatial map" are plausible in a voxel 
engine's vocabulary.

The second exception is if the ambiguous name exists more than once in its
parent scope, a rather lengthy exception, but the easiest way to imagine
the concept is using `Error`s. `Error` is unambiguous in concept, but what if
multiple `Error`s exist within the parent scope? At that point, the clearest, and
briefest way is to disambiguate them by name; the best example of this is in the
`std::num` API, providing the `Error`s `ParseIntError`, `ParseFloatError`, and
`TryFromIntError`. You must disambiguate at the identifier-level, because even
though `Error`s are unambiguous, the number module may fail in different kinds of
ways.

While what's been discussed largely involves struct-naming, the same rules apply
to function naming. The difference is that the struct (or other enclosing scope)
acts as the disambiguating context rather than the module. Just as `BlockPos`
becomes `block::Pos`, `insert_chunk` becomes `ChunkMap::insert`.

> [!Important]
The same disambiguation rules apply. If you may only `get`, `insert`, `build`, etc.
one thing (as far as the enclosing scope's public API exposes), then said methods
should just be called `get`, `insert`, `build`, etc. If the enclosing scope's public
API exposes a way to `get`, `insert`, `build`, etc. more than one thing, they should
be disambiguated at the identifier-level.

---

### Getters and Setters

Setters shouldn't exist. If you wish to create a setter, you should instead use
the format, `get_mut`, or `*_mut`, and return a mutable reference to what you
wish to set. If you really need some form of setter, then use the [with_*
pattern](#with-pattern).

> [!Tip]
If you find yourself in a position where you wish to reuse a value but with some
altered state, what you need isn't a setter, it's
[the with pattern](#the-with-pattern).

A question that may come up when faced with writing getters is "why not just
make the field public?". That's a perfectly valid question, and the reasoning
for that can be found [here](#public-fields).

Most of the time, you should avoid writing methods starting with `get_`. Instead,
opt for using the field name.

```Rust
pub struct Foo {
    bar: Bar,
}

impl Foo {
    // not `get_bar`
    #[must_use]
    pub const fn bar(&self) -> &Bar {
        &self.bar
    }

    // neither `mut_bar`, `get_bar_mut`, nor `get_mut_bar`
    #[must_use]
    pub const fn bar_mut(&mut self) -> &mut Bar {
        &mut self.bar
    }
}
```

As with most rules, there are exceptions. Most notably, if it is entirely
unambiguous what you're getting; even that though has its caveats. The argument
for unambiguity will only not be considered for struct members that contain
meaningful names. For structs like `Foo` above, the field `bar`, for all
intents and purposes, is considered meaningful.

For instance:

```Rust
// ambiguous as it has no name.
pub struct Bar(Baz);

// ambiguous as it is a wrapper struct for `Qux`.
pub struct Baz {
    // `inner` will be the only identifier that qualifies ambiguity.
    inner: Qux,
}
```

Other notable examples include, but aren't limited to, `Vec`, `HashMap`, and
most other containers in the standard library. In fact, this applies to most
containers across the Rust ecosystem, which introduces the next exception.

```Rust
pub struct Registry {
    by_string: HashMap<String, super::Id>,
    by_id: HashMap<super::Id, super::MyDatatype>,
}
```

In this case, it's difficult to tell what you would name your getters; in fact,
the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/) don't
actually mention this case at all in their section on
[getters](https://rust-lang.github.io/api-guidelines/naming.html#c-getter). With
that said, here's how you *should* do it.

---

> [!Tip]
If you've read and digested the section on
[clarity over brevity](#clarity-over-brevity), the following code will come
naturally.

```Rust
impl Registry {
    #[must_use]
    pub fn get_id(&self, string: &str) -> Option<super::Id> {
        // note that this assumes `super::Id: Copy`
        self.by_string.get(string).copied()
    }

    #[must_use]
    pub fn get_my_datatype(&self, id: super::Id) -> Option<&super::MyDatatype> {
        self.by_id.get(&id)
    }

    #[must_use]
    pub fn get_my_datatype_by_string(
        &self,
        string: &str,
    ) -> Option<&super::MyDatatype> {
        self.get_my_datatype(self.get_id(string)?)
    }
}
```

> [!Important]
If you're defining a similar type generic over `super::MyDatatype`, make the
following name changes:
> - `get_my_datatype` -> `get`
> - `get_my_datatype_by_string` -> `get_by_string`

---

### The With Pattern



## Public Fields
