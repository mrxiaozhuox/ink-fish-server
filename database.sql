create table if not exists auth (
    id      varchar(128)    not null   primary key,
    user    int             not null,
    expire  int             not null
);

create table if not exists user (
    id              serial          not null    primary key,
    email           varchar(255)    default ''  unique,
    username        varchar(255)    not null,
    gender          varchar(10)     default 'unknown',
    reg_date        date            default current_date,
    recently        date            default current_date,
    password        varchar(255)    not null,
    salt            varchar(20)     not null,
    introduction    text            default '',
    avatar          varchar(255)    default '/assets/image/default_avatar.png',
    role            int             default 0
);

-- test data
insert into user (email, username, password, salt, avatar) values (
    'mrxzx@qq.com', 
    'mrxiaozhuox', 
    '22b48f7a98d9a8d684c7000dde01ef6e', 
    'ccIvwLYPegqy', 
    'https://avatars.githubusercontent.com/u/41265098?v=4'
);