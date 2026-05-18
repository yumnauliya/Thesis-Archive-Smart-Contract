#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};

// Struktur data yang akan menyimpan data skripsi
#[contracttype]
#[derive(Clone, Debug)]
pub struct Thesis {
    id: u64,
    title: String,
    author: String,
    year: u32,
    category: String,
    abstract_text: String,
}

// Storage key untuk data skripsi
const THESIS_DATA: Symbol = symbol_short!("THESIS");

#[contract]
pub struct ThesisContract;

#[contractimpl]
impl ThesisContract {

    // Fungsi untuk mengambil semua data skripsi
    pub fn get_all_thesis(env: Env) -> Vec<Thesis> {
        return env.storage()
            .instance()
            .get(&THESIS_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Fungsi untuk menambahkan skripsi baru
    pub fn create_thesis(
        env: Env,
        title: String,
        author: String,
        year: u32,
        category: String,
        abstract_text: String,
    ) -> String {

        // 1. ambil data skripsi dari storage
        let mut thesis_list: Vec<Thesis> = env.storage()
            .instance()
            .get(&THESIS_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. buat object skripsi baru
        let thesis = Thesis {
            id: env.prng().gen::<u64>(),
            title,
            author,
            year,
            category,
            abstract_text,
        };

        // 3. tambahkan skripsi baru
        thesis_list.push_back(thesis);

        // 4. simpan data ke storage
        env.storage().instance().set(&THESIS_DATA, &thesis_list);

        return String::from_str(&env, "Skripsi berhasil ditambahkan");
    }

    // Fungsi untuk menghapus skripsi berdasarkan id
    pub fn delete_thesis(env: Env, id: u64) -> String {

        // 1. ambil data skripsi dari storage
        let mut thesis_list: Vec<Thesis> = env.storage()
            .instance()
            .get(&THESIS_DATA)
            .unwrap_or(Vec::new(&env));

        // 2. cari index skripsi
        for i in 0..thesis_list.len() {

            if thesis_list.get(i).unwrap().id == id {

                thesis_list.remove(i);

                env.storage()
                    .instance()
                    .set(&THESIS_DATA, &thesis_list);

                return String::from_str(&env, "Skripsi berhasil dihapus");
            }
        }

        return String::from_str(&env, "Skripsi tidak ditemukan");
    }

    // Fungsi untuk filter skripsi berdasarkan tahun
    pub fn get_thesis_by_year(env: Env, target_year: u32) -> Vec<Thesis> {

        let thesis_list: Vec<Thesis> = env.storage()
            .instance()
            .get(&THESIS_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..thesis_list.len() {

            let thesis = thesis_list.get(i).unwrap();

            if thesis.year == target_year {
                result.push_back(thesis);
            }
        }

        return result;
    }

    // Fungsi untuk filter skripsi berdasarkan kategori
    pub fn get_thesis_by_category(
        env: Env,
        target_category: String
    ) -> Vec<Thesis> {

        let thesis_list: Vec<Thesis> = env.storage()
            .instance()
            .get(&THESIS_DATA)
            .unwrap_or(Vec::new(&env));

        let mut result = Vec::new(&env);

        for i in 0..thesis_list.len() {

            let thesis = thesis_list.get(i).unwrap();

            if thesis.category == target_category {
                result.push_back(thesis);
            }
        }

        return result;
    }
}

mod test;