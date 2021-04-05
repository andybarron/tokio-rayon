# Changelog

## 2.1.0

- Re-export `rayon` crate.

## 2.0.0

- Use `std::panic` to propagate panics from the thread pool into the async
  context, rather than triggering the Rayon panic handler.
- Add `AsyncRayonHandle` type that implements `Future`, which makes the
  `async-trait` crate unnecessary.
- Bypass Tokio `RecvError`. We control the `Sender`, so it should never be
  dropped too early.
- Remove `prelude` module.
- Seal `AsyncThreadPool` trait.

## 1.0.0

- Initial release
