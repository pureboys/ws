drop table if exists course;

create table course
(
    id serial primary key,
    teacher_id INT not null,
    name varchar(140) not null,
    time TIMESTAMP default now(),
    description varchar(140) not null,
    format varchar(30) not null,
    structure varchar(200),
    duration varchar(30) not null,
    price INT not null,
    language varchar(30) not null,
    level varchar(30) not null,
);
