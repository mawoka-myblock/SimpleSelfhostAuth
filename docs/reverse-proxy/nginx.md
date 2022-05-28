

# Nginx

SSA works best with Nginx.
Below is an example-nginx-config.
The highlighted lines need extra attention.

```nginx hl_lines="23 31"

server {
    listen 80;
    server_name _;
    types_hash_max_size 1024;


    location / {
        root /usr/share/nginx/html; # (1)
        index index.html index.htm;
        
        # SSA-Start (2)
        auth_request /auth; # (3)
        auth_request_set $auth_cookie $upstream_http_set_cookie;
        add_header Set-Cookie $auth_cookie;
        error_page 401 = @proxy_signin; # (4) 
        auth_request_set $auth_cookie $upstream_http_set_cookie;
        add_header Set-Cookie $auth_cookie;
        # SSA-End

    }
    location = /auth {
        internal;
        proxy_pass http://localhost:8080; # (5)
        proxy_pass_request_body off;
        proxy_set_header Content-Length "";
        proxy_set_header X-Original-URI $request_uri;
        proxy_set_header X-Original-Remote-Addr $remote_addr;
        proxy_set_header X-Original-Host $host;
    }
    location /account { # (6)
        proxy_pass http://localhost:8080; # (7)
        proxy_set_header X-Original-URI $request_uri;
        proxy_set_header X-Original-Remote-Addr $remote_addr;
        proxy_set_header X-Original-Host $host;
    }
    location @proxy_signin { # (8)
        internal;
        add_header Set-Cookie $auth_cookie;
        return 302 /account/login?return_to=$request_uri;
    }

}
```

1. Add your normal stuff here.
2. Here starts the configuration for SSA. You don't have to change anything here.
3. That tells Nginx where it should check if the user is logged in.
4. This line tells Nginx that the error-page for the code `401` (unauthorized) is another server-block.
5. Replace it with the URL for SSA.
6. This is the block which gets passed to SSA. You may not change the name (or the URL (/account/)).
7. Replace it with the URL for SSA.
8. This block is required.