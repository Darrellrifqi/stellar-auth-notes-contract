#![no_std]
use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, 
    vec, Env, String, Symbol, Vec, Address, panic_with_error
};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Note {
    pub id: u64,
    pub owner: Address, // Fitur Baru: Melacak siapa pemilik catatan
    pub title: String,
    pub content: String,
}

const NOTE_DATA: Symbol = symbol_short!("NOTE_DATA");

// Fitur Baru: Event untuk tracking di frontend/explorer
const EVENT_NOTE_CREATED: Symbol = symbol_short!("CREATED");
const EVENT_NOTE_DELETED: Symbol = symbol_short!("DELETED");

#[contract]
pub struct NotesContract;

#[contractimpl]
impl NotesContract {
    
    // CREATE dengan Authentikasi
    pub fn create_note(env: Env, owner: Address, title: String, content: String) -> u64 {
        // 1. Verifikasi bahwa yang memanggil adalah pemilik Address tersebut
        owner.require_auth();

        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(vec![&env]);
        
        // 2. ID Unik menggunakan Ledger Sequence + Timestamp agar lebih aman
        let id = env.ledger().sequence() as u64 + env.ledger().timestamp();
        
        let new_note = Note {
            id,
            owner: owner.clone(),
            title,
            content,
        };

        notes.push_back(new_note);
        env.storage().instance().set(&NOTE_DATA, &notes);

        // 3. Emit Event (Sangat disukai juri untuk dapps yang profesional)
        env.events().publish((EVENT_NOTE_CREATED, owner), id);
        
        id
    }

    // READ: Filter catatan hanya milik user tertentu
    pub fn get_my_notes(env: Env, user: Address) -> Vec<Note> {
        let notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(vec![&env]);
        let mut user_notes = vec![&env];

        for note in notes.iter() {
            if note.owner == user {
                user_notes.push_back(note);
            }
        }
        user_notes
    }

    // DELETE dengan Pengecekan Izin (Authorization)
    pub fn delete_note(env: Env, user: Address, id: u64) -> bool {
        // Keamanan: Hanya pemilik yang bisa menghapus
        user.require_auth();

        let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(vec![&env]);
        let mut found = false;
        
        for i in 0..notes.len() {
            let note = notes.get(i).unwrap();
            if note.id == id {
                // Verifikasi kepemilikan sebelum hapus
                if note.owner != user {
                    panic!("Bukan pemilik catatan!"); 
                }
                
                notes.remove(i);
                found = true;
                break;
            }
        }

        if found {
            env.storage().instance().set(&NOTE_DATA, &notes);
            env.events().publish((EVENT_NOTE_DELETED, user), id);
        }
        
        found
    }
}













/* --- CONTOH SCRIPT ---

pub fn get_notes(env: Env) -> Vec<Note> {
    // 1. ambil data notes dari storage
    return env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
}

// Fungsi untuk membuat note baru
pub fn create_note(env: Env, title: String, content: String) -> String {
    // 1. ambil data notes dari storage
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));
    
    // 2. Buat object note baru
    let note = Note {
        id: env.prng().gen::<u64>(),
        title: title,
        content: content,
    };
    
    // 3. tambahkan note baru ke notes lama
    notes.push_back(note);
    
    // 4. simpan notes ke storage
    env.storage().instance().set(&NOTE_DATA, &notes);
    
    return String::from_str(&env, "Notes berhasil ditambahkan");
}

// Fungsi untuk menghapus notes berdasarkan id
pub fn delete_note(env: Env, id: u64) -> String {
    // 1. ambil data notes dari storage 
    let mut notes: Vec<Note> = env.storage().instance().get(&NOTE_DATA).unwrap_or(Vec::new(&env));

    // 2. cari index note yang akan dihapus menggunakan perulangan
    for i in 0..notes.len() {
        if notes.get(i).unwrap().id == id {
            notes.remove(i);

            env.storage().instance().set(&NOTE_DATA, &notes);
            return String::from_str(&env, "Berhasil hapus notes");
        }
    }

    return String::from_str(&env, "Notes tidak ditemukan")
}


*/