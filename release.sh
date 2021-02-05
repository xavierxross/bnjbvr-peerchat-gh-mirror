#!/bin/bash

(cd client && NODE_ENV=production yarn build)
(cd server && cargo build --release)
zip release.zip -r ./client/public ./server/target/release/server
