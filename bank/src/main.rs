#[derive(Debug)]
struct Account {
    id: u32,
    balance: i32,
    holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
        Account {
            id,
            holder,
            balance: 0,
        }
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        Bank { accounts: vec![] }
    }
}

fn add_account(bank: &mut Bank, account: Account) {
    bank.accounts.push(account);
}

fn main() {
    let mut bank = Bank::new();
    let account = Account::new(1, String::from("me"));
    
    // TODO: call the 'add_account' function here
    add_account(&mut bank, account);
    
    
    // Note: we're using the Bank value here, so 'bank' still
    // needs ownership of that value
    println!("{:#?}", bank);
}







