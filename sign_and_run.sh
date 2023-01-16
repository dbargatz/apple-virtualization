#!/usr/bin/env zsh
codesign -f --entitlement $(PWD)/virtualization.entitlements -s - "$1"
exec "$@"