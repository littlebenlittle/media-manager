#!/bin/bash

function serve {
    RUSTFLAGS=--cfg=web_sys_unstable_apis APP_BASE_PATH=media-manager trunk serve --features=demo
}
