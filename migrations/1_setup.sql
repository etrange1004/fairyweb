use fairydb;
create table user1 (
	id varchar(36) not null primary key,
    name varchar(30) not null,
    email varchar(100) not null,
    pw varchar(256) not null,
    date datetime not null default now(),
    permit tinyint unsigned default 0,
    bio text,
    pimg text
);
create table comment1 (
	number int unsigned not null auto_increment primary key,
    parent int unsigned not null,
    id varchar(36) not null,
    name varchar(30) not null,
    content text not null,
    date datetime not null default now()
);
create table board1 (
    number int unsigned not null auto_increment primary key,
    title varchar(150) not null,
    content text not null,
    id varchar(36) not null,
    name varchar(30) not null,
    password varchar(20) not null,
    date datetime not null default now(),
    hit int unsigned not null default 0
);