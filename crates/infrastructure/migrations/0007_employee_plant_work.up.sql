CREATE TABLE IF NOT EXISTS "employee_plant_work" (
        id UUID PRIMARY KEY NOT NULL,
        employee_id UUID REFERENCES "employee"(id) ON DELETE NO ACTION NOT NULL,
        plant_id UUID REFERENCES "plant"(id) ON DELETE NO ACTION NOT NULL,
        work_type WORK_TYPE NOT NULL,

        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);