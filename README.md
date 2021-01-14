# jtd-codegen: Generate code from JSON Typedef schemas

[JSON Type Definition](https://jsontypedef.com), aka
[RFC8927](https://tools.ietf.org/html/rfc8927), is an easy-to-learn,
standardized way to define a schema for JSON data. You can use JSON Typedef to
portably validate data across programming languages, create dummy data, generate
code, and more.

`jtd-codegen` is a CLI tool that generates code bindings in many different
programming languages from JSON Typedef schemas. For example, from this JSON
Typedef schema:

```json
{
    "properties": {
        "name": { "type": "string" },
        "isAdmin": { "type": "boolean" },
        "favoriteNumbers": { "elements": { "type": "float64" }}
    }
}
```

You can generate this TypeScript interface:

```ts
export interface User {
    favoriteNumbers: number[];
    isAdmin: boolean;
    name: string;
}
```

Or this Go type:

```go
package user

type User struct {
    FavoriteNumbers []float64 `json:"favoriteNumbers"`
    IsAdmin         bool      `json:"isAdmin"`
    Name            string    `json:"name"`
}
```

Or types/classes/structs for any of the following programming languages:

* C# with `System.Text.Json` as the JSON backend
* Golang
* Java with Jackson as the JSON backend
* Python
* TypeScript

With many more on the way. If you'd like a particular programming language
included, please open an issue on this repo!

## What is JSON Type Definition?

JSON Type Definition is a schema format for JSON data. A JSON Type Definition
schema describes what is and isn't a "valid" JSON document. JSON Type Definition
is easy to learn, portable (there are functionally-identical implementations
across many programming languages) and standardized (the spec is set in stone as
IETF RFC 8927).

Here's an example of a JSON Type Definition schema:

```json
{
    "properties": {
        "name": {
            "type": "string"
        },
        "isAdmin": {
            "type": "boolean"
        }
    }
}
```

This schema considers any object with a `name` property (whose value must be a
string), an `isAdmin` property (whose value must a boolean), and no other
properties, to be valid.

To learn more about JSON Type Definition, [check out the online documentation at
jsontypedef.com](https://jsontypedef.com).

## Installation

Go to [the latest `jtd-codegen` release on
GitHub](https://github.com/jsontypedef/json-typedef-codegen/releases/latest),
and then install the file for your platform.

## Usage

To use `jtd-codegen`, you first need to have a JSON Typedef schema to generate
data from. Let's say you have this example data already in place:

```
$ cat user.jtd.json
{
    "properties": {
        "name": { "type": "string" },
        "isAdmin": { "type": "boolean" },
        "favoriteNumbers": { "elements": { "type": "float64" }}
    }
}
```

Then you can invoke `jtd-codegen` as:

```bash
# make sure you've already created the "user" directory before running this
$ jtd-codegen user.jtd.json --typescript-out user
✍️  Writing TypeScript code to: user
📦 Generated TypeScript code.
📦	Root schema converted into type: User
```

In that example, we generated TypeScript code. If you want to generate something
else, use the appropriate "out" parameter for your desired language. For
specific instructions for each programming languages, check out the
documentation for:

* C# with `System.Text.Json` as the JSON backend
* Golang
* Java with Jackson as the JSON backend
* Python
* TypeScript

You can produce code for multiple programming languages at once. Just pass all
of the relevant parameters in the `jtd-codegen` invocation. For example:

```
$ jtd-codegen user.jtd.json --typescript-out ts-user --python-out py-user
✍️  Writing Python code to: py-user
📦 Generated Python code.
📦	Root schema converted into type: User
📦	Definition "name" converted into type: Name
✍️  Writing TypeScript code to: ts-user
📦 Generated TypeScript code.
📦	Root schema converted into type: User
📦	Definition "name" converted into type: Name
```

### Advanced Usage: Adding descriptions to generated code

If you'd like to add a commented description to generated code -- for example,
JavaDocs for Java code, or a docstring for Python -- `jtd-codegen` supports
those via the `description` and `enumDescription` fields in any schema's
`metadata`.

For example, this schema:

```json
{
    "properties": {
        "name": {
            "metadata": {
                "description": "The user's name"
            },
            "type": "string"
        },
        "status": {
            "metadata": {
                "description": "The user's account status",
                "enumDescription": {
                    "UNVERIFIED": "The user's email has not yet been verified",
                    "VERIFIED": "The user's email has been verified",
                    "DISABLED": "The user's account was terminated"
                }
            },
            "enum": ["UNVERIFIED", "VERIFIED", "DISABLED"]
        }
    }
}
```

Generates into this TypeScript:

```ts
/**
 * The user's account status
 */
export enum UserStatus {
    /**
     * The user's account was terminated
     */
	Disabled = "DISABLED",

    /**
     * The user's email has not yet been verified
     */
	Unverified = "UNVERIFIED",

    /**
     * The user's email has been verified
     */
	Verified = "VERIFIED",
}

export interface User {
    /**
     * The user's name
     */
    name: string;

    /**
     * The user's account status
     */
    status: UserStatus;
}
```

### Advanced Usage: Customizing `jtd-codegen` output

If you'd like to force `jtd-codegen` to use a particular type/class for some
subset of your schema, you can use a "type override" property to do this. For
example, if you generate TypeScript from this schema:

```json
{
    "properties": {
        "name": {
            "metadata": {
                "typescriptType": "MyCustomNameType"
            },
            "type": "string"
        },
        "isAdmin": { "type": "boolean" },
        "favoriteNumbers": { "elements": { "type": "float64" }}
    }
}
```

You'll get:

```ts
export interface User {
    favoriteNumbers: number[];
    isAdmin: boolean;
    name: MyCustomNameType;
}
```

Each language supported by `jtd-codegen` supports a different set of overrides:

* C# with `System.Text.Json` as the JSON backend
    * `csharpSystemTextType` overrides the entire outputted type
    * `csharpSystemTextContainer` overrides `IList<T>` or `IDictionary<string,
      T>` in favor of a different container type
* Golang
    * `goType` overrides the entire outputted type
* Java with Jackson as the JSON backend
    * `javaJacksonType` overrides the entire outputted type
    * `javaJacksonContainer` overrides `List<T>` or `Map<String, T>` in favor of
      a different container type
* Python
    * `pythonType` overrides the entire outputted type
* TypeScript
    * `typescriptType` overrides the entire outputted type

### Advanced Usage: Using `jtd-codegen` in a larger build process

If you're using `jtd-codegen` as part of a larger build process (for example: if
you're building an OpenAPI-like format on top of JSON Typedef), you may find it
useful to programmatically get back the names of `jtd-codegen`-generated types.
`jtd-codegen` supports this use-case via the `--log-format` CLI option.

By default, `jtd-codegen` uses `--log-format pretty`, which outputs
human-friendly text to stdout. This is an example of `pretty` output:

```
✍️  Writing TypeScript code to: user
📦 Generated TypeScript code.
📦	Root schema converted into type: User
📦	Definition "name" converted into type: Name
```

If instead you use `--log-format minimal`, then `jtd-codegen` outputs startup
diagnostic information to stderr, and information about generated data
structures to stdout:

```
TypeScript: writing to: user
TypeScript: root: User
TypeScript: definition: name: Name
```

(The first line above is to stderr, the subsequent two lines are to stdout.)

Finally, `--log-format json` outputs information about generated data structures
to stdout as JSON. No startup information is produced:

```json
{
  "TypeScript": {
    "out_dir": "user",
    "root_name": "User",
    "definition_names": {
      "name": "Name"
    }
  }
}
```

Typically speaking, `--log-format minimal` is easier to process in simple bash
scripts. `--log-format json` is often easier to use from anything that's not a
shell-like programming language.
