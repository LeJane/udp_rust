table! {
    blacklists (bid) {
        bid -> Int8,
        uuid_a -> Int8,
        uuid_b -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_groups (gid) {
        gid -> Int8,
        group_name -> Varchar,
        group_thumbnail -> Varchar,
        uuid -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_groups_uids (guid) {
        guid -> Int8,
        gid -> Int8,
        uuid -> Int8,
        latest_timestamp -> Int8,
        unread_count -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    chat_messages (mid) {
        mid -> Int8,
        send_id -> Int8,
        to_id -> Int8,
        content -> Varchar,
        created_timestamp -> Int8,
        kind -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    enemys (eid) {
        eid -> Int8,
        enemy_name -> Varchar,
        model_path -> Varchar,
        thumbnail -> Varchar,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        animation_hit_delay -> Float4,
        spawn_style_class -> Varchar,
        bp_enemy -> Varchar,
        ap_enemy -> Varchar,
        skm_enemy -> Varchar,
        aenemy_die -> Varchar,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    equipment_kinds (kid) {
        kid -> Int8,
        name -> Varchar,
        kind -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    equipments (eid) {
        eid -> Int8,
        kid -> Int8,
        name -> Varchar,
        thumbnail -> Varchar,
        price -> Int4,
        hp -> Int4,
        multiplier -> Float4,
        kind -> Int2,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    friends (fid) {
        fid -> Int8,
        uuid_a -> Int8,
        uuid_b -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    gem_relateds (grid) {
        grid -> Int8,
        obj_id -> Int8,
        gid -> Int8,
        obj_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    gems (gid) {
        gid -> Int8,
        gem_icon -> Varchar,
        gem_selected_material -> Varchar,
        gem_link_material -> Varchar,
        model_path -> Varchar,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    link_accounts (lid) {
        lid -> Int8,
        uuid -> Int8,
        account_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    player_mount_equipments (id) {
        id -> Int8,
        pid -> Int8,
        uid -> Int8,
        equipment_id -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    players (pid) {
        pid -> Int8,
        player_name -> Varchar,
        model_path -> Varchar,
        thumbnail -> Varchar,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        animation_hit_delay -> Float4,
        spawn_style_class -> Varchar,
        level -> Int2,
        star_level -> Int2,
        level_experience -> Int4,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    server_lists (slid) {
        slid -> Int8,
        name -> Varchar,
        country_code -> Varchar,
        area -> Varchar,
        ip -> Varchar,
        port -> Int2,
        server_type -> Int2,
        state -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    servers (sid) {
        sid -> Int8,
        server_number -> Int4,
        name -> Varchar,
        ip -> Varchar,
        ports -> Int2,
        person_count -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    skill_fight_relateds (id) {
        id -> Int8,
        obj_id -> Int8,
        skill_id -> Int8,
        cool_down -> Int4,
        attack_power -> Int4,
        mana_power -> Int4,
        probability -> Int2,
        level -> Int2,
        level_experience -> Int4,
        obj_type -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    skills (id) {
        id -> Int8,
        thumbnail -> Varchar,
        skill_name -> Varchar,
        skill_description -> Varchar,
        model_path -> Varchar,
        cool_down -> Int4,
        attack_power -> Int4,
        mana_power -> Int4,
        level -> Int2,
        level_experience -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_assets (asid) {
        asid -> Int8,
        uid -> Int8,
        golds -> Int4,
        diamonds -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_chat_unread_counts (ucid) {
        ucid -> Int8,
        uuid_s -> Int8,
        uuid_d -> Int8,
        latest_timestamp -> Int8,
        unread_count -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_equipments (id) {
        id -> Int8,
        eid -> Int8,
        uid -> Int8,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_players (id) {
        id -> Int8,
        pid -> Int8,
        uid -> Int8,
        max_hp -> Int4,
        attack_power -> Int4,
        move_speed -> Float4,
        max_mana -> Int4,
        defense -> Int4,
        level -> Int2,
        star_level -> Int2,
        level_experience -> Int4,
        is_default -> Int2,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    user_player_tracks (tid) {
        tid -> Int8,
        pid -> Int8,
        uid -> Int8,
        rotaion_x -> Float4,
        rotaion_y -> Float4,
        rotaion_z -> Float4,
        location_x -> Float4,
        location_y -> Float4,
        location_z -> Float4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
    }
}

table! {
    users (uuid) {
        uuid -> Int8,
        uid -> Int4,
        name -> Varchar,
        avatar -> Varchar,
        login_days -> Int4,
        server_id -> Int4,
        modify_time -> Timestamp,
        created_time -> Timestamp,
        action_force -> Int4,
    }
}

allow_tables_to_appear_in_same_query!(
    blacklists,
    chat_groups,
    chat_groups_uids,
    chat_messages,
    enemys,
    equipment_kinds,
    equipments,
    friends,
    gem_relateds,
    gems,
    link_accounts,
    player_mount_equipments,
    players,
    server_lists,
    servers,
    skill_fight_relateds,
    skills,
    user_assets,
    user_chat_unread_counts,
    user_equipments,
    user_players,
    user_player_tracks,
    users,
);
