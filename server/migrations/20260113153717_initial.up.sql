-- Add migration script here

drop table if exists "user";
create table "user" (
	id SERIAL NOT NULL PRIMARY KEY
);

drop table if exists "api_key";
create table "api_key" (
	id SERIAL NOT NULL PRIMARY KEY,
	user_id INT NOT NULL REFERENCES user (id) ON DELETE CASCADE,
	description TEXT,
	prefix VARCHAR(7) NOT NULL,
	hash_info VARCHAR(10) NOT NULL,
	hash VARCHAR(70) NOT NULL UNIQUE,

	UNIQUE (user_id, prefix)
);

drop table if exists "image";
create table "image" (
	id SERIAL NOT NULL PRIMARY KEY,
	path TEXT NOT NULL
);

drop table if exists "artist";
create table "artist" (
	id SERIAL NOT NULL PRIMARY KEY,
	name VARCHAR NOT NULL,
	starred DATE,
	image_id INT REFERENCES image (id),
	music_brainz_id VARCHAR(36)
);

drop table if exists "album";
create table "album" (
	id SERIAL NOT NULL PRIMARY KEY,
	name VARCHAR NOT NULL,
	music_brainz_id VARCHAR(36),
	main_artist INT REFERENCES artist (id),
	added_date DATE NOT NULL,
	publish_date DATE,
	album_picture TEXT,
);


drop table if exists "album_contributor";
create table "album_contributor" (
	artist_id INT NOT NULL REFERENCES artist (id),
	album_id INT NOT NULL REFERENCES album (id),
	PRIMARY KEY (artist_id, album_id)
);



drop table if exists "child";
