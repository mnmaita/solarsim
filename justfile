[private]
default:
  just --list

# Valid values for the `app` parameter: client, server.
build app:
  just solarsim-{{app}}/build

# Valid values for the `app` parameter: client, server.
run app:
  just solarsim-{{app}}/run

# Valid values for the `app` parameter: client, server.
dev app:
  just solarsim-{{app}}/dev
