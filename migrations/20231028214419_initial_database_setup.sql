-- Add migration script here
CREATE TABLE IF NOT EXISTS quotes (
  id UUID       PRIMARY KEY,
  book VARCHAR  NOT NULL,
  quote TEXT    NOT NULL
)
