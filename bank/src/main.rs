#[derive(Debug)]

struct Account {
    balance: u32,
    description: String,
}

fn change_account( account: &mut Account) {
    account.balance = 10;
}

fn main(){

    let mut account = Account {
        balance: 1,
        description: String::from("Complete bank transaction \n"),
    };

    let account_ref = &mut account; //mutable reference
    change_account(account_ref); //passing mutable reference to function
    println!(" \n {:#?}", account_ref); 

    account.balance = 100; //directly changing the value of balance
    println!(" \n {:#?}", account); 
}

    


