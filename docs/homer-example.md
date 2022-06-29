# Homer-Example

At first, you'll need docker and nginx installed.

We'll have the following domain as a placeholder: `my-stuff.home` and our goal is that homer should be secureley available at `homer.my-stuff.home`!

### 1: Setup Homer
Create a new directory and create a file called `docker-compose.yml` with the following content:
```yaml
version: "2"
services:
  homer:
    image: b4bz/homer
    #To build from source, comment previous line and uncomment below
    #build: .
    container_name: homer
    volumes:
      - ./assets:/www/assets
    ports:
      - 8081:8080
    environment:
      - INIT_ASSETS=1 # default
```
Start the stack with `docker compose up -d`
You should be able to access homer now at `my-stuff.home:8081`

### 2: Setup SSA
Create a new directory and run the following command to download the `docker-compose.yml` and generate a secret: 
```shell
wget -q docker-compose.yml https://raw.githubusercontent.com/mawoka-myblock/SimpleSelfhostAuth/master/docker-compose.yml; sed -i "s/NOT_SECRET/$(openssl rand -hex 32)/g" docker-compose.yml
```
Next, start the containers: `docker compose up -d` and visit SSA: `my-stuff.home:8080`. You can log in with the username and password `admin`. At first, change your password by clicking the "Change Password"-button. After that, go into the "Admin"-tab and there into the Apps-tab and click on "Create App".
Enter a name and an optional description for the app. You don't have to change the "Token Lifetime"but you'll have to add a domain who it is described in the picture.
![](/assets/upload_9cfda2a3364311af058b5aa7f3513792.png)
Now, click on the blue "Create"-button.
Note: if the app isn't in the list, refresh the page.


### 3: Setup Nginx

Create a new Nginx-config-file:

```nginx
server {
    listen 80;
    server_name homer.my-stuff.home;
    types_hash_max_size 1024;

    location / {
        proxy_pass http://localhost:8081; # URL of Homer

        # SSA-Start
        auth_request /auth; #
        auth_request_set $auth_cookie $upstream_http_set_cookie;
        add_header Set-Cookie $auth_cookie;
        error_page 401 = @proxy_signin; #
        auth_request_set $auth_cookie $upstream_http_set_cookie;
        add_header Set-Cookie $auth_cookie;
        # SSA-End

    }
    location = /auth {
        internal;
        proxy_pass http://localhost:8080; # URL of SSA
        proxy_pass_request_body off;
        proxy_set_header Content-Length "";
        proxy_set_header X-Original-URI $request_uri;
        proxy_set_header X-Original-Remote-Addr $remote_addr;
        proxy_set_header X-Original-Host $host;
    }
    location /account { #
        proxy_pass http://localhost:8080; # URL of SSA
        proxy_set_header X-Original-URI $request_uri;
        proxy_set_header X-Original-Remote-Addr $remote_addr;
        proxy_set_header X-Original-Host $host;
    }
    location @proxy_signin { #
        internal;
        add_header Set-Cookie $auth_cookie;
        return 302 /account/login?return_to=$request_uri;
    }

}
```
That's it! Go to `homer.my-stuff.home` and sign in with your credentials!