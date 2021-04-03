# Changelog

## 2.0.0

- Use `std::panic` to propagate panics from the thread pool into the async
  context, rather than triggering the Rayon panic handler.
- Add `AsyncHandle` type that implements `Future`, which makes the
  `async-trait` crate unnecessary.
- Add custom `Error` type.

## 1.0.0

- Initial release
