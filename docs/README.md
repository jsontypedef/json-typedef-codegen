# `jtd-codegen` Technical Documentation

This document is a set of technical documentation about the sort of code
`jtd-codegen` will produce. As you use `jtd-codegen` in larger projects, or when
you embed `jtd-codegen` as a component of a broader code generation pipeline,
this document will help you understand what `jtd-codegen` will and will not do
for you.

What follows is a prioritized set of requirements that `jtd-codegen` will
typically follow when generating code. The point of these requirements is to:

1. Deliver a consistent and pleasant user experience for developers writing
   against generated code, and
2. Reduce "gotchas", like having generated code produce classes whose names
   suggest they represent something they don't actually represent.

Broadly speaking, `jtd-codegen`'s requirements, in order of priority, are:

1. **Generated code will compile.** Above all else, `jtd-codegen` will produce
   code that is syntactically correct and, if the relevant language has a
   compiler (`javac`, `rustc`, `tsc`, etc.), then the code will compile.

2. **Generated code will round-trip JSON satisfying the relevant schema.** If
   you generate code from `foo.jtd.json`, then the "top-level" data structure
   `jtd-codegen` produced from that schema will support deserialization from
   JSON satisfying `foo.jtd.json`, and can be serialized back into data
   satisfying `foo.jtd.json`.

   This round-tripping is not guaranteed to preserve all data if you are using
   `additionalProperties: true` in your schema; `additionalProperties` lets you
   pass in "extra" properties in an object, and JTD implementations will
   "ignore" those properties. In `jtd-codegen`-generated code, such "extra"
   properties will be ignored during deserialization, and so will be omitted
   from serialization.

   If you give `jtd-codegen`-generated code JSON that does not satisfy the
   relevant schema, then you will not get round-tripping. Instead, you'll get an
   exception or error depending on the language.

3. **Generated code will have a friendly name for the "root" data structure.**
   If you generate code from `foo.jtd.json`, then `jtd-codegen` will produce a
   data structure named `Foo` or `foo` (depending on what the language's
   convention is). That data structure will represent the "root" of the schema.

   Requirement (1) may force `jtd-codegen` to mangle the name of your schema.
   For instance, if your schema is in a file called `1.jtd.json` or
   `string.jtd.json`, then `jtd-codegen` may generate a less-friendly name for
   your data in order to make sure the code still works.

4. **Generated code will have a friendly name for each of the definitions.** If
   you have a definition named `bar` in a schema, then `jtd-codegen` will try to
   produce a data structure named `Bar` of `bar` (dependong on the language's
   convention). That data structure will represent the "bar" definition.

   Requirements (1) and (3) take precedent over this requirement. For instance,
   if you name your definition `1` or `string`, or if you name a definition
   `user` inside a schema called `user.jtd.json`, then `jtd-codegen` may
   generate a less-friendly name to make sure the code works and the "root" data
   structure has a friendly name.

5. **Generated code will be "plain-old" data structures.** In languages where
   there's a distinction between "fancy" and "plain" data structures,
   `jtd-codegen` will generate "plain" data structures. These data structures
   are usually more convenient to work with.

   For instance, `jtd-codegen` generates Python `@dataclass` classes with
   ordinary `from_json` and `to_json` methods. Generated Python classes do not
   do their own manipulation of "dunder" (`__dict__`, etc.) methods.

   In some languages, the "plain-old" way to do something isn't powerful enough
   to represent a schema. For instance, the schema consisting of just:

   ```json
   { "type": "string" }
   ```

   Requirement (3) says we need to generate a friendly name, so we basically
   need to make a type named "Foo" that's just an alias for a string. In
   TypeScript, that's easy:

   ```ts
   type Foo = string;
   ```

   In other languages, we may need to generate some data structure that is a
   pass-through "wrapper" for the data. For instance, in Java:

   ```java
   // Assuming we're using the "Jackson" mode for Java codegen, which gives us
   // the @JsonValue annotation.
   public class Foo {
       @JsonValue
       private String value;

       public Foo() {}
       public String getValue() { return value; }
       public void setValue(String value) { this.value = value; }
   }
   ```

   These are the sorts of compromises we must occasionally perform in order to
   give a consistent experience.

6. **Generated code will have documentation comments.** "Documentation comments"
   means things like Javadocs, C# XML comments, Python heredocs, or whatever the
   equivalent is for the language.

   `jtd-codegen` will look for strings to put in documentation comments in the
   `description` property of a schema's `metadata`. For enums, `jtd-codegen`
   will try to document each enum value using a corresponding dictionary entry
   in `metadata.enumDescriptions`. For instance:

   ```json
   {
       "metdata": {
            "description": "A job in our message-processing system."
        },
        "properties": {
            "id": {
                "metadata": {
                    "description": "The unique identifier for the job."
                },
                "type": "string"
            },
            "status": {
                "metadata": {
                    "description": "The processing status of the job.",
                    "enumDescriptions": {
                        "QUEUED": "The job is still waiting to be processed.",
                        "IN_PROGRESS": "The job is being processed.",
                        "DONE": "The job is completed."
                    }
                },
                "enum": ["QUEUED", "IN_PROGRESS", "DONE"]
            }
        }
   }
   ```

7. **Generated code will have friendly names for internal properties.** In other
   words, `jtd-codegen` will avoid producing mangled names when possible, and
   instead generate names that correspond to the "path to" the relevant
   sub-schema. For instance, consider:

   ```json
   {
       "properties": {
           "bar": {
               "properties": {
                   "widgets": {
                       "elements": {
                           "properties": {
                               "widget_id": { "type": "string" }
                           }
                       }
                   }
               }
           }
       }
   }
   ```

   If that schema were in `foo.jtd.json`, then `jtd-codegen` will produce data
   structures with names like `Foo`, `FooBar`, and `FooBarWidget` (where
   `jtd-codegen` saw that it could "singularize" `widgets` into `widget`, since
   the schema uses `elements`).

   If `jtd-codegen` can't come up with a friendly, unique name for something, it
   will generate an unambiguous one instead.

8. **Generated code will be "pretty".** `jtd-codegen` will, when possible,
   generate code that humans can read and would consider "pretty".

   This is the very lowest requirement. For many languages, there are competing
   and mutually incompatible conventions. If you care about the prettiness of
   generated code, run your preferred code formatter on `jtd-codegen`'s output.
