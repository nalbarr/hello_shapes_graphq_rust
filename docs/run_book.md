# run_book

## How to run

- Run just to see available recipes
  `just`

- build source
  `just build`

- run local web server
  `just run`

- test query using curl
  `just test`

## Known issues

- Main entry point must directive to explicitly configure for single vs. multi-threading or will result in
  - The default runtime flavor is `multi_thread`, but the `rt-multi-thread` feature is disabled.

`#[tokio::main(flavor = "current_thread")]` preferred over default
`#[tokio::main]`

## References

- https://github.com/tokio-rs/tokio/discussions/4718
- https://www.maxivanov.io/make-graphql-requests-with-curl/
