struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: f64,
}

impl BankAccount {
    fn deposit(&mut self, amount: f64) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: f64) {
        if amount <= self.balance {
            self.balance -= amount;
        } else {
            println!("Insufficient funds!");
        }
    }

    fn display(&self) {
        println!("Account: {}, Name: {}, Balance: {:.2}", self.account_number, self.holder_name, self.balance);
    }
}

fn main() {
    let mut account = BankAccount { account_number: 12345, holder_name: "John Doe".to_string(), balance: 1000.0 };

    account.deposit(500.0);
    account.withdraw(200.0);
    account.display();
}
