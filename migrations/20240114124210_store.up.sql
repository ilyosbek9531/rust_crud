-- Add up migration script here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE users (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(50) UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE categories (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    category_name VARCHAR(50) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE products (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    product_name VARCHAR(100) NOT NULL UNIQUE,
    price NUMERIC(19, 2) NOT NULL,
    category_id UUID NOT NULL REFERENCES categories(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE ratings (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    rating INT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    product_id UUID NOT NULL REFERENCES products(id),
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE purchases (
    id UUID PRIMARY KEY NOT NULL DEFAULT (uuid_generate_v4()),
    product_id UUID NOT NULL REFERENCES products(id),
    user_id UUID NOT NULL REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);