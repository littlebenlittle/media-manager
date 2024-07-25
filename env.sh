#!/bin/bash

function serve {
    pushd ui || exit 1
        RUSTFLAGS=--cfg=web_sys_unstable_apis APP_BASE_PATH=media-manager trunk serve --features=demo
    popd || exit 1
}

function nginx-reload {
    podman-compose -f compose.yaml exec nginx nginx -s reload
}
