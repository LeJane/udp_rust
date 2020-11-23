CREATE table users
(
    uuid BIGINT PRIMARY KEY,
    uid INTEGER NOT NULL,
    name VARCHAR(500) NOT NULL,
    avatar VARCHAR(500) NOT NULL,
    login_days SERIAL NOT NULL,
    server_id INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now(),
    action_force INTEGER NOT NULL DEFAULT 1000,
);

CREATE table user_assets
(
    asid BIGINT PRIMARY KEY,
    uid INTEGER NOT NULL,
    golds INTEGER NOT NULL DEFAULT 0,
    diamonds BIGINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


CREATE table link_accounts
(
    lid BIGINT PRIMARY KEY,
    uuid BIGINT NOT NULL,
    account_type SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table servers
(
    sid BIGINT PRIMARY KEY,
    server_number INTEGER NOT NULL,
    name VARCHAR(500) NOT NULL,
    ip VARCHAR(500) NOT NULL,
    ports SMALLINT NOT NULL,
    person_count INTEGER NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

INSERT INTO servers
    (sid,server_number,name,ip,ports,person_count)
VALUES(6829118970656486265, 1001, 'localhost-mache', '127.0.0.1', 4433, 1);

CREATE table friends
(
    fid BIGINT PRIMARY KEY,
    uuid_a BIGINT NOT NULL,
    uuid_b BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table blacklists
(
    bid BIGINT PRIMARY KEY,
    uuid_a BIGINT NOT NULL,
    uuid_b BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table chat_groups
(
    gid BIGINT PRIMARY KEY,
    group_name VARCHAR(500) NOT NULL,
    group_thumbnail VARCHAR(500) NOT NULL,
    uuid BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table chat_groups_uids
(
    guid BIGINT PRIMARY KEY,
    gid BIGINT NOT NULL,
    uuid BIGINT NOT NULL,
    latest_timestamp BIGINT NOT NULL,
    unread_count SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table user_chat_unread_counts
(
    ucid BIGINT PRIMARY KEY,
    uuid_s BIGINT NOT NULL,
    uuid_d BIGINT NOT NULL,
    latest_timestamp BIGINT NOT NULL,
    unread_count SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table chat_messages
(
    mid BIGINT PRIMARY KEY,
    send_id BIGINT NOT NULL,
    to_id BIGINT NOT NULL,
    content VARCHAR(500) NOT NULL,
    created_timestamp BIGINT NOT NULL,
    kind SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

CREATE table server_lists
(
    slid BIGINT PRIMARY KEY,
    name VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    area VARCHAR(50) NOT NULL,
    ip VARCHAR(200) NOT NULL,
    port SMALLINT NOT NULL,
    server_type SMALLINT NOT NULL,
    state SMALLINT NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

INSERT INTO server_lists
    (slid,name,country_code,area,ip,port,server_type,state)
VALUES(7817487988393814824, 'localhost-mache', '86', 'ch', '192.168.1.129', 4434, 2, 0);


---Match3RPG

drop table enemys;
drop table players;
drop table skills;
drop table skill_fight_relateds;
drop table equipment_kinds;
drop table equipments;
drop table gems;
drop table gem_relateds;
drop table player_mount_equipments;
drop table user_players;
drop table user_equipments;
drop table user_player_tracks;



delete from users;
delete from user_assets;
delete from link_accounts;
delete from enemys;
delete from players;
delete from skills;
delete from skill_fight_relateds;
delete from equipment_kinds;
delete from equipments;
delete from gems;
delete from gem_relateds;
delete from player_mount_equipments;
delete from user_players;
delete from user_equipments;
delete from user_player_tracks;


-- enemys
CREATE table enemys
(
    eid BIGINT PRIMARY KEY,
    enemy_name VARCHAR(200) NOT NULL,
    model_path VARCHAR(100) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    animation_hit_delay REAL NOT NULL,
    spawn_style_class VARCHAR(200) NOT NULL,
    bp_enemy VARCHAR(200) NOT NULL,
    ap_enemy VARCHAR(200) NOT NULL,
    skm_enemy VARCHAR(200) NOT NULL,
    aenemy_die VARCHAR(200) NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

-- players
CREATE table players
(
    pid BIGINT PRIMARY KEY,
    player_name VARCHAR(200) NOT NULL,
    model_path VARCHAR(100) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    animation_hit_delay REAL NOT NULL,
    spawn_style_class VARCHAR(200) NOT NULL,
    level SMALLINT NOT NULL,
    star_level SMALLINT NOT NULL,
    level_experience INTEGER NOT NULL,
    is_default SMALLINT NOT NULL DEFAULT 1,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--skills
CREATE table skills
(
    id BIGINT PRIMARY KEY,
    thumbnail VARCHAR(200) NOT NULL,
    skill_name VARCHAR(200) NOT NULL,
    skill_description VARCHAR(200) NOT NULL,
    model_path VARCHAR(200) NOT NULL,
    cool_down INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    mana_power INTEGER NOT NULL,
    level SMALLINT NOT NULL DEFAULT 0,
    level_experience INTEGER NOT NULL DEFAULT 0,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--skill_fight_relateds
CREATE table skill_fight_relateds
(
    id BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    skill_id BIGINT NOT NULL,
    cool_down INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    mana_power INTEGER NOT NULL,
    probability SMALLINT NOT NULL DEFAULT 0,
    level SMALLINT NOT NULL DEFAULT 0,
    level_experience INTEGER NOT NULL DEFAULT 0,
    obj_type SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--equipment kinds
CREATE table equipment_kinds
(
    kid BIGINT PRIMARY KEY,
    name VARCHAR(200) NOT NULL,
    kind SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--equipments
CREATE table equipments
(
    eid BIGINT PRIMARY KEY,
    kid BIGINT NOT NULL,
    name VARCHAR(200) NOT NULL,
    thumbnail VARCHAR(200) NOT NULL,
    price INTEGER NOT NULL,
    hp INTEGER NOT NULL,
    multiplier REAL NOT NULL,
    kind SMALLINT NOT NULL,
    is_default SMALLINT NOT NULL DEFAULT 1,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--gems
CREATE table gems
(
    gid BIGINT PRIMARY KEY,
    gem_icon VARCHAR(200) NOT NULL,
    gem_selected_material VARCHAR(200) NOT NULL,
    gem_link_material VARCHAR(200) NOT NULL,
    model_path VARCHAR(200) NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);



--gem_relateds
CREATE table gem_relateds
(
    grid BIGINT PRIMARY KEY,
    obj_id BIGINT NOT NULL,
    gid BIGINT NOT NULL,
    obj_type SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--player_mount_equipments
CREATE table player_mount_equipments
(
    id BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    equipment_id BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--user_players
CREATE table user_players
(
    id BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    max_hp INTEGER NOT NULL,
    attack_power INTEGER NOT NULL,
    move_speed REAL NOT NULL,
    max_mana INTEGER NOT NULL,
    defense INTEGER NOT NULL,
    level SMALLINT NOT NULL,
    star_level SMALLINT NOT NULL,
    level_experience INTEGER NOT NULL,
    is_default SMALLINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);


--user_equipments
CREATE table user_equipments
(
    id BIGINT PRIMARY KEY,
    eid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);

--user_player_tracks
CREATE table user_player_tracks
(
    tid BIGINT PRIMARY KEY,
    pid BIGINT NOT NULL,
    uid BIGINT NOT NULL,
    rotaion_x REAL NOT NULL,
    rotaion_y REAL NOT NULL,
    rotaion_z REAL NOT NULL,
    location_x REAL NOT NULL,
    location_y REAL NOT NULL,
    location_z REAL NOT NULL,
    modify_time TIMESTAMP NOT NULL DEFAULT now(),
    created_time TIMESTAMP NOT NULL DEFAULT now()
);