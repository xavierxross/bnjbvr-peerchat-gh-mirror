# peerchat

(name is a work-in-progress)

## dev

- `cd client && yarn && yarn dev`
- `cd server && cargo run`
    - or better if you've installed [cargo-watch](https://crates.io/crates/cargo-watch), `cargo watch -x run`.
- go to [localhost:8080](http://localhost:8080) by default
- tada!

## production

- `cd server && HOST="0.0.0.0" PORT=1337 cargo run --release`

### docker

- `docker build -t bnjbvr/peerchat . && docker run`
- `docker run --rm -ti -p 8080:8080 bnjbvr/peerchat`
