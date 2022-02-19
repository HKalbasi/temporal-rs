# temporal-rs
Intl-first date and time library for rust

## Based on ECMAScript's Temporal API

Designing a full featured date time library is hard, which is why standard library doesn't provide it. Temporal is a new date time
library for JS, which is being standardized for years (not finished yet). It is designed with the experience from previous efforts
and it supports multiple calendars, timezones and DST, arithmetic of those, all with corner cases considered, and compatible and
interoperable with other systems by using standards such as iso8601 and RFC 5545 (iCalendar). So it make sense to follow their
decisions closely.

But there is some difference, for making this library more rust friendly. See [the differences](./docs/diff-with-ecma.md).

## How to use

There is [a cookbook](./docs/cookbook.md), contains examples of common tasks.





