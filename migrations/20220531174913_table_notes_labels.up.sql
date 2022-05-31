-- Add up migration script here
-- Add up migration script here
CREATE TABLE IF NOT EXISTS notes_labels(
    id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    note_id INTEGER NOT NULL,
    label_id INTEGER NOT NULL,
);
