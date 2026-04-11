CREATE TABLE IF NOT EXISTS "greenhouse_condition" (
        greenhouse_id UUID REFERENCES "greenhouse"(id) ON DELETE CASCADE NOT NULL,
        condition CONDITION NOT NULL,

        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        PRIMARY KEY(greenhouse_id, condition)
);