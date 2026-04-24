DO $$
BEGIN
    IF NOT EXISTS (
        SELECT 1
        FROM pg_constraint
        WHERE conname = 'todos_title_not_blank'
          AND conrelid = 'todos'::regclass
    ) THEN
        ALTER TABLE todos
        ADD CONSTRAINT todos_title_not_blank
        CHECK (char_length(btrim(title)) > 0);
    END IF;
END
$$;

CREATE INDEX IF NOT EXISTS idx_todos_created_at_desc
ON todos (created_at DESC);
