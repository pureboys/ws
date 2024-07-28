drop table if exists teacher;

create table teacher
(
    id serial primary key,
    name varchar(100) not null,
    picture_url varchar(200) not null,
    profile varchar(2000) not null,
)
