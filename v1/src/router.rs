use crate::controller;
use crate::RouterRegister;
use controller::user_players::get_player_collection_data;
use std::sync::Arc;

pub fn build_routers() -> Arc<RouterRegister> {
    let mut routers = RouterRegister::new();
    routers.add(1000, controller::server_list::get_server_list);
    routers.add(1001, controller::users::create_user);
    routers.add(1002, controller::user_equipments::get_user_equipment_list);
    routers.add(1003, controller::user_players::get_user_default_player_data);
    routers.add(1004, controller::equipment_kinds::get_equipment_kind_list);
    routers.add(1005, controller::equipments::get_equipment_list_by_kid);
    routers.add(1006, get_player_collection_data);
    routers.add(1007, controller::enemys::get_enemy_list_data);
    routers.add(1008, controller::users::get_user_base_info);
    routers.add(1009, controller::user_assets::get_user_asset_info);
    routers.add(1010, controller::user_assets::update_user_asset_info);
    routers.add(1011, controller::users::update_user_action_force);
    routers.add(
        1012,
        controller::user_player_track::insert_or_update_player_track,
    );
    routers.add(1013, controller::user_player_track::get_player_track);
    routers.add(
        1014,
        controller::player_mount_equipments::mount_user_player_equipment,
    );
    routers.add(
        1015,
        controller::player_mount_equipments::umount_user_player_equipment,
    );
    routers.add(
        1016,
        controller::player_mount_equipments::switch_user_player_equipment,
    );
    routers.add(1017, controller::users::update_user_name);
    routers.add(1018, controller::user_equipments::user_buy_equipment);

    Arc::new(routers)
}
