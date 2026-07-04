ALTER TABLE servers ADD COLUMN connected_at DATETIME;

UPDATE servers
SET connected_at = added_at
WHERE connected = TRUE;
