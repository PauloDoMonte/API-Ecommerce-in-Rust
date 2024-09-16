-- Habilitar a extensão pgcrypto para gerar UUIDs
CREATE EXTENSION IF NOT EXISTS "pgcrypto";

-- Criação da tabela de usuários
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Identificador único da tabela
    username VARCHAR(255) NOT NULL UNIQUE, 
    email VARCHAR(255) NOT NULL UNIQUE,     
    password_hash VARCHAR(60) NOT NULL  
);

-- Criação da tabela de categorias
CREATE TABLE categories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Identificador único da categoria
    name VARCHAR(255) NOT NULL  -- Nome da categoria, obrigatório
);

-- Criação da tabela de produtos base
CREATE TABLE product_base (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Identificador único do produto base
    name VARCHAR(255) NOT NULL, -- Nome do produto
    price DECIMAL(10, 2) NOT NULL, -- Preço do produto base
    description TEXT,        -- Descrição do produto
    category_id UUID,       -- Chave estrangeira para a categoria
    CONSTRAINT fk_category
      FOREIGN KEY (category_id)
      REFERENCES categories (id) -- Relaciona com a tabela de categorias
);

-- Criação da tabela de variações de produtos
CREATE TABLE product_variation (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Identificador único da variação
    product_base_id UUID, -- Chave estrangeira para o produto base
    sku VARCHAR(100) NOT NULL UNIQUE, -- SKU da variação
    price DECIMAL(10, 2), -- Preço da variação (pode ser diferente do produto base)
    attributes JSONB, -- Atributos da variação (ex: cor, tamanho, etc.)
    CONSTRAINT fk_product_base
      FOREIGN KEY (product_base_id)
      REFERENCES product_base (id) -- Relaciona com a tabela de produtos base
);

-- Criação da tabela de promoções
CREATE TABLE promotions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(), -- Identificador único da promoção
    product_base_id UUID, -- Chave estrangeira para o produto base
    discount_percentage DECIMAL(5, 2) NOT NULL, -- Percentual de desconto
    start_date TIMESTAMP NOT NULL, -- Data de início da promoção
    end_date TIMESTAMP NOT NULL, -- Data de término da promoção
    CONSTRAINT fk_product_base_promotion
      FOREIGN KEY (product_base_id)
      REFERENCES product_base (id) -- Relaciona com a tabela de produtos base
);