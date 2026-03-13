CREATE TABLE IF NOT EXISTS "employee" (
        id UUID PRIMARY KEY NOT NULL,
        name TEXT NOT NULL,
        surname TEXT NOT NULL,
        patronymic TEXT DEFAULT NULL,
        currency CURRENCY NOT NULL,
        amount U32 NOT NULL,
        works_since DATE NOT NULL DEFAULT CURRENT_DATE,
        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        deleted_at TIMESTAMP DEFAULT NULL
);