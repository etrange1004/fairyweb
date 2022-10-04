use fairydb;
create table user (
	id varchar(36) not null primary key,
    name varchar(30) not null,
    email varchar(100) not null,
    pw varchar(256) not null,
    date datetime not null default now(),
    permit tinyint unsigned default 0,
    bio text,
    pimg text
);
create table comment (
	number int unsigned not null auto_increment primary key,
    parent int unsigned not null,
    id varchar(36) not null,
    name varchar(30) not null,
    content text not null,
    date datetime not null default now()
);
create table board (
    number int unsigned not null auto_increment primary key,
    title varchar(150) not null,
    content text not null,
    id varchar(36) not null,
    name varchar(30) not null,
    password varchar(20) not null,
    date datetime not null default now(),
    hit int unsigned not null default 0
);
create table user (
	id_bin binary(16) not null primary key,
    id_text varchar(36) generated always as
	(insert(
		insert(
			insert(
				insert(hex(id_bin),9,0,'-'),
			14,0,'-'),
		19,0,'-'),
	24,0,'-')
	) virtual,
    name varchar(30) not null,
    email varchar(100) not null,
    pw text not null,
    date timestamp,
    permit tinyint unsigned,
    bio text,
    pimg text
);

insert into user (id_bin, name, email, pw, date, permit, bio, pimg)
values (UUID_TO_BIN(UUID()), 'uncleoppa', 'uncleoppafairy@hotmail.com', '0000', now(), 8, 'fairy from sangdodong', '');

select * from user;

CREATE TABLE OrderDetails(
   OrderId BINARY(16) PRIMARY KEY,
   ProductName VARCHAR(100) NOT NULL,
   Price DECIMAL(10, 2) NOT NULL,
   ExpectedDelivery DATE NOT NULL
);

select last_insert_id();
select * from user order by date;
select * from board order by date desc;
select number, title, id, name, date, hit from board order by date desc;
drop table user;
select count(*) from board order by date desc;
select * from (select * from  board order by date asc limit 5) as dbfairy order by date desc;
select * from  board where title like '%요정%' or content like '%차차%' order by date desc limit 0, 5;
select number, title, id, name, date, hit from board where title like '%하기%' or content like '%하기%'
order by date desc limit 0, 15;
select  count(*) as rec_count from board where title like '%하기%' or content like '%하기%';
insert into user (id, name, email, pw, permit, bio, pimg)
values (uuid(), 'Guest', 'guest@fairyland.com', '0000', '0', 'anonymous fairy', '');