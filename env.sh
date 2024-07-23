#!/bin/bash

function serve {
    cd ui || exit 1
    RUSTFLAGS=--cfg=web_sys_unstable_apis APP_BASE_PATH=media-manager trunk serve --features=demo
}

function nginx-reload {
    podman-compose -f compose.yaml exec nginx nginx -s reload
}
