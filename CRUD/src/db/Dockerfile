# utilizando a ultima versão da imagem do Postgres
FROM postgres:latest

# definindo as variaveis de ambiente
ENV POSTGRES_USER=grupo_do_henrique
ENV POSTGRES_PASSWORD=cucuz_de_frango_com_arroz_e_bom
ENV POSTGRES_DB=CRUD_com_rust

# Instanciando banco de dados
COPY init.sql /docker-entrypoint-initdb.d/
# Copiando o init.sql na pasta do postgres para inicialização do DB

# porta padrão do Postgres
EXPOSE 5432
