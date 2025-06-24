BEGIN;

CREATE TABLE
    todos (
        "id" SERIAL PRIMARY KEY,
        "topic" VARCHAR(64) NOT NULL,
        "completed" BOOLEAN DEFAULT false,
        "completed_at" TIMESTAMP,
        "created_at" TIMESTAMP NOT NULL,
        "updated_at" TIMESTAMP NOT NULL
    );

COMMIT;