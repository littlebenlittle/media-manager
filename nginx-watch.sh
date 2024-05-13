#!/bin/bash

while inotifywait -e modify nginx.conf
    do podman-compose -f compose.yaml exec nginx nginx -s reload
done
