-- This file should undo anything in `up.sql`

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
ALTER TABLE categories ALTER COLUMN id TYPE UUID;
ALTER TABLE categories ALTER COLUMN id SET DEFAULT uuid_generate_v4();