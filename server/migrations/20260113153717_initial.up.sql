-- Add migration script here

drop table if exists "users";
create table "users" (
	id SERIAL NOT NULL PRIMARY KEY
);

drop table if exists "api_keys";
create table "api_keys" (
	id SERIAL NOT NULL PRIMARY KEY,
	user_id INT NOT NULL REFERENCES users (id) ON DELETE CASCADE,
	description TEXT,
	prefix VARCHAR(7) NOT NULL,
	hash_info VARCHAR(10) NOT NULL,
	hash VARCHAR(70) NOT NULL UNIQUE,

	UNIQUE (user_id, prefix)
);
