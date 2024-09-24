Projeto simples com actix e minjinja e rodando no servidor EC2 da AWS

comandos:
- cargo build --release
- cargo run

OBS: Se o IP da AWS estiver rodando nginx no navegador quer dizer q é config padrao 
da pasta /etc/nginx/nginx.conf na porta 80

OBS2: Caso queira criar projeto do zero: cargo new actix_minijinja_project --bin
