# jtd-codegen

`jtd-codegen` generates code (datatypes, classes, etc.) in many programming
languages from JSON Typedef schemas.

For high-level guidance on how to use this package, see ["Generating Code from
JSON Typedef Schemas"][jtd-jtd-codegen] in the JSON Typedef docs. For high-level
guidance on how to use this package with your particular programming language of
choice, see:

* ["Generating TypeScript from JSON Typedef Schemas"][jtd-ts-codegen]
* ["Generating Golang from JSON Typedef Schemas"][jtd-go-codegen]

In addition to the general docs in this README, each programming language that
`jtd-codegen` supports has its own set of specific documentation:

* [`jtd-codegen` + TypeScript README](./src/target/typescript)
* [`jtd-codegen` + Golang README](./src/target/go)

## Installation

To install `jtd-codegen`, you have a few options:

### Install with Homebrew

This option is recommended if you're on macOS.

```bash
brew install jsontypedef/jsontypedef/jtd-codegen
```

### Install with Docker

This option is recommended on non-Mac platforms, or if you're running
`jtd-codegen` in some sort of script and you want to make sure that everyone
running the script uses the same version of `jtd-codegen`.

```bash
docker pull jsontypedef/jtd-tools
```

If you opt to use the Docker approach, you will need to change all invocations
of `jtd-codegen` in this README from:

```bash
jtd-codegen [...]
```

To:

```bash
docker exec -it jsontypedef/jtd-tools /jtd-codegen [...]
```

### Install with Cargo

This option is recommended if you already have `cargo` installed, or if you
would prefer to use a version of `jtd-codegen` compiled on your machine:

```bash
cargo install jtd-codegen
```

## Usage

> See the top of this README for links to high-level guidance and specifics for
> each programming language.

For help running `jtd-codegen`, run:

```bash
jtd-codegen --help
```

There are two prerequisites to invoking `jtd-codegen`:

1. You need to have your schema in a file. `jtd-codegen` will use the name of
   that file to infer the names of the datatypes/classes/etc it will generate.

2. You need to have a separate "output" directory for each output language you
   want to use. `jtd-codegen` will not create this directory for you.

For example, if you have a schema called `user.jtd.json` that looks like this:

```json
{
  "properties": {
    "name": { "type": "string" },
    "created_at": { "type": "timestamp" },
    "favorite_numbers": {
      "elements": { "type": "float64" }
    }
  }
}
```

### Generate code for a single programming language

To create code for a single programming language, run:

```bash
# Generate TypeScript into "out/" from user.jtd.json
jtd-codegen --typescript-out=out -- user.jtd.json
```

This will create a file called `out/index.ts` that looks like this:

```ts
export interface User {
  createdAt: string;
  favoriteNumbers: number[];
  name: string;
}
```

Each programming language has its own set of options. See `jtd-codegen --help`
and the specific documentation for each programming language for more specifics.

### Generate code for multiple programming languages

To create code for multiple programming languages at once, pass flags for each
programming language you'd like to generate code for:

```bash
# Generate Golang into "go_out/" and Java into "java-out/" from user.jtd.json
jtd-codegen --go-out=go_out --java-out=java-out --java-pkg=com.example.user -- user.jtd.json
```

This will create two files, called `go_out/index.go`:

```go
package go_out

import "time"

type User struct {
  CreatedAt time.Time `json:"created_at"`
  FavoriteNumbers []float64 `json:"favorite_numbers"`
  Name string `json:"name"`
}
```

And another called `java-out/User.java`:

```java
package com.example.user;

import java.time.OffsetDateTime;

import com.fasterxml.jackson.annotation.JsonProperty;

public class User {
  @JsonProperty("created_at")
  private OffsetDateTime createdAt;

  @JsonProperty("favorite_numbers")
  private List<Double> favoriteNumbers;

  @JsonProperty("name")
  private String name;

  public User() {
  }

  public OffsetDateTime getCreatedAt() {
    return createdAt;
  }

  public void setCreatedAt(OffsetDateTime createdAt) {
    this.createdAt = createdAt;
  }

  public List<Double> getFavoriteNumbers() {
    return favoriteNumbers;
  }

  public void setFavoriteNumbers(List<Double> favoriteNumbers) {
    this.favoriteNumbers = favoriteNumbers;
  }

  public String getName() {
    return name;
  }

  public void setName(String name) {
    this.name = name;
  }
}
```

[jtd-jtd-codegen]: https://jsontypedef.com/docs/tools/jtd-codegen
[jtd-ts-codegen]: https://jsontypedef.com/docs/javascript/code-generation
[jtd-go-codegen]: https://jsontypedef.com/docs/golang/code-generation
