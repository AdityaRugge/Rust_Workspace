struct Bank {
    holder: String,
    balance: f64,
    account_number: u32,
}

impl Bank {
    fn new(holder: String, balance: f64, account_number: u32) -> Self { // Constructor for the Bank struct
        Bank {
            holder,
            balance,
            account_number,
        }
    }

    fn deposit(&mut self, amount: f64) { // Method to deposit money into the account
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) -> Result<(), String> { // Method to withdraw money from the account
        if amount > self.balance {
            Err(String::from("Insufficient funds"))
        } else {
            self.balance -= amount;
            Ok(())
        }
    }

    fn get_balance(&self) -> f64 {
        println!("Current balance: {}", self.balance);
        self.balance
    }
}

fn main() {
    println!("Welcome to the Bank Application!");
    let mut bank_account = Bank::new(String::from("John Doe"), 1000.0, 12345);

    bank_account.deposit(50.0);
    bank_account.get_balance();
}
