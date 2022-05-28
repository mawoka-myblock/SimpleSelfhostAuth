# SimpleSelfhostAuth

Your reverse-proxy auth-service!



## What is SimpleSelfhostAuth (SSA)?

SSA is something like [Authelia](https://www.authelia.com/), but (in my opinion) simpler to set up.
SSA needs a reverse proxy to run.

**tl;dr**
> Got a reverse proxy and want to build SSO into it? Then SSA is the right thing for you.

> You want more features? Have a look at [authentik](https://goauthentik.io/) or [Authelia](https://www.authelia.com/).

## Features
- Simple
- Isn't very resource-intensive
- Bug-free[^1]
- Supports 2FA (TOTP)
- Easy to deploy via docker

## Get started
Have a look at the [docs](https://ssa.mawoka.eu/quickstart/).

[^1]: The backend is written with [Rust](https://rust-lang.org)