use keyring::Entry;

pub struct LocalStorage;
const APP_NAME: &str = "rust-chat";

impl LocalStorage {
    pub fn set_value(key:&str,value:&str){
        let e = Entry::new(APP_NAME, key).expect("ERROR SETTING KEYRING VALUE");
        e.set_password(value).expect("ERROR SETTING KEYRING VALUE");

    }
    pub fn get_value(key:&str)->Option<String>{
        let e = Entry::new(APP_NAME, key).expect("ERROR CREATING KEYRING VALUE");
        let v= e.get_password();
        return v.ok();
    }
    pub fn delete_value(key:&str){
        let e = Entry::new(APP_NAME, key).expect("ERROR CREATING KEYRING VALUE");
        e.delete_credential().expect("ERROR DELETING KEYRING VALUE");
    }
}