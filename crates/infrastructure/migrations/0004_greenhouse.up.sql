CREATE TABLE IF NOT EXISTS "greenhouse" (
        id UUID PRIMARY KEY NOT NULL,
        name TEXT UNIQUE NOT NULL,
        humidity UINT CHECK ( humidity <= 100 ) NOT NULL,
        area_square_meters UREAL NOT NULL,
        target_temperature_celsius UREAL NOT NULL,

        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
        deleted_at TIMESTAMP DEFAULT NULL
);