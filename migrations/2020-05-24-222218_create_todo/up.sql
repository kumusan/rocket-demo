CREATE TABLE todos (
  id INTEGER PRIMARY KEY,
  title VARCHAR NOT NULL,
  body TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT 'f'
)
