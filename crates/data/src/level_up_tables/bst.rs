use crate::level_up_tables::LevelUpStats;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn stats_table_summed() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Sum Level 2 - bst_level_up_table_sum
        m.insert(
            2,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 2,
                magical_defense: 1,
            },
        );

        // Sum Level 3 - bst_level_up_table_sum
        m.insert(
            3,
            LevelUpStats {
                hp: 10,
                mp: 0,
                physical_attack: 8,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Sum Level 4 - bst_level_up_table_sum
        m.insert(
            4,
            LevelUpStats {
                hp: 15,
                mp: 1,
                physical_attack: 12,
                physical_defense: 6,
                magical_attack: 5,
                magical_defense: 6,
            },
        );

        // Sum Level 5 - bst_level_up_table_sum
        m.insert(
            5,
            LevelUpStats {
                hp: 20,
                mp: 1,
                physical_attack: 16,
                physical_defense: 9,
                magical_attack: 7,
                magical_defense: 9,
            },
        );

        // Sum Level 6 - bst_level_up_table_sum
        m.insert(
            6,
            LevelUpStats {
                hp: 25,
                mp: 2,
                physical_attack: 20,
                physical_defense: 12,
                magical_attack: 9,
                magical_defense: 12,
            },
        );

        // Sum Level 7 - bst_level_up_table_sum
        m.insert(
            7,
            LevelUpStats {
                hp: 31,
                mp: 3,
                physical_attack: 24,
                physical_defense: 15,
                magical_attack: 12,
                magical_defense: 15,
            },
        );

        // Sum Level 8 - bst_level_up_table_sum
        m.insert(
            8,
            LevelUpStats {
                hp: 37,
                mp: 3,
                physical_attack: 28,
                physical_defense: 18,
                magical_attack: 14,
                magical_defense: 18,
            },
        );

        // Sum Level 9 - bst_level_up_table_sum
        m.insert(
            9,
            LevelUpStats {
                hp: 43,
                mp: 4,
                physical_attack: 32,
                physical_defense: 21,
                magical_attack: 17,
                magical_defense: 21,
            },
        );

        // Sum Level 10 - bst_level_up_table_sum
        m.insert(
            10,
            LevelUpStats {
                hp: 49,
                mp: 4,
                physical_attack: 36,
                physical_defense: 24,
                magical_attack: 19,
                magical_defense: 24,
            },
        );

        // Sum Level 11 - bst_level_up_table_sum
        m.insert(
            11,
            LevelUpStats {
                hp: 55,
                mp: 4,
                physical_attack: 40,
                physical_defense: 27,
                magical_attack: 22,
                magical_defense: 27,
            },
        );

        // Sum Level 12 - bst_level_up_table_sum
        m.insert(
            12,
            LevelUpStats {
                hp: 62,
                mp: 4,
                physical_attack: 44,
                physical_defense: 30,
                magical_attack: 24,
                magical_defense: 30,
            },
        );

        // Sum Level 13 - bst_level_up_table_sum
        m.insert(
            13,
            LevelUpStats {
                hp: 69,
                mp: 5,
                physical_attack: 48,
                physical_defense: 33,
                magical_attack: 27,
                magical_defense: 33,
            },
        );

        // Sum Level 14 - bst_level_up_table_sum
        m.insert(
            14,
            LevelUpStats {
                hp: 76,
                mp: 5,
                physical_attack: 52,
                physical_defense: 36,
                magical_attack: 30,
                magical_defense: 36,
            },
        );

        // Sum Level 15 - bst_level_up_table_sum
        m.insert(
            15,
            LevelUpStats {
                hp: 83,
                mp: 6,
                physical_attack: 56,
                physical_defense: 39,
                magical_attack: 34,
                magical_defense: 39,
            },
        );

        // Sum Level 16 - bst_level_up_table_sum
        m.insert(
            16,
            LevelUpStats {
                hp: 90,
                mp: 7,
                physical_attack: 64,
                physical_defense: 42,
                magical_attack: 37,
                magical_defense: 42,
            },
        );

        // Sum Level 17 - bst_level_up_table_sum
        m.insert(
            17,
            LevelUpStats {
                hp: 97,
                mp: 7,
                physical_attack: 66,
                physical_defense: 45,
                magical_attack: 41,
                magical_defense: 45,
            },
        );

        // Sum Level 18 - bst_level_up_table_sum
        m.insert(
            18,
            LevelUpStats {
                hp: 104,
                mp: 8,
                physical_attack: 74,
                physical_defense: 48,
                magical_attack: 44,
                magical_defense: 48,
            },
        );

        // Sum Level 19 - bst_level_up_table_sum
        m.insert(
            19,
            LevelUpStats {
                hp: 111,
                mp: 9,
                physical_attack: 76,
                physical_defense: 48,
                magical_attack: 48,
                magical_defense: 48,
            },
        );

        // Sum Level 20 - bst_level_up_table_sum
        m.insert(
            20,
            LevelUpStats {
                hp: 118,
                mp: 9,
                physical_attack: 84,
                physical_defense: 50,
                magical_attack: 51,
                magical_defense: 50,
            },
        );

        // Sum Level 21 - bst_level_up_table_sum
        m.insert(
            21,
            LevelUpStats {
                hp: 126,
                mp: 9,
                physical_attack: 86,
                physical_defense: 50,
                magical_attack: 55,
                magical_defense: 50,
            },
        );

        // Sum Level 22 - bst_level_up_table_sum
        m.insert(
            22,
            LevelUpStats {
                hp: 134,
                mp: 10,
                physical_attack: 94,
                physical_defense: 51,
                magical_attack: 58,
                magical_defense: 51,
            },
        );

        // Sum Level 23 - bst_level_up_table_sum
        m.insert(
            23,
            LevelUpStats {
                hp: 142,
                mp: 10,
                physical_attack: 96,
                physical_defense: 51,
                magical_attack: 62,
                magical_defense: 51,
            },
        );

        // Sum Level 24 - bst_level_up_table_sum
        m.insert(
            24,
            LevelUpStats {
                hp: 150,
                mp: 10,
                physical_attack: 104,
                physical_defense: 53,
                magical_attack: 65,
                magical_defense: 53,
            },
        );

        // Sum Level 25 - bst_level_up_table_sum
        m.insert(
            25,
            LevelUpStats {
                hp: 158,
                mp: 10,
                physical_attack: 106,
                physical_defense: 53,
                magical_attack: 70,
                magical_defense: 53,
            },
        );

        // Sum Level 26 - bst_level_up_table_sum
        m.insert(
            26,
            LevelUpStats {
                hp: 166,
                mp: 12,
                physical_attack: 114,
                physical_defense: 54,
                magical_attack: 74,
                magical_defense: 54,
            },
        );

        // Sum Level 27 - bst_level_up_table_sum
        m.insert(
            27,
            LevelUpStats {
                hp: 175,
                mp: 12,
                physical_attack: 116,
                physical_defense: 54,
                magical_attack: 77,
                magical_defense: 54,
            },
        );

        // Sum Level 28 - bst_level_up_table_sum
        m.insert(
            28,
            LevelUpStats {
                hp: 184,
                mp: 12,
                physical_attack: 124,
                physical_defense: 56,
                magical_attack: 80,
                magical_defense: 56,
            },
        );

        // Sum Level 29 - bst_level_up_table_sum
        m.insert(
            29,
            LevelUpStats {
                hp: 193,
                mp: 12,
                physical_attack: 126,
                physical_defense: 56,
                magical_attack: 83,
                magical_defense: 56,
            },
        );

        // Sum Level 30 - bst_level_up_table_sum
        m.insert(
            30,
            LevelUpStats {
                hp: 202,
                mp: 12,
                physical_attack: 134,
                physical_defense: 59,
                magical_attack: 87,
                magical_defense: 59,
            },
        );

        m
    })
}

pub fn stats_table() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Level 2 - bst_level_up_table
        m.insert(
            2,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 2,
                magical_defense: 1,
            },
        );

        // Level 3 - bst_level_up_table
        m.insert(
            3,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 1,
                magical_attack: 1,
                magical_defense: 2,
            },
        );

        // Level 4 - bst_level_up_table
        m.insert(
            4,
            LevelUpStats {
                hp: 5,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 5 - bst_level_up_table
        m.insert(
            5,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 6 - bst_level_up_table
        m.insert(
            6,
            LevelUpStats {
                hp: 5,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 7 - bst_level_up_table
        m.insert(
            7,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 8 - bst_level_up_table
        m.insert(
            8,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 9 - bst_level_up_table
        m.insert(
            9,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 10 - bst_level_up_table
        m.insert(
            10,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 11 - bst_level_up_table
        m.insert(
            11,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 12 - bst_level_up_table
        m.insert(
            12,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 13 - bst_level_up_table
        m.insert(
            13,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 14 - bst_level_up_table
        m.insert(
            14,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 15 - bst_level_up_table
        m.insert(
            15,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 16 - bst_level_up_table
        m.insert(
            16,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 8,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 17 - bst_level_up_table
        m.insert(
            17,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 18 - bst_level_up_table
        m.insert(
            18,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 8,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 19 - bst_level_up_table
        m.insert(
            19,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 20 - bst_level_up_table
        m.insert(
            20,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 8,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 21 - bst_level_up_table
        m.insert(
            21,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 22 - bst_level_up_table
        m.insert(
            22,
            LevelUpStats {
                hp: 8,
                mp: 1,
                physical_attack: 8,
                physical_defense: 1,
                magical_attack: 3,
                magical_defense: 1,
            },
        );

        // Level 23 - bst_level_up_table
        m.insert(
            23,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 24 - bst_level_up_table
        m.insert(
            24,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 8,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 25 - bst_level_up_table
        m.insert(
            25,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 26 - bst_level_up_table
        m.insert(
            26,
            LevelUpStats {
                hp: 8,
                mp: 2,
                physical_attack: 8,
                physical_defense: 1,
                magical_attack: 4,
                magical_defense: 1,
            },
        );

        // Level 27 - bst_level_up_table
        m.insert(
            27,
            LevelUpStats {
                hp: 9,
                mp: 0,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 28 - bst_level_up_table
        m.insert(
            28,
            LevelUpStats {
                hp: 9,
                mp: 0,
                physical_attack: 8,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 29 - bst_level_up_table
        m.insert(
            29,
            LevelUpStats {
                hp: 9,
                mp: 0,
                physical_attack: 2,
                physical_defense: 0,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 30 - bst_level_up_table
        m.insert(
            30,
            LevelUpStats {
                hp: 9,
                mp: 0,
                physical_attack: 8,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        m
    })
}
