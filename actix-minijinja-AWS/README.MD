Projeto simples com actix e minjinja e rodando dentro do servidor EC2 da AWS

comandos:
- cargo build --release
- cargo run

OBS: Se o IP da AWS estiver rodando nginx no navegador quer dizer q é config padrao 
da pasta /etc/nginx/nginx.conf na porta 80

OBS2: Caso queira criar projeto do zero: cargo new actix_minijinja_project --bin


```nginx server { listen 80; listen [::]:80; server_name 3.84.37.42;
-Configurando o proxy reverso para o Actix Web
location / {
    proxy_pass http://127.0.0.1:8080;  # Porta onde o Actix Web está rodando
    proxy_set_header Host $host;
    proxy_set_header X-Real-IP $remote_addr;
    proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
    proxy_set_header X-Forwarded-Proto $scheme;
}

error_page 404 /404.html;
location = /404.html {
}

error_page 500 502 503 504 /50x.html;
location = /50x.html {
}
} ```
