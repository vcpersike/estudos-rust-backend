# Estágio de construção: Use a imagem base oficial do Rust para compilar o código-fonte
FROM rust:latest as builder

# Cria o diretório de trabalho
WORKDIR /usr/src/estudos-rust-backend

# Copia os arquivos de configuração do Cargo para o diretório de trabalho
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock

# Cria um projeto fictício para compilar as dependências separadamente,
# o que melhora o aproveitamento do cache do Docker
RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/estudos_rust_backend*

# Copia o código-fonte real para o diretório de trabalho
COPY ./src ./src

# Compila a aplicação em modo de release
# Os artefatos da compilação ficam no diretório target/release/
RUN cargo build --release

# Estágio de execução: Usa a imagem Debian Bullseye slim para reduzir o tamanho da imagem final
FROM ubuntu:latest

# Instala as dependências necessárias
RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates \
    && rm -rf /var/lib/apt/lists/*


# Copia o binário compilado para o diretório /usr/local/bin
COPY --from=builder /usr/src/estudos-rust-backend/target/release/estudos-rust-backend /usr/local/bin/estudos-rust-backend

EXPOSE 8080

CMD ["/usr/local/bin/estudos-rust-backend"]
