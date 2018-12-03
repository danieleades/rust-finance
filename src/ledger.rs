use crate::Transaction;
use std::path::PathBuf;

// this is an intentially simplistic collection type. Once I've been using this for a while, and know what i actually need it to be able to do, then I
// can start optimising it and getting clever about it.

#[derive(Default)]
pub struct Ledger {
    transactions: Vec<Transaction>,
    file_path: Option<PathBuf>,
    n: u32,
}

impl Ledger {
    pub fn from_file<P: Into<PathBuf>>(path: P) -> Result<Self, ()> {
        unimplemented!()
    }

    pub fn from_transactions<T>(transactions: T) -> Self
    where
        T: IntoIterator<Item = Transaction>,
    {
        let mut ledger = Ledger {
            transactions: transactions.into_iter().collect(),
            ..Self::default()
        };
        ledger.sort_by_date();
        ledger
    }

    fn sort_by_date(&mut self) {
        self.transactions.sort_by_key(Transaction::date)
    }

    pub fn categories(&self) -> Vec<String> {
        let categories: Vec<String> = self
            .transactions
            .iter()
            .map(|x| x.category())
            .filter(|x| x.is_some())
            .map(|x| x.clone().unwrap())
            .collect();
        categories
    }

    pub fn add(&mut self, t: Transaction) {
        self.transactions.push(t);
        self.n += 1;
        self.sort_by_date();
    }
}

impl IntoIterator for Ledger {
    type Item = Transaction;
    type IntoIter = std::vec::IntoIter<Transaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.into_iter()
    }
}

impl<'a> IntoIterator for &'a Ledger {
    type Item = &'a Transaction;
    type IntoIter = std::slice::Iter<'a, Transaction>;

    fn into_iter(self) -> Self::IntoIter {
        self.transactions.iter()
    }
}

impl<'a> IntoIterator for &'a mut Ledger {
    type Item = &'a mut Transaction;
    type IntoIter = std::slice::IterMut<'a, Transaction>;
    fn into_iter(self) -> Self::IntoIter {
        self.transactions.iter_mut()
    }
}
