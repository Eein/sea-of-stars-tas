use crate::level_up_tables::LevelUpStats;

use std::collections::HashMap;
use std::sync::OnceLock;

pub fn stats_table_summed() -> &'static HashMap<u32, LevelUpStats> {
    static HASHMAP: OnceLock<HashMap<u32, LevelUpStats>> = OnceLock::new();
    HASHMAP.get_or_init(|| {
        let mut m = HashMap::new();

        // Sum Level 2 - valere_level_up_table_sum
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

        // Sum Level 3 - valere_level_up_table_sum
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

        // Sum Level 4 - valere_level_up_table_sum
        m.insert(
            4,
            LevelUpStats {
                hp: 13,
                mp: 1,
                physical_attack: 5,
                physical_defense: 6,
                magical_attack: 5,
                magical_defense: 6,
            },
        );

        // Sum Level 5 - valere_level_up_table_sum
        m.insert(
            5,
            LevelUpStats {
                hp: 17,
                mp: 1,
                physical_attack: 7,
                physical_defense: 9,
                magical_attack: 7,
                magical_defense: 9,
            },
        );

        // Sum Level 6 - valere_level_up_table_sum
        m.insert(
            6,
            LevelUpStats {
                hp: 23,
                mp: 2,
                physical_attack: 10,
                physical_defense: 12,
                magical_attack: 9,
                magical_defense: 12,
            },
        );

        // Sum Level 7 - valere_level_up_table_sum
        m.insert(
            7,
            LevelUpStats {
                hp: 27,
                mp: 2,
                physical_attack: 13,
                physical_defense: 15,
                magical_attack: 11,
                magical_defense: 15,
            },
        );

        // Sum Level 8 - valere_level_up_table_sum
        m.insert(
            8,
            LevelUpStats {
                hp: 33,
                mp: 2,
                physical_attack: 15,
                physical_defense: 18,
                magical_attack: 14,
                magical_defense: 18,
            },
        );

        // Sum Level 9 - valere_level_up_table_sum
        m.insert(
            9,
            LevelUpStats {
                hp: 37,
                mp: 3,
                physical_attack: 18,
                physical_defense: 21,
                magical_attack: 16,
                magical_defense: 21,
            },
        );

        // Sum Level 10 - valere_level_up_table_sum
        m.insert(
            10,
            LevelUpStats {
                hp: 43,
                mp: 3,
                physical_attack: 21,
                physical_defense: 24,
                magical_attack: 18,
                magical_defense: 24,
            },
        );

        // Sum Level 11 - valere_level_up_table_sum
        m.insert(
            11,
            LevelUpStats {
                hp: 47,
                mp: 3,
                physical_attack: 23,
                physical_defense: 27,
                magical_attack: 21,
                magical_defense: 27,
            },
        );

        // Sum Level 12 - valere_level_up_table_sum
        m.insert(
            12,
            LevelUpStats {
                hp: 53,
                mp: 5,
                physical_attack: 26,
                physical_defense: 30,
                magical_attack: 23,
                magical_defense: 30,
            },
        );

        // Sum Level 13 - valere_level_up_table_sum
        m.insert(
            13,
            LevelUpStats {
                hp: 57,
                mp: 5,
                physical_attack: 29,
                physical_defense: 33,
                magical_attack: 25,
                magical_defense: 33,
            },
        );

        // Sum Level 14 - valere_level_up_table_sum
        m.insert(
            14,
            LevelUpStats {
                hp: 64,
                mp: 5,
                physical_attack: 32,
                physical_defense: 36,
                magical_attack: 29,
                magical_defense: 36,
            },
        );

        // Sum Level 15 - valere_level_up_table_sum
        m.insert(
            15,
            LevelUpStats {
                hp: 69,
                mp: 5,
                physical_attack: 36,
                physical_defense: 39,
                magical_attack: 32,
                magical_defense: 39,
            },
        );

        // Sum Level 16 - valere_level_up_table_sum
        m.insert(
            16,
            LevelUpStats {
                hp: 76,
                mp: 7,
                physical_attack: 40,
                physical_defense: 42,
                magical_attack: 35,
                magical_defense: 42,
            },
        );

        // Sum Level 17 - valere_level_up_table_sum
        m.insert(
            17,
            LevelUpStats {
                hp: 81,
                mp: 7,
                physical_attack: 43,
                physical_defense: 45,
                magical_attack: 39,
                magical_defense: 45,
            },
        );

        // Sum Level 18 - valere_level_up_table_sum
        m.insert(
            18,
            LevelUpStats {
                hp: 88,
                mp: 7,
                physical_attack: 47,
                physical_defense: 48,
                magical_attack: 42,
                magical_defense: 48,
            },
        );

        // Sum Level 19 - valere_level_up_table_sum
        m.insert(
            19,
            LevelUpStats {
                hp: 93,
                mp: 9,
                physical_attack: 51,
                physical_defense: 48,
                magical_attack: 45,
                magical_defense: 50,
            },
        );

        // Sum Level 20 - valere_level_up_table_sum
        m.insert(
            20,
            LevelUpStats {
                hp: 100,
                mp: 9,
                physical_attack: 54,
                physical_defense: 50,
                magical_attack: 49,
                magical_defense: 50,
            },
        );

        // Sum Level 21 - valere_level_up_table_sum
        m.insert(
            21,
            LevelUpStats {
                hp: 105,
                mp: 9,
                physical_attack: 58,
                physical_defense: 50,
                magical_attack: 52,
                magical_defense: 51,
            },
        );

        // Sum Level 22 - valere_level_up_table_sum
        m.insert(
            22,
            LevelUpStats {
                hp: 112,
                mp: 9,
                physical_attack: 62,
                physical_defense: 52,
                magical_attack: 55,
                magical_defense: 51,
            },
        );

        // Sum Level 23 - valere_level_up_table_sum
        m.insert(
            23,
            LevelUpStats {
                hp: 117,
                mp: 9,
                physical_attack: 65,
                physical_defense: 52,
                magical_attack: 59,
                magical_defense: 53,
            },
        );

        // Sum Level 24 - valere_level_up_table_sum
        m.insert(
            24,
            LevelUpStats {
                hp: 124,
                mp: 9,
                physical_attack: 69,
                physical_defense: 54,
                magical_attack: 62,
                magical_defense: 53,
            },
        );

        // Sum Level 25 - valere_level_up_table_sum
        m.insert(
            25,
            LevelUpStats {
                hp: 132,
                mp: 9,
                physical_attack: 74,
                physical_defense: 54,
                magical_attack: 66,
                magical_defense: 54,
            },
        );

        // Sum Level 26 - valere_level_up_table_sum
        m.insert(
            26,
            LevelUpStats {
                hp: 138,
                mp: 11,
                physical_attack: 78,
                physical_defense: 56,
                magical_attack: 71,
                magical_defense: 54,
            },
        );

        // Sum Level 27 - valere_level_up_table_sum
        m.insert(
            27,
            LevelUpStats {
                hp: 146,
                mp: 11,
                physical_attack: 83,
                physical_defense: 56,
                magical_attack: 75,
                magical_defense: 56,
            },
        );

        // Sum Level 28 - valere_level_up_table_sum
        m.insert(
            28,
            LevelUpStats {
                hp: 152,
                mp: 11,
                physical_attack: 88,
                physical_defense: 58,
                magical_attack: 79,
                magical_defense: 56,
            },
        );

        // Sum Level 29 - valere_level_up_table_sum
        m.insert(
            29,
            LevelUpStats {
                hp: 160,
                mp: 11,
                physical_attack: 92,
                physical_defense: 58,
                magical_attack: 84,
                magical_defense: 58,
            },
        );

        // Sum Level 30 - valere_level_up_table_sum
        m.insert(
            30,
            LevelUpStats {
                hp: 166,
                mp: 11,
                physical_attack: 97,
                physical_defense: 60,
                magical_attack: 88,
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
        // Level 2 - valere_level_up_table
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

        // Level 3 - valere_level_up_table
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

        // Level 4 - valere_level_up_table
        m.insert(
            4,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 5 - valere_level_up_table
        m.insert(
            5,
            LevelUpStats {
                hp: 4,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 6 - valere_level_up_table
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

        // Level 7 - valere_level_up_table
        m.insert(
            7,
            LevelUpStats {
                hp: 4,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 8 - valere_level_up_table
        m.insert(
            8,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 9 - valere_level_up_table
        m.insert(
            9,
            LevelUpStats {
                hp: 4,
                mp: 1,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 10 - valere_level_up_table
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

        // Level 11 - valere_level_up_table
        m.insert(
            11,
            LevelUpStats {
                hp: 4,
                mp: 0,
                physical_attack: 2,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 12 - valere_level_up_table
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

        // Level 13 - valere_level_up_table
        m.insert(
            13,
            LevelUpStats {
                hp: 4,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 2,
                magical_defense: 3,
            },
        );

        // Level 14 - valere_level_up_table
        m.insert(
            14,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 15 - valere_level_up_table
        m.insert(
            15,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 16 - valere_level_up_table
        m.insert(
            16,
            LevelUpStats {
                hp: 7,
                mp: 2,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 17 - valere_level_up_table
        m.insert(
            17,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 3,
                physical_defense: 3,
                magical_attack: 4,
                magical_defense: 3,
            },
        );

        // Level 18 - valere_level_up_table
        m.insert(
            18,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 3,
                magical_attack: 3,
                magical_defense: 3,
            },
        );

        // Level 19 - valere_level_up_table
        m.insert(
            19,
            LevelUpStats {
                hp: 5,
                mp: 2,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 3,
                magical_defense: 2,
            },
        );

        // Level 20 - valere_level_up_table
        m.insert(
            20,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 3,
                physical_defense: 2,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        // Level 21 - valere_level_up_table
        m.insert(
            21,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 3,
                magical_defense: 1,
            },
        );

        // Level 22 - valere_level_up_table
        m.insert(
            22,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 23 - valere_level_up_table
        m.insert(
            23,
            LevelUpStats {
                hp: 5,
                mp: 0,
                physical_attack: 3,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 24 - valere_level_up_table
        m.insert(
            24,
            LevelUpStats {
                hp: 7,
                mp: 0,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 3,
                magical_defense: 0,
            },
        );

        // Level 25 - valere_level_up_table
        m.insert(
            25,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 5,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 1,
            },
        );

        // Level 26 - valere_level_up_table
        m.insert(
            26,
            LevelUpStats {
                hp: 6,
                mp: 2,
                physical_attack: 4,
                physical_defense: 2,
                magical_attack: 5,
                magical_defense: 0,
            },
        );

        // Level 27 - valere_level_up_table
        m.insert(
            27,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 5,
                physical_defense: 0,
                magical_attack: 4,
                magical_defense: 2,
            },
        );

        // Level 28 - valere_level_up_table
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

        // Level 29 - valere_level_up_table
        m.insert(
            29,
            LevelUpStats {
                hp: 8,
                mp: 0,
                physical_attack: 4,
                physical_defense: 0,
                magical_attack: 5,
                magical_defense: 2,
            },
        );

        // Level 30 - valere_level_up_table
        m.insert(
            30,
            LevelUpStats {
                hp: 6,
                mp: 0,
                physical_attack: 5,
                physical_defense: 2,
                magical_attack: 4,
                magical_defense: 0,
            },
        );

        m
    })
}
