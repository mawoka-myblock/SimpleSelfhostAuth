-- This file should undo anything in `up.sql`
DROP TABLE if exists users cascade;
DROP TABLE if exists apps cascade;
DROP TYPE if exists two_factor_type;