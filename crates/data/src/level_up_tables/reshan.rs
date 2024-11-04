use crate::level_up_tables::LevelUpStats;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn stats_table_summed() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Sum Level 2 - reshan_level_up_table_sum
        m.insert(
            2,
            LevelUpStats {
                hp: 3,
                mp: 1,
                physical_attack: 1,
                physical_defense: 2,
                magical_attack: 2,
                magical_defense: 1,
            },
        );

        // Sum Level 3 - reshan_level_up_table_sum
        m.insert(
            3,
            LevelUpStats {
                hp: 9,
                mp: 1,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Sum Level 4 - reshan_level_up_table_sum
        m.insert(
            4,
            LevelUpStats {
                hp: 15,
                mp: 2,
                physical_attack: 4,
                physical_defense: 6,
                magical_attack: 6,
                magical_defense: 6,
            },
        );

        // Sum Level 5 - reshan_level_up_table_sum
        m.insert(
            5,
            LevelUpStats {
                hp: 21,
                mp: 2,
                physical_attack: 5,
                physical_defense: 9,
                magical_attack: 9,
                magical_defense: 9,
            },
        );

        // Sum Level 6 - reshan_level_up_table_sum
        m.insert(
            6,
            LevelUpStats {
                hp: 27,
                mp: 3,
                physical_attack: 8,
                physical_defense: 12,
                magical_attack: 11,
                magical_defense: 12,
            },
        );

        // Sum Level 7 - reshan_level_up_table_sum
        m.insert(
            7,
            LevelUpStats {
                hp: 33,
                mp: 4,
                physical_attack: 10,
                physical_defense: 15,
                magical_attack: 14,
                magical_defense: 15,
            },
        );

        // Sum Level 8 - reshan_level_up_table_sum
        m.insert(
            8,
            LevelUpStats {
                hp: 39,
                mp: 4,
                physical_attack: 13,
                physical_defense: 18,
                magical_attack: 16,
                magical_defense: 18,
            },
        );

        // Sum Level 9 - reshan_level_up_table_sum
        m.insert(
            9,
            LevelUpStats {
                hp: 45,
                mp: 5,
                physical_attack: 15,
                physical_defense: 21,
                magical_attack: 19,
                magical_defense: 21,
            },
        );

        // Sum Level 10 - reshan_level_up_table_sum
        m.insert(
            10,
            LevelUpStats {
                hp: 51,
                mp: 5,
                physical_attack: 17,
                physical_defense: 24,
                magical_attack: 22,
                magical_defense: 24,
            },
        );

        // Sum Level 11 - reshan_level_up_table_sum
        m.insert(
            11,
            LevelUpStats {
                hp: 57,
                mp: 5,
                physical_attack: 19,
                physical_defense: 27,
                magical_attack: 25,
                magical_defense: 27,
            },
        );

        // Sum Level 12 - reshan_level_up_table_sum
        m.insert(
            12,
            LevelUpStats {
                hp: 63,
                mp: 7,
                physical_attack: 22,
                physical_defense: 30,
                magical_attack: 27,
                magical_defense: 30,
            },
        );

        // Sum Level 13 - reshan_level_up_table_sum
        m.insert(
            13,
            LevelUpStats {
                hp: 69,
                mp: 7,
                physical_attack: 24,
                physical_defense: 33,
                magical_attack: 30,
                magical_defense: 33,
            },
        );

        // Sum Level 14 - reshan_level_up_table_sum
        m.insert(
            14,
            LevelUpStats {
                hp: 75,
                mp: 7,
                physical_attack: 27,
                physical_defense: 36,
                magical_attack: 34,
                magical_defense: 36,
            },
        );

        // Sum Level 15 - reshan_level_up_table_sum
        m.insert(
            15,
            LevelUpStats {
                hp: 81,
                mp: 7,
                physical_attack: 30,
                physical_defense: 39,
                magical_attack: 38,
                magical_defense: 39,
            },
        );

        // Sum Level 16 - reshan_level_up_table_sum
        m.insert(
            16,
            LevelUpStats {
                hp: 87,
                mp: 9,
                physical_attack: 34,
                physical_defense: 42,
                magical_attack: 41,
                magical_defense: 42,
            },
        );

        // Sum Level 17 - reshan_level_up_table_sum
        m.insert(
            17,
            LevelUpStats {
                hp: 93,
                mp: 9,
                physical_attack: 37,
                physical_defense: 45,
                magical_attack: 45,
                magical_defense: 45,
            },
        );

        // Sum Level 18 - reshan_level_up_table_sum
        m.insert(
            18,
            LevelUpStats {
                hp: 99,
                mp: 9,
                physical_attack: 40,
                physical_defense: 48,
                magical_attack: 49,
                magical_defense: 48,
            },
        );

        // Sum Level 19 - reshan_level_up_table_sum
        m.insert(
            19,
            LevelUpStats {
                hp: 105,
                mp: 11,
                physical_attack: 43,
                physical_defense: 49,
                magical_attack: 53,
                magical_defense: 49,
            },
        );

        // Sum Level 20 - reshan_level_up_table_sum
        m.insert(
            20,
            LevelUpStats {
                hp: 111,
                mp: 11,
                physical_attack: 47,
                physical_defense: 50,
                magical_attack: 56,
                magical_defense: 49,
            },
        );

        // Sum Level 21 - reshan_level_up_table_sum
        m.insert(
            21,
            LevelUpStats {
                hp: 117,
                mp: 11,
                physical_attack: 50,
                physical_defense: 50,
                magical_attack: 60,
                magical_defense: 50,
            },
        );

        // Sum Level 22 - reshan_level_up_table_sum
        m.insert(
            22,
            LevelUpStats {
                hp: 123,
                mp: 11,
                physical_attack: 54,
                physical_defense: 50,
                magical_attack: 63,
                magical_defense: 52,
            },
        );

        // Sum Level 23 - reshan_level_up_table_sum
        m.insert(
            23,
            LevelUpStats {
                hp: 129,
                mp: 11,
                physical_attack: 57,
                physical_defense: 51,
                magical_attack: 67,
                magical_defense: 52,
            },
        );

        // Sum Level 24 - reshan_level_up_table_sum
        m.insert(
            24,
            LevelUpStats {
                hp: 135,
                mp: 11,
                physical_attack: 61,
                physical_defense: 53,
                magical_attack: 70,
                magical_defense: 52,
            },
        );

        // Sum Level 25 - reshan_level_up_table_sum
        m.insert(
            25,
            LevelUpStats {
                hp: 141,
                mp: 11,
                physical_attack: 65,
                physical_defense: 53,
                magical_attack: 75,
                magical_defense: 53,
            },
        );

        // Sum Level 26 - reshan_level_up_table_sum
        m.insert(
            26,
            LevelUpStats {
                hp: 147,
                mp: 13,
                physical_attack: 70,
                physical_defense: 53,
                magical_attack: 79,
                magical_defense: 55,
            },
        );

        // Sum Level 27 - reshan_level_up_table_sum
        m.insert(
            27,
            LevelUpStats {
                hp: 153,
                mp: 13,
                physical_attack: 74,
                physical_defense: 54,
                magical_attack: 84,
                magical_defense: 55,
            },
        );

        // Sum Level 28 - reshan_level_up_table_sum
        m.insert(
            28,
            LevelUpStats {
                hp: 159,
                mp: 13,
                physical_attack: 79,
                physical_defense: 56,
                magical_attack: 88,
                magical_defense: 55,
            },
        );

        // Sum Level 29 - reshan_level_up_table_sum
        m.insert(
            29,
            LevelUpStats {
                hp: 165,
                mp: 13,
                physical_attack: 83,
                physical_defense: 56,
                magical_attack: 93,
                magical_defense: 56,
            },
        );

        // Sum Level 30 - reshan_level_up_table_sum
        m.insert(
            30,
            LevelUpStats {
                hp: 171,
                mp: 13,
                physical_attack: 88,
                physical_defense: 58,
                magical_attack: 97,
                magical_defense: 58,
            },
        );

        m
    })
}

pub fn stats_table() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Level 2 - reshan_level_up_table
        m.insert(
            2,
            LevelUpStats {
                hp: 3,
                mp: 1,
                physical_attack: 1,
                physical_defense: 2,
                magical_attack: 2,
                magical_defense: 1,
            },
        );

        // Level 3 - reshan_level_up_table
        m.insert(
            3,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 1,
                magical_attack: 1,
                magical_defense: 2,
            },
        );

        // Level 4 - reshan_level_up_table
        m.insert(
            4,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 1,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 5 - reshan_level_up_table
        m.insert(
            5,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 1,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 6 - reshan_level_up_table
        m.insert(
            6,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 7 - reshan_level_up_table
        m.insert(
            7,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 8 - reshan_level_up_table
        m.insert(
            8,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 9 - reshan_level_up_table
        m.insert(
            9,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 10 - reshan_level_up_table
        m.insert(
            10,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 11 - reshan_level_up_table
        m.insert(
            11,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 12 - reshan_level_up_table
        m.insert(
            12,
            LevelUpStats {
                hp: 6,
                mp: 2,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 13 - reshan_level_up_table
        m.insert(
            13,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 14 - reshan_level_up_table
        m.insert(
            14,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 15 - reshan_level_up_table
        m.insert(
            15,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 16 - reshan_level_up_table
        m.insert(
            16,
            LevelUpStats {
                hp: 6,
                mp: 2,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 17 - reshan_level_up_table
        m.insert(
            17,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 18 - reshan_level_up_table
        m.insert(
            18,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 19 - reshan_level_up_table
        m.insert(
            19,
            LevelUpStats {
                hp: 6,
                mp: 2,
                physical_attack: 3,
                physical_defense: 1,
                magical_attack: 4,
                magical_defense: 1,
            },
        );

        // Level 20 - reshan_level_up_table
        m.insert(
            20,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 1,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 21 - reshan_level_up_table
        m.insert(
            21,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 1,
            },
        );

        // Level 22 - reshan_level_up_table
        m.insert(
            22,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 23 - reshan_level_up_table
        m.insert(
            23,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 1,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 24 - reshan_level_up_table
        m.insert(
            24,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 25 - reshan_level_up_table
        m.insert(
            25,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 1,
            },
        );

        // Level 26 - reshan_level_up_table
        m.insert(
            26,
            LevelUpStats {
                hp: 6,
                mp: 2,
                physical_attack: 5,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 27 - reshan_level_up_table
        m.insert(
            27,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 1,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 28 - reshan_level_up_table
        m.insert(
            28,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 5,
                physical_defense: 2,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 29 - reshan_level_up_table
        m.insert(
            29,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 1,
            },
        );

        // Level 30 - reshan_level_up_table
        m.insert(
            30,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 5,
                physical_defense: 2,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        m
    })
}
