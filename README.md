[![License BSD-2-Clause](https://img.shields.io/badge/License-BSD--2--Clause-blue.svg)](https://opensource.org/licenses/BSD-2-Clause)
[![License MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)

# `haproxy_autconf`
Welcome to `haproxy_autconf` 🎉

`haproxy_autconf` is a container that manages automatic backend registration and deregistration. It is designed to be
run together with your real backend services so that they are registered automatically once you call `docker-compose up`
and deregistered if the services go down.

## Example
See the [Docker-Compose.yml](Docker-Compose.yml) file as example.
