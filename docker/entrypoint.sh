#!/bin/sh
set -eu

configure_pin_remote_service() {
    service=${MA_PIN_REMOTE_NAME:-}
    endpoint=${MA_PIN_REMOTE_API_URL:-}
    key=${MA_PIN_REMOTE_API_SECRET:-}

    if [ -z "$service" ] && [ -z "$endpoint" ] && [ -z "$key" ]; then
        return
    fi

    if [ -z "$service" ] || [ -z "$endpoint" ] || [ -z "$key" ]; then
        echo "MA_PIN_REMOTE_NAME, MA_PIN_REMOTE_API_URL, and MA_PIN_REMOTE_API_SECRET must be set together" >&2
        exit 1
    fi

    kubo_url=${MA_KUBO_RPC_URL:-http://kubo:5001}
    services=$(curl --fail --silent --show-error --retry 10 --retry-connrefused \
        --get --data-urlencode "arg=$service" \
        "$kubo_url/api/v0/pin/remote/service/ls")

    if printf '%s' "$services" | grep -Fq "\"Service\":\"$service\""; then
        return
    fi

    curl --fail --silent --show-error \
        --post301 --post302 --post303 \
        --data-urlencode "arg=$service" \
        --data-urlencode "arg=$endpoint" \
        --data-urlencode "arg=$key" \
        "$kubo_url/api/v0/pin/remote/service/add" >/dev/null
}

configure_pin_remote_service
exec ma "$@"