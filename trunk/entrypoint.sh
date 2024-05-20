#!/bin/bash

IP=$(getent hosts trunk | cut -d ' ' -f 1)
trunk serve --address "$IP" --verbose
