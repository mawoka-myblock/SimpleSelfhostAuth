# Quick Start

## Requirements

- A supported reverse proxy. Have a look at the [list of reverse proxies](/reverse-proxy)
- [Docker](https://docs.docker.com/get-docker/) ([-compose](https://docs.docker.com/compose/install/))
- A public domain (+ HTTPS)

## Download and install

1. Create a new directory and enter it: `mkdir SimpleSelfhostAuth && cd SimpleSelfhostAuth`
2. Next, download
   the [docker-compose.yml](https://github.com/mawoka-myblock/SimpleSelfhostAuth/blob/master/docker-compose.yml):
   ```shell
   wget -q docker-compose.yml https://raw.githubusercontent.com/mawoka-myblock/SimpleSelfhostAuth/master/docker-compose.yml
   ```
3. Add a secret key by running `sed -i "s/NOT_SECRET/$(openssl rand -hex 32)/g" docker-compose.yml`.
4. Start the stack: `docker compose up -d` (or, if you use the old docker-compose: `docker compose up -d`)
5. Now, open the admin-interface at `<YOUR_SSA_DOMAIN>` and sign in. The username is `admin` and the password is the
   same as the username.
6. **CHANGE YOUR PASSWORD!**
7. Create users, apps, etc.