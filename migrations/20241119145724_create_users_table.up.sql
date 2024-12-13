-- Add up migration script here
CREATE TYPE user_status AS ENUM ('ACTIVE', 'DELETED');

CREATE TABLE
    users (
        id UUID DEFAULT gen_random_uuid () PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        email VARCHAR(255) UNIQUE NOT NULL,
        password VARCHAR(255) NOT NULL,
        status user_status DEFAULT 'ACTIVE' NOT NULL,
        created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
        deleted_at TIMESTAMPTZ
    );