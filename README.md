# Either

The enum type `Either` with variants `Left` and `Right` is a general purpose sum type with two cases.

```rust,ignore
enum Either<L, R> {
   Left(L),
   Right(R),
}
```

Different from `Result`, which represents success or failure, variants `Left` and `Right` are symmetric and represent values without preference.

## Method overview

## Querying the variant

The `is_left` and `is_right` methods return `true` if the `Either` is `Left` or `Right`, respectively.

Futhermore, the `is_left_and` and `is_right_and` methods return `true` if the `Left` or `Right` variant matches a predicate.

## Adapters for working with references

- `as_ref` converts from `&Either<L, R>` to `Either<&L, &R>`
- `as_mut` converts from `&mut Either<L, R>` to `Either<&mut L, &mut R>`
- `as_deref` converts from `&Either<L, R>` to `Either<&L::Target, &R::Target>`
- `as_deref_mut` converts from `&mut Either<L, R>` to `Either<&mut L::Target, &mut R::Target>`

## Extracting contained values

These methods extract the contained `Left` value in an `Either`. If the `Either` is `Right`:

- `left` returns `None`
- `left_or_err` returns provided error contained in `Err`
- `into_left` converts the right value to `L` (which requires `R` to implement `Int<L>`)
- `try_into_left` attempts to convert the right value to `L` (which requires `R` to implement `TryInt<L>`)
- `expect_left` panics with a provided custom message
- `unwrap_left` panics with a generic message
- `unwrap_left_or` returns the provided default value
- `unwrap_left_or_else` returns the result of evaluating the provided function
- `unwrap_left_or_default` returns the default value of the type `L` (which requires `L` to implement `Default`)

Also, there are symmetric methods that work with the `Right` variant.

### Transforming contained values

There are some methods to transform an `Either`:

- `flip` converts from `Either<L, R>` to `Either<R, L>`
- `into_inner` converts from `Either<T, T>` to `T`

The `map` and `fold` methods are the most basic higher-order functions to transform an `Either`:

- `map` applies one of the two functions on the `Left` or `Right` variant according to its content, returning the result re-wrapped in `Left` or `Right`
- `fold` applies one of the two functions on the `Left` or `Right` variant according to its content, returning the unified result

The `map_with` and `fold_with` methods are the contextual version of the `map` and `fold` methods, respectively.

These methods transform the `Left` value by applying a function on it. If `Either` is `Right`:

- `map_left` returns the `Right` value
- `map_left_or` returns the provided default value
- `map_left_or_else` returns the result of evaluating the provided function
- `map_left_or_default` returns the default value of `L` (which requires `L` to implement `Default`)

Also, there are symmetric methods that work with the `Right` variant.

The `From` trait allows to factorize or transpose the `Either`:

| From                                     | Into                                     | Description                                                     |
| ---------------------------------------- | ---------------------------------------- | --------------------------------------------------------------- |
| `Either<(T, L), (T, R)>`                 | `(T, Either<L, R>)`                      | factorize an `Either` of a tuple of two elements                |
| `Either<(L, T), (R, T)>`                 | `(Either<L, R>, T)`                      | factorize an `Either` of a tuple of two elements                |
| `Either<Option<L>, Option<R>>`           | `Option<Either<L, R>>`                   | factorize an `Either` of `Option`                               |
| `Either<Result<L, E>, Result<R, E>>`     | `Result<Either<L, R>, E>`                | factorize an `Either` of `Result`                               |
| `Either<Result<T, L>, Result<T, R>>`     | `Result<T, Either<L, R>>`                | factorize an `Either` of `Result`                               |
| `Either<Result<T1, E1>, Result<T2, E2>>` | `Result<Either<T1, T2>, Either<E1, E2>>` | transpose between `Either` of `Result` and `Result` of `Either` |

## Boolean operators

These methods treat the `Result` as a boolean value:

| method      | alias      | self       | input     | output     |
| ----------- | ---------- | ---------- | --------- | ---------- |
| `right_and` | `left_or`  | `Left(x)`  | (ignored) | `Left(x)`  |
| `right_and` | `left_or`  | `Right(x)` | `y`       | `y`        |
| `left_and`  | `right_or` | `Left(e)`  | `y`       | `y`        |
| `left_and`  | `right_or` | `Right(x)` | (ignored) | `Right(x)` |

The `left_and_then` (alias `right_or_else`) and `right_and_then` (alias `left_or_else`) methods take a function as input, and only evaluate the function when they need to produce a new value.

## Modifying contained values

- `insert_left` inserts provided value into the `Left` variant.
- `left_or_insert` inserts provided value into the `Left` variant only if it is `Right`.
- `left_or_insert_with` inserts a value computed from provided function into the `Left` variant if it is `Right`.
- `left_and_modify` modifies the `Left` variant, if any.

Also, there are symmetric methods that work with the `Right` variant.
