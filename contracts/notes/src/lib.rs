#![no_std] //Step 1
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

// Struktur data yang akan menyimpan resident
#[contracttype]
#[derive(Clone, Debug)] //Representasi table di DB / MODEL di MVC / Entity / POJO
pub struct Resident {
    nik: u64,
    full_name: String,
    gender: String,
    age: u32,
}

// Storage key untuk data resident
const RESIDENT: Symbol = symbol_short!("RESIDENT");

#[contract] //Step 2
pub struct ResidentContract; // Service Class / Controller

#[contractimpl] //Step 3
impl ResidentContract {
    // Helper function untuk mengambil data (agar tidak menulis ulang logic get & unwrap)
    fn get_all_residents(env: &Env) -> Vec<Resident> {
        // Gunakan turbofish ::<Symbol, Vec<Resident>> agar compiler tahu tipe datanya
        env.storage()
            .instance()
            .get::<Symbol, Vec<Resident>>(&RESIDENT)
            .unwrap_or(Vec::new(env))
    }

    // Fungsi untuk mendapatkan semua resident
    pub fn get_residents(env: Env) -> Vec<Resident> {
        // 1. ambil data resident dari storage
        Self::get_all_residents(&env)
    }

    // Fungsi untuk membuat resident baru
    pub fn create_resident(env: Env, full_name: String, gender: String, age: u32) -> String {
        // 1. ambil data resident dari storage
        let mut residents = Self::get_all_residents(&env);

        // 2. Buat object resident baru
        let resident = Resident {
            nik: env.prng().gen::<u64>(), // Generate NIK secara random
            full_name,
            gender,
            age,
        };

        // 3. tambahkan resident baru ke residents lama
        residents.push_back(resident);

        // 4. simpan residents ke storage
        env.storage().instance().set(&RESIDENT, &residents);

        return String::from_str(&env, "Resident berhasil ditambahkan");
    }

    // Fungsi untuk menghapus resident berdasarkan nik
    pub fn delete_resident(env: Env, nik: u64) -> String {
        // 1. ambil data resident dari storage
        let mut residents = Self::get_all_residents(&env);

        // 2. cari index resident yang akan dihapus menggunakan perulangan
        for i in 0..residents.len() {
            let resident = residents.get(i).unwrap();
            if resident.nik == nik {
                residents.remove(i);
                env.storage().instance().set(&RESIDENT, &residents);
                return String::from_str(&env, "Resident berhasil dihapus");
            }
        }

        return String::from_str(&env, "Resident tidak ditemukan");
    }

    // Fungsi untuk mengupdate resident berdasarkan nik
    pub fn update_resident(
        env: Env,
        nik: u64,
        full_name: String,
        gender: String,
        age: u32,
    ) -> String {
        // 1. ambil data resident dari storage
        let mut residents = Self::get_all_residents(&env);

        // 2. cari index resident yang akan diupdate
        let mut found_index: Option<u32> = None;
        for i in 0..residents.len() {
            if residents.get(i).unwrap().nik == nik {
                found_index = Some(i);
                break;
            }
        }

        // 3. Cek apakah ada, lalu update resident pada index tersebut
        if let Some(i) = found_index {
            let updated_resident = Resident {
                nik,
                full_name,
                gender,
                age,
            };
            residents.set(i, updated_resident);
        } else {
            return String::from_str(&env, "Resident tidak ditemukan");
        }

        // 4. simpan residents ke storage
        env.storage().instance().set(&RESIDENT, &residents);

        return String::from_str(&env, "Resident berhasil diupdate");
    }
}

mod test;
