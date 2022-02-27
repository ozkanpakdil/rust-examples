# how to use timestamp in juniper

check [structs](src/schema.rs)

![example query look](https://user-images.githubusercontent.com/604405/155902564-ae9debe6-8077-401e-bdaa-a0e0ce2052f8.png)

I am not sure which is best, but we can get timestamp as string or integer.

# (Original source)GraphQL using Juniper

[Juniper](https://github.com/graphql-rust/juniper) integration for Actix Web.
If you want more advanced example, see also the [juniper-advanced example].

[juniper-advanced example]: https://github.com/actix/examples/tree/master/graphql/juniper-advanced

## Usage

### Server

```sh
cd graphql/juniper
cargo run
```

### Web Client

Go to <http://localhost:8080/graphiql> in your browser.

_Query example:_

```graphql
{
  human(id: "1234") {
    name
    appearsIn
    createDate
    createDate1
    createDate2
  }
}
```

_Result:_

```json
{
  "data": {
    "human": {
      "name": "Luke",
      "appearsIn": [
        "NEW_HOPE"
      ],
      "createDate": "1646000247",
      "createDate1": "2022-02-27T22:17:27.556208535+00:00",
      "createDate2": 1646000247
    }
  }
}
```
