#!/usr/bin/env sh
systemfd --no-pid -s http::8000 -- cargo watch -x run
