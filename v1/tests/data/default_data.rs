extern crate lazy_static;
use v1::models::enemys::NewEnemy;
use v1::models::equipment_kinds::NewEquipmentKind;
use v1::models::equipments::NewEquipment;
use v1::models::gem_relateds::NewGemRelated;
use v1::models::gems::NewGem;
use v1::models::players::NewPlayer;
use v1::models::skill_fight_relateds::NewSkillFigthRelated;
use v1::models::skills::NewSkill;
use v1::utils::helper::get_guid_value;

pub fn get_default_data() -> (
    Vec<NewSkill<'static>>,
    Vec<NewEnemy<'static>>,
    Vec<NewGem<'static>>,
    Vec<NewEquipmentKind<'static>>,
    Vec<NewEquipment<'static>>,
    Vec<NewPlayer<'static>>,
    Vec<NewSkillFigthRelated>,
    Vec<NewGemRelated>,
) {
    let skills: Vec<NewSkill> = vec![
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Skill/Icon_Refresh",
            skill_name: "Refresh Board",
            skill_description: "Refreshes the board with new gems.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MRefreshBoard'",
            cool_down: 2,
            attack_power: 0,
            mana_power: 5,
            level: 1,
            level_experience: 1,
        },
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Skill/Skill_Treatment",
            skill_name: "Blood Return",
            skill_description: "Focuses and performs a precise attack.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MBloodReturn'",
            cool_down: 0,
            attack_power: 6,
            mana_power: 1,
            level: 1,
            level_experience: 1,
        },
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2",
            skill_name: "Three Combo",
            skill_description: "A weak attack performed by an average fighter.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MThreeCombo'",
            cool_down: 3,
            attack_power: 10,
            mana_power: 0,
            level: 0,
            level_experience: 0,
        },
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Skill/Skill_1",
            skill_name: "Single Attack",
            skill_description: "A weak attack performed by an average fighter.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MSingleAttack'",
            cool_down: 3,
            attack_power: 15,
            mana_power: 0,
            level: 0,
            level_experience: 0,
        },
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Skill/Skill_3",
            skill_name: "Jump Attack",
            skill_description: "Breaks 5 shield gems on the board.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MJumpAttack'",
            cool_down: 3,
            attack_power: 0,
            mana_power: 0,
            level: 0,
            level_experience: 0,
        },
        NewSkill {
            id: get_guid_value() as i64,
            thumbnail: "/Game/Commons/Textures/Skill/Skill_Mobs_1",
            skill_name: "Five Combo",
            skill_description: "Breaks 5 shield gems on the board.",
            model_path: "Class'/Script/TheForeAwakensCPlus.MFiveCombo'",
            cool_down: 3,
            attack_power: 0,
            mana_power: 0,
            level: 0,
            level_experience: 0,
        }
    ];
    let enemys: Vec<NewEnemy> = vec![
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Bat",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MBat",
            thumbnail: "/Game/Commons/Textures/Enemy/Bat",
            max_hp: 30,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.60000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MBat.MBat_C'", 
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Bat/ABP_Bat.ABP_Bat_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/Bat/Bat_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Bat/Die_Bat_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Dragon",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MDragon",
            thumbnail: "/Game/Commons/Textures/Enemy/Dragon",
            max_hp: 53,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.60000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MDragon.MDragon_C'",
            ap_enemy:
                "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Dragon/ABP_Dragon.ABP_Dragon_C'",
            skm_enemy:
                "/Game/RPGMonsterWavePBR/Meshes/Dragon/Dragon_SK",
            aenemy_die:
                "/Game/RPGMonsterWavePBR/Animations/Dragon/Die_Dragon_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Evilmage",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MEvilmage",
            thumbnail: "/Game/Commons/Textures/Enemy/Evilmage",
            max_hp: 56,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MEvilmage.MEvilmage_C'",
            ap_enemy:
                "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/EvilMage/OneMeshCharacter/ABP_EvilMage_OneMesh.ABP_EvilMage_OneMesh_C'",
            skm_enemy:
                "/Game/RPGMonsterWavePBR/Meshes/EvilMage/OneMeshCharacter/EvilMage_OneMesh_SK",
            aenemy_die:
                "/Game/RPGMonsterWavePBR/Animations/EvilMage/Die_EvilMage_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Ghoul",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul",
            thumbnail: "/Game/Commons/Textures/Enemy/Ghoul",
            max_hp: 30,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 6,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul.MGhoul_C'",
            ap_enemy: "AnimBlueprint'/Game/Ghoul_Set/Ghoul/ABP_Ghoul.ABP_Ghoul_C'",
            skm_enemy: "/Game/Ghoul_Set/Ghoul/SK_Ghoul",
            aenemy_die: "/Game/Ghoul_Set/Ghoul/Anims/Ghoul_Die",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Ghoul_Boss",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Boss",
            thumbnail: "/Game/Commons/Textures/Enemy/Ghoul_Boss",
            max_hp: 100,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 8,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Boss.MGhoul_Boss_C'",
            ap_enemy: "AnimBlueprint'/Game/Ghoul_Set/Ghoul_Boss/ABP_Ghoul_Boss.ABP_Ghoul_Boss_C'",
            skm_enemy: "/Game/Ghoul_Set/Ghoul_Boss/SK_Ghoul_Boss",
            aenemy_die: "/Game/Ghoul_Set/Ghoul_Boss/Anims/Ghoul_Boss_Die",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Ghoul_Festering",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Festering",
            thumbnail: "/Game/Commons/Textures/Enemy/Ghoul_Festering",
            max_hp: 100,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 7,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Festering.MGhoul_Festering_C'",
            ap_enemy: "AnimBlueprint'/Game/Ghoul_Set/Ghoul_Festering/ABP_Ghoul_festering.ABP_Ghoul_Festering_C'",
            skm_enemy: "/Game/Ghoul_Set/Ghoul_Festering/SK_Ghoul_Festering",
            aenemy_die: "/Game/Ghoul_Set/Ghoul_Festering/Anims/Festering_Ghoul_Die",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Ghoul_Scavenger",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Scavenger",
            thumbnail: "/Game/Commons/Textures/Enemy/Ghoul_Scavenger",
            max_hp: 78,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGhoul_Scavenger.MGhoul_Scavenger_C'",
            ap_enemy: "AnimBlueprint'/Game/Ghoul_Set/Ghoul_Scavenger/ABP_Ghoul_Scavenger.ABP_Ghoul_Scavenger_C'",
            skm_enemy: "/Game/Ghoul_Set/Ghoul_Scavenger/SK_Ghoul_Scavenger",
            aenemy_die: "/Game/Ghoul_Set/Ghoul_Scavenger/Anims/Ghoul_Scavenger_Die",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Golem",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MGolem",
            thumbnail: "/Game/Commons/Textures/Enemy/Golem",
            max_hp: 50,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 6,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MGolem.MGolem_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Golem/ABP_Golem.ABP_Golem_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/Golem/Golem_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Golemm",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "MonsterPlant",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MMonsterPlant",
            thumbnail: "/Game/Commons/Textures/Enemy/MonsterPlant",
            max_hp: 30,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MMonsterPlant.MMonsterPlant_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/MonsterPlant/ABS_MonsterPlant.ABS_MonsterPlant_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/MonsterPlant/MonsterPlant_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/MonsterPlant/Die_MP_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Orc",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MOrc",
            thumbnail: "/Game/Commons/Textures/Enemy/Orc",
            max_hp: 60,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 6,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MOrc.MOrc_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Orc/OneMeshCharacter/ABP_Orc_OneMesh.ABP_Orc_OneMesh_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/Orc/OneMeshCharacter/Orc_OneMesh_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Orc/Die_Orc_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Skeleton",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MSkeleton",
            thumbnail: "/Game/Commons/Textures/Enemy/Bat",
            max_hp: 60,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 6,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSkeleton.MSkeleton_C'",
            ap_enemy:
                "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Skeleton/OneMeshCharacter/ABP_Skeleton_OneMesh.ABP_Skeleton_OneMesh_C'",
            skm_enemy:
                "/Game/RPGMonsterWavePBR/Meshes/Skeleton/OneMeshCharacter/Skeleton_OneMesh_SK",
            aenemy_die:
                "/Game/RPGMonsterWavePBR/Animations/Skeleton/AnimForOneMeshSK/Die_Skeleton_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Slime",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MSlime",
            thumbnail: "/Game/Commons/Textures/Enemy/Slime",
            max_hp: 36,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 6,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSlime.MSlime_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Slime/ABP_Slime.ABP_Slime_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/Slime/Slime_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Slime/Die_Slime_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "Spider",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MSpider",
            thumbnail: "/Game/Commons/Textures/Enemy/Spider",
            max_hp: 40,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MSpider.MSpider_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/Spider/ABP_Spider.ABP_Spider_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/Spider/Spider_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Spider/Die_Spider_Anim",
        },
        NewEnemy {
            eid: get_guid_value() as i64,
            enemy_name: "TurleShell",
            model_path: "/Game/Match3RPG/Blueprints/Units/Enemy/MTurleShell",
            thumbnail: "/Game/Commons/Textures/Enemy/TurleShell",
            max_hp: 20,
            attack_power: 0,
            move_speed: 600.000000,
            max_mana: 0,
            defense: 5,
            animation_hit_delay: 0.50000,
            spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
            bp_enemy: "Blueprint'/Game/Match3RPG/Blueprints/Units/Enemy/MTurleShell.MTurleShell_C'",
            ap_enemy: "AnimBlueprint'/Game/RPGMonsterWavePBR/Meshes/TurtleShell/ABP_TurtleShell.ABP_TurtleShell_C'",
            skm_enemy: "/Game/RPGMonsterWavePBR/Meshes/TurtleShell/TurtleShell_SK",
            aenemy_die: "/Game/RPGMonsterWavePBR/Animations/Spider/Die_Spider_Anim",
        },
    ];

    let gems: Vec<NewGem> = vec![
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Gems/Gems_Axelcon",
            kind: 1,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemSwordSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkSwordMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemSword'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Gems/Gems_ShieldGem",
            kind: 2,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemShieldSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkShieldMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemShield'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Gems/Gems_GoldGem",
            kind: 3,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemGoldSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkGoldMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemGold'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Gems/Gems_Managem",
            kind: 3,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemManaSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkManaMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemMana'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1",
            kind: 1,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemSwordSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkSwordMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemSword'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2",
            kind: 1,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemSwordSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkSwordMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemSword'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_3",
            kind: 1,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemSwordSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkSwordMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemSword'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1",
            kind: 2,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemShieldSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkShieldMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemShield'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_2",
            kind: 2,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemShieldSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkShieldMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemShield'",
        },
        NewGem {
            gid: get_guid_value() as i64,
            gem_icon: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_3",
            kind: 2,
            gem_selected_material: "/Game/Commons/Textures/Material/M_GemShieldSelectedMat",
            gem_link_material: "/Game/Commons/Textures/Material/M_LinkShieldMat",
            model_path: "Class'/Script/TheForeAwakensCPlus.MGemShield'",
        }

    ];
    let equipment_kinds: Vec<NewEquipmentKind> = vec![
        NewEquipmentKind {
            kid: get_guid_value() as i64,
            name: "Weapon",
            kind: 1,
        },
        NewEquipmentKind {
            kid: get_guid_value() as i64,
            name: "Shield",
            kind: 2,
        },
        NewEquipmentKind {
            kid: get_guid_value() as i64,
            name: "Armor",
            kind: 3,
        },
    ];
    let equipments: Vec<NewEquipment> = vec![
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[2].kid,
            name: "First Armor",
            thumbnail: "/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_1",
            price: 1,
            hp: 5,
            multiplier: 1.0,
            kind: 3,
            is_default: 2,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[2].kid,
            name: "Second Armor",
            thumbnail: "/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_2",
            price: 240,
            hp: 10,
            multiplier: 1.0,
            kind: 3,
            is_default: 1,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[2].kid,
            name: "Third Armor",
            thumbnail: "/Game/Commons/Textures/Equipment/Clothes/Equipment_Clothes_3",
            price: 20,
            hp: 20,
            multiplier: 1.0,
            kind: 3,
            is_default: 1,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[1].kid,
            name: "First Shield",
            thumbnail: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_1",
            price: 15,
            hp: 0,
            multiplier: 1.0,
            kind: 2,
            is_default: 2,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[1].kid,
            name: "Secend Shield",
            thumbnail: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_2",
            price: 115,
            hp: 0,
            multiplier: 1.0,
            kind: 2,
            is_default: 1,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[1].kid,
            name: "Third Shield",
            thumbnail: "/Game/Commons/Textures/Equipment/Shield/Equipment_Shield_3",
            price: 215,
            hp: 0,
            multiplier: 2.0,
            kind: 2,
            is_default: 1,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[0].kid,
            name: "First Weapon",
            thumbnail: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_1",
            price: 1,
            hp: 0,
            multiplier: 1.0,
            kind: 1,
            is_default: 2,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[0].kid,
            name: "Second Weapon",
            thumbnail: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_2",
            price: 240,
            hp: 0,
            multiplier: 2.0,
            kind: 1,
            is_default: 1,
        },
        NewEquipment {
            eid: get_guid_value() as i64,
            kid: equipment_kinds[0].kid,
            name: "Third Weapon",
            thumbnail: "/Game/Commons/Textures/Equipment/Weapon/Equipment_Weapon_3",
            price: 900,
            hp: 0,
            multiplier: 3.0,
            kind: 1,
            is_default: 1,
        },
    ];

    let players: Vec<NewPlayer> = vec![NewPlayer {
        pid: get_guid_value() as i64,
        player_name: "default_name",
        model_path: "Blueprint'/Game/Match3RPG/Blueprints/Units/Player/HeroWizardSM.HeroWizardSM_C'",
        thumbnail: "/Game/Commons/Textures/ChatSystem/Icon/Icon_ChatSystemHead",
        max_hp: 20,
        attack_power: 5,
        move_speed: 600.000000,
        max_mana: 20,
        defense: 5,
        animation_hit_delay: 0.50000,
        spawn_style_class: "/Script/TheForeAwakensCPlus.MRunSpawn",
        level: 1,
        star_level: 1,
        level_experience: 120,
        is_default: 2,
    }];

    let skills_relateds: Vec<NewSkillFigthRelated> = vec![
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[0].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[1].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[2].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[3].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[4].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[4].eid,
            skill_id: skills[3].id,
            cool_down: skills[3].cool_down,
            attack_power: skills[3].attack_power,
            mana_power: skills[3].mana_power,
            probability: 1,
            level: skills[3].level,
            level_experience: skills[3].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[4].eid,
            skill_id: skills[4].id,
            cool_down: skills[4].cool_down,
            attack_power: skills[4].attack_power,
            mana_power: skills[4].mana_power,
            probability: 1,
            level: skills[4].level,
            level_experience: skills[4].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[5].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[5].eid,
            skill_id: skills[4].id,
            cool_down: skills[4].cool_down,
            attack_power: skills[4].attack_power,
            mana_power: skills[4].mana_power,
            probability: 1,
            level: skills[4].level,
            level_experience: skills[4].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[6].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[6].eid,
            skill_id: skills[3].id,
            cool_down: skills[3].cool_down,
            attack_power: skills[3].attack_power,
            mana_power: skills[3].mana_power,
            probability: 1,
            level: skills[3].level,
            level_experience: skills[3].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[7].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[8].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[9].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[9].eid,
            skill_id: skills[3].id,
            cool_down: skills[3].cool_down,
            attack_power: skills[3].attack_power,
            mana_power: skills[3].mana_power,
            probability: 1,
            level: skills[3].level,
            level_experience: skills[3].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[10].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[11].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[12].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: enemys[13].eid,
            skill_id: skills[2].id,
            cool_down: skills[2].cool_down,
            attack_power: skills[2].attack_power,
            mana_power: skills[2].mana_power,
            probability: 1,
            level: skills[2].level,
            level_experience: skills[2].level_experience,
            obj_type: 3,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[3].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[4].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[5].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[6].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[7].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: equipments[8].eid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 2,
        },
        NewSkillFigthRelated {
            id: get_guid_value() as i64,
            obj_id: players[0].pid,
            skill_id: skills[0].id,
            cool_down: skills[0].cool_down,
            attack_power: skills[0].attack_power,
            mana_power: skills[0].mana_power,
            probability: 0,
            level: skills[0].level,
            level_experience: skills[0].level_experience,
            obj_type: 1,
        },
    ];

    let gem_relateds: Vec<NewGemRelated> = vec![
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[3].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[4].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[5].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[6].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[7].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: equipments[8].eid,
            gid: gems[0].gid,
            obj_type: 2,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: players[0].pid,
            gid: gems[1].gid,
            obj_type: 1,
        },
        NewGemRelated {
            grid: get_guid_value() as i64,
            obj_id: players[0].pid,
            gid: gems[2].gid,
            obj_type: 1,
        },
    ];

    (
        skills,
        enemys,
        gems,
        equipment_kinds,
        equipments,
        players,
        skills_relateds,
        gem_relateds,
    )
}
