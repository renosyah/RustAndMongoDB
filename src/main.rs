
extern crate bson;
extern crate mongodb;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
extern crate serde;


use mongodb::{Client, ThreadedClient};
use mongodb::db::ThreadedDatabase;

fn connect() -> Client {
    let client = Client::connect("localhost", 27017)
        .expect("Failed to initialize standalone client.");
    return client;
}

const Mahasiswa : &str = "Mahasiswa";
const Ka : &str = "KA";


#[derive(Clone)]
#[derive(Serialize, Deserialize, Debug)]
pub struct Mahasiswa {
    pub Nim : i32,
    pub  Nama : String,
    pub Jurusan : String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AllMahasiswa {
    pub All : Vec<Mahasiswa>
}


impl Mahasiswa {


    fn insertData(data: Mahasiswa) -> bool {
        let db = connect();
        let coll = db.db(Ka).collection(Mahasiswa);

        let serialMhs = bson::to_bson(&data).ok().expect("failed");

        if let bson::Bson::Document(document) = serialMhs {
            coll.insert_one(document, None)
                .ok().expect("Failed to insert document.");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
            return false;
        }

        return true;
    }

    fn findAllData() -> AllMahasiswa {
        let db = connect();
        let coll = db
            .db(Ka)
            .collection(Mahasiswa);

        let mut AllData: Vec<Mahasiswa> = Vec::new();

        let cursor = coll.find(None, None)
            .ok()
            .expect("Failed to execute find.");

        for item in cursor {
            let person: Mahasiswa = bson::from_bson(bson::Bson::Document(item.unwrap())).ok().expect("none");
            AllData.push(person);
        }
        let all = AllMahasiswa { All: AllData };


        return all;
    }

    fn findOne(data : Mahasiswa) -> Mahasiswa {
        let db = connect();
        let coll = db
            .db(Ka)
            .collection(Mahasiswa);

        let mut a = bson::Document::new();

        let serialMhs = bson::to_bson(&data)
            .ok()
            .expect("failed");
        if let bson::Bson::Document(document) = serialMhs {
            a = document;
        } else {
            println!("Error converting the BSON object into a MongoDB document");
        }

        let cursor = coll.find_one(Some(a), None)
            .ok()
            .expect("Failed to execute find.");

        let person : Mahasiswa = bson::from_bson(bson::Bson::Document(cursor.unwrap())).ok().expect("none");
        return person;
    }

    fn update(data: Mahasiswa, dataNew: Mahasiswa) {
        Mahasiswa::delete(data);
        Mahasiswa::insertData(dataNew);
    }

    fn delete(data: Mahasiswa) -> bool {
        let db = connect();
        let coll = db.db(Ka).collection(Mahasiswa);

        let serialMhs = bson::to_bson(&data).ok().expect("failed");
        if let bson::Bson::Document(document) = serialMhs {
           coll.delete_many(document, None)
                .ok().expect("Failed to execute find.");
        } else {
            println!("Error converting the BSON object into a MongoDB document");
            return false;
        }

        return true;
    }
}



fn main() {

    let mhs = Mahasiswa{
        Nim:153210003,
        Nama:"Reno Syahputra".to_string(),
        Jurusan:"Akuntansi".to_string()
    };

    Mahasiswa::insertData(mhs.clone());
    println!("{} , {:?} , {:?}",serde_json::to_string(&Mahasiswa::findAllData()).unwrap(),Mahasiswa::findAllData(),Mahasiswa::findOne(mhs.clone()));
    Mahasiswa::delete(mhs.clone());
}
