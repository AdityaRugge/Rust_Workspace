#[derive(Debug)]

struct Account {
    id: u32,
    description: String,
}

fn change_account( account: &mut Account) {
    account.id = 10;
}

fn main(){

    let mut account = Account {
        id: 1,
        description: String::from("Complete bank transaction \n"),
    };

    let account_ref = &account; //read only reference
    change_account(&mut account); //mutable reference
    println!(" After bank transaction \n {:#?}", account); 
    prinytln!(" After bank transaction \n {:#?}", account_ref); // This will throw an error because we cannot have a mutable reference and an immutable reference at the same time.
}


