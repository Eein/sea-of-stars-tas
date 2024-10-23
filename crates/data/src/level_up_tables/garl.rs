use crate::level_up_tables::LevelUpStats;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn stats_table_summed() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Sum Level 2 - garl_level_up_table_sum
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

        // Sum Level 3 - garl_level_up_table_sum
        m.insert(
            3,
            LevelUpStats {
                hp: 7,
                mp: 1,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Sum Level 4 - garl_level_up_table_sum
        m.insert(
            4,
            LevelUpStats {
                hp: 13,
                mp: 2,
                physical_attack: 5,
                physical_defense: 6,
                magical_attack: 5,
                magical_defense: 6,
            },
        );

        // Sum Level 5 - garl_level_up_table_sum
        m.insert(
            5,
            LevelUpStats {
                hp: 19,
                mp: 2,
                physical_attack: 7,
                physical_defense: 9,
                magical_attack: 7,
                magical_defense: 9,
            },
        );

        // Sum Level 6 - garl_level_up_table_sum
        m.insert(
            6,
            LevelUpStats {
                hp: 25,
                mp: 3,
                physical_attack: 10,
                physical_defense: 12,
                magical_attack: 9,
                magical_defense: 12,
            },
        );

        // Sum Level 7 - garl_level_up_table_sum
        m.insert(
            7,
            LevelUpStats {
                hp: 31,
                mp: 4,
                physical_attack: 12,
                physical_defense: 15,
                magical_attack: 12,
                magical_defense: 15,
            },
        );

        // Sum Level 8 - garl_level_up_table_sum
        m.insert(
            8,
            LevelUpStats {
                hp: 37,
                mp: 4,
                physical_attack: 15,
                physical_defense: 18,
                magical_attack: 14,
                magical_defense: 18,
            },
        );

        // Sum Level 9 - garl_level_up_table_sum
        m.insert(
            9,
            LevelUpStats {
                hp: 43,
                mp: 5,
                physical_attack: 17,
                physical_defense: 21,
                magical_attack: 17,
                magical_defense: 21,
            },
        );

        // Sum Level 10 - garl_level_up_table_sum
        m.insert(
            10,
            LevelUpStats {
                hp: 49,
                mp: 5,
                physical_attack: 20,
                physical_defense: 24,
                magical_attack: 19,
                magical_defense: 24,
            },
        );

        // Sum Level 11 - garl_level_up_table_sum
        m.insert(
            11,
            LevelUpStats {
                hp: 55,
                mp: 5,
                physical_attack: 22,
                physical_defense: 27,
                magical_attack: 22,
                magical_defense: 27,
            },
        );

        // Sum Level 12 - garl_level_up_table_sum
        m.insert(
            12,
            LevelUpStats {
                hp: 61,
                mp: 7,
                physical_attack: 25,
                physical_defense: 30,
                magical_attack: 24,
                magical_defense: 30,
            },
        );

        // Sum Level 13 - garl_level_up_table_sum
        m.insert(
            13,
            LevelUpStats {
                hp: 67,
                mp: 7,
                physical_attack: 27,
                physical_defense: 33,
                magical_attack: 27,
                magical_defense: 33,
            },
        );

        // Sum Level 14 - garl_level_up_table_sum
        m.insert(
            14,
            LevelUpStats {
                hp: 74,
                mp: 7,
                physical_attack: 31,
                physical_defense: 36,
                magical_attack: 30,
                magical_defense: 36,
            },
        );

        // Sum Level 15 - garl_level_up_table_sum
        m.insert(
            15,
            LevelUpStats {
                hp: 81,
                mp: 7,
                physical_attack: 34,
                physical_defense: 39,
                magical_attack: 34,
                magical_defense: 39,
            },
        );

        // Sum Level 16 - garl_level_up_table_sum
        m.insert(
            16,
            LevelUpStats {
                hp: 88,
                mp: 9,
                physical_attack: 38,
                physical_defense: 42,
                magical_attack: 37,
                magical_defense: 41,
            },
        );

        // Sum Level 17 - garl_level_up_table_sum
        m.insert(
            17,
            LevelUpStats {
                hp: 95,
                mp: 9,
                physical_attack: 41,
                physical_defense: 45,
                magical_attack: 41,
                magical_defense: 43,
            },
        );

        // Sum Level 18 - garl_level_up_table_sum
        m.insert(
            18,
            LevelUpStats {
                hp: 102,
                mp: 9,
                physical_attack: 45,
                physical_defense: 48,
                magical_attack: 44,
                magical_defense: 45,
            },
        );

        // Sum Level 19 - garl_level_up_table_sum
        m.insert(
            19,
            LevelUpStats {
                hp: 109,
                mp: 11,
                physical_attack: 48,
                physical_defense: 48,
                magical_attack: 48,
                magical_defense: 45,
            },
        );

        // Sum Level 20 - garl_level_up_table_sum
        m.insert(
            20,
            LevelUpStats {
                hp: 116,
                mp: 11,
                physical_attack: 52,
                physical_defense: 50,
                magical_attack: 51,
                magical_defense: 45,
            },
        );

        // Sum Level 21 - garl_level_up_table_sum
        m.insert(
            21,
            LevelUpStats {
                hp: 123,
                mp: 11,
                physical_attack: 55,
                physical_defense: 50,
                magical_attack: 55,
                magical_defense: 47,
            },
        );

        // Sum Level 22 - garl_level_up_table_sum
        m.insert(
            22,
            LevelUpStats {
                hp: 130,
                mp: 11,
                physical_attack: 59,
                physical_defense: 52,
                magical_attack: 58,
                magical_defense: 48,
            },
        );

        // Sum Level 23 - garl_level_up_table_sum
        m.insert(
            23,
            LevelUpStats {
                hp: 137,
                mp: 11,
                physical_attack: 62,
                physical_defense: 52,
                magical_attack: 62,
                magical_defense: 48,
            },
        );

        // Sum Level 24 - garl_level_up_table_sum
        m.insert(
            24,
            LevelUpStats {
                hp: 144,
                mp: 11,
                physical_attack: 66,
                physical_defense: 54,
                magical_attack: 65,
                magical_defense: 50,
            },
        );

        // Sum Level 25 - garl_level_up_table_sum
        m.insert(
            25,
            LevelUpStats {
                hp: 152,
                mp: 11,
                physical_attack: 70,
                physical_defense: 54,
                magical_attack: 70,
                magical_defense: 50,
            },
        );

        // Sum Level 26 - garl_level_up_table_sum
        m.insert(
            26,
            LevelUpStats {
                hp: 160,
                mp: 13,
                physical_attack: 75,
                physical_defense: 55,
                magical_attack: 74,
                magical_defense: 51,
            },
        );

        // Sum Level 27 - garl_level_up_table_sum
        m.insert(
            27,
            LevelUpStats {
                hp: 168,
                mp: 13,
                physical_attack: 79,
                physical_defense: 55,
                magical_attack: 79,
                magical_defense: 51,
            },
        );

        // Sum Level 28 - garl_level_up_table_sum
        m.insert(
            28,
            LevelUpStats {
                hp: 176,
                mp: 13,
                physical_attack: 84,
                physical_defense: 57,
                magical_attack: 83,
                magical_defense: 53,
            },
        );

        // Sum Level 29 - garl_level_up_table_sum
        m.insert(
            29,
            LevelUpStats {
                hp: 184,
                mp: 13,
                physical_attack: 88,
                physical_defense: 57,
                magical_attack: 88,
                magical_defense: 53,
            },
        );

        // Sum Level 30 - garl_level_up_table_sum
        m.insert(
            30,
            LevelUpStats {
                hp: 192,
                mp: 13,
                physical_attack: 93,
                physical_defense: 60,
                magical_attack: 92,
                magical_defense: 56,
            },
        );

        m
    })
}

pub fn stats_table() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();
        // Level 2 - garl_level_up_table
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

        // Level 3 - garl_level_up_table
        m.insert(
            3,
            LevelUpStats {
                hp: 4,
                mp: 0,
                physical_attack: 2,
                physical_defense: 1,
                magical_attack: 1,
                magical_defense: 2,
            },
        );

        // Level 4 - garl_level_up_table
        m.insert(
            4,
            LevelUpStats {
                hp: 6,
                mp: 1,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 5 - garl_level_up_table
        m.insert(
            5,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 6 - garl_level_up_table
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

        // Level 7 - garl_level_up_table
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

        // Level 8 - garl_level_up_table
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

        // Level 9 - garl_level_up_table
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

        // Level 10 - garl_level_up_table
        m.insert(
            10,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 11 - garl_level_up_table
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

        // Level 12 - garl_level_up_table
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

        // Level 13 - garl_level_up_table
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

        // Level 14 - garl_level_up_table
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

        // Level 15 - garl_level_up_table
        m.insert(
            15,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 16 - garl_level_up_table
        m.insert(
            16,
            LevelUpStats {
                hp: 7,
                mp: 2,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 17 - garl_level_up_table
        m.insert(
            17,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 18 - garl_level_up_table
        m.insert(
            18,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 19 - garl_level_up_table
        m.insert(
            19,
            LevelUpStats {
                hp: 7,
                mp: 2,
                physical_attack: 3,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 20 - garl_level_up_table
        m.insert(
            20,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 21 - garl_level_up_table
        m.insert(
            21,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 22 - garl_level_up_table
        m.insert(
            22,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 1,
            },
        );

        // Level 23 - garl_level_up_table
        m.insert(
            23,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 24 - garl_level_up_table
        m.insert(
            24,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 25 - garl_level_up_table
        m.insert(
            25,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 26 - garl_level_up_table
        m.insert(
            26,
            LevelUpStats {
                hp: 8,
                mp: 2,
                physical_attack: 5,
                physical_defense: 1,
                magical_attack: 4,
                magical_defense: 1,
            },
        );

        // Level 27 - garl_level_up_table
        m.insert(
            27,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 28 - garl_level_up_table
        m.insert(
            28,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 5,
                physical_defense: 2,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 29 - garl_level_up_table
        m.insert(
            29,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 30 - garl_level_up_table
        m.insert(
            30,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 5,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        m
    })
}
