# Blockchain API em Rust

Este projeto é uma implementação básica de um **Blockchain** em **Rust**, com uma API para interagir com ele. A API permite:

1. **Consultar a blockchain**: Visualizar todos os blocos existentes na cadeia.
2. **Inserir um novo bloco**: Submeter um hash válido (que satisfaça as condições de mineração) para adicionar um novo bloco à blockchain.
3. **Gerenciar transações**: Criar novas transações e visualizar o mempool de transações pendentes.

---

## 🛠 Pré-requisitos

Certifique-se de ter instalado:

- [Rust](https://www.rust-lang.org/tools/install)
- Cargo
- Docker

---

## 🚀 Como executar o projeto

### 1. Clone o repositório

```bash
git clone https://github.com/pedrolucas27/blockchain-api.git
cd blockchain-api
```

### 2. Configure o Redis

A API utiliza o **Redis**, um banco de dados NoSQL, para gerenciar transações e o mempool. Para configurar o Redis:

1. Inicie uma instância do Redis usando Docker:

   ```bash
   docker run --name redis-db -p 6379:6379 -d redis
   ```

   - **`--name redis-db`**: Define o nome do contêiner como `redis-db`.
   - **`-p 6379:6379`**: Mapeia a porta 6379 do contêiner para a porta 6379 do host.
   - **`-d redis`**: Baixa e executa a imagem oficial do Redis em segundo plano.

2. Verifique se o Redis está em execução:

   ```bash
   docker ps
   ```

3. Certifique-se de que o Redis está acessível na URL padrão: `redis://127.0.0.1:6379`.

### 3. Execute o projeto

Use o Cargo para executar o código:

```bash
cargo run
```

---

## 📚 Documentação das Rotas

### **GET /chain**

Retorna a cadeia de blocos atual.

### **POST /mine**

Minera um novo bloco.

### **GET /transactions/mempool**

Retorna o mempool de transações pendentes.

### **POST /transactions/create**

Adiciona uma nova transação ao mempool da blockchain.

- **Parâmetros**:
  - `sender` (String): Endereço do remetente.
  - `recipient` (String): Endereço do destinatário.
  - `amount` (float): Valor da transação.
  - `priv_wif_key` (String): Chave privada do remetente.
- **Requisição**:
  - Corpo (JSON):
    ```json
    {
      "sender": "Alice",
      "recipient": "Bob",
      "amount": 15.0,
      "priv_wif_key": "private_key_example"
    }
    ```

---

## 🧑‍💻 Contribuidores

- Alisson Diogo Soares de Souza
- Pedro Lucas Farias Figueiredo de Medeiros
- Victor Gabriel Oliveira do Couto
