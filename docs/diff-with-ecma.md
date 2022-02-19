# Differences between this library and ECMAScript Temporal

## Duration

`Duration` in ECMA is renamed to `NominalDuration` in order to prevent confusion with
`std::time::Duration`. There is also a `SignedDuration`, signed counterpart of
std's `Duration`.

## Instant

There is no `Instant` in this library, in order to prevent confusion with
`std::time::Instant`. Role of instant is mostly done by keeping a `SignedDuration` since
unix epoch.

## Resolving ambiguty and overflow

In ECMA, there is a default mode for handling ambiguty and overflows, for example in
overflow the default mode is `constrain`, and these are equivalent:

```JS
Temporal.PlainDate.from({ year: 2001, month: 13, day: 1 }, { overflow: 'constrain' })
Temporal.PlainDate.from({ year: 2001, month: 13, day: 1 })
```

In Rust, there is no optional arguments, and no exception (for reject mode). So functions
that may overflow return a Result-like thing with a method `constrain` on it, so the above
will become like this:

```Rust
use temporal_core::{PlainDate, Calendar};
PlainDate::from_ymd(2001, 13, 1, Calendar::Iso8601).constrain();
```

For `reject` behavior, there are typical methods of `Result`:

```Rust
use temporal_core::{PlainDate, Calendar};
PlainDate::from_ymd(2001, 13, 1, Calendar::Iso8601).unwrap(); // Panics!
PlainDate::from_ymd(2001, 13, 1, Calendar::Iso8601).ok(); // None
PlainDate::from_ymd(2001, 13, 1, Calendar::Iso8601).is_ok(); // false
```

## Sub second time zone offset

ECMA supports time zones with sub second offset, like `+02:35:53.1423`. We don't support
them, because we store seconds and subsecond part separated, so it will need mixing them in
a single nanosecond offset, calculate the result, and separate it back with division. This
is unnecessary for all practical usages.
