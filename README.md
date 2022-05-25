# SimpleSelfhostAuth

Your reverse-proxy auth-service!

## What is SimpleSelfhostAuth (SSA)?

SSA is something like [Authelia](https://www.authelia.com/), but (in my opinion) simpler to set up.
SSA needs a reverse proxy to run.

**tl;dr**
> Got a reverse proxy and want to build SSO into it? Then SSA is the right thing for you.

> You want more features? Have a look at [authentik](https://goauthentik.io/) or [Authelia](https://www.authelia.com/).


## Get started
At first, make sure that all the requirements are installed.
### Requirements
- [Docker](https://docs.docker.com/get-docker/) ([-compose](https://docs.docker.com/compose/install/))
- Nginx or any other reverse proxy
- A public domain
- 100mb of free ram

