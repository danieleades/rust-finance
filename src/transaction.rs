use chrono::{DateTime, Utc};
use decimal::d128;
use serde_derive::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// transaction value. a positive number represents flow into the account
    amount: d128,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    payee: Option<String>,

    /// the date that the transaction is created. If no transaction date is set, this will be used for sorting
    date_created: DateTime<Utc>,

    /// the date that the transaction occurred
    #[serde(skip_serializing_if = "Option::is_none")]
    date_transaction: Option<DateTime<Utc>>,

    /// An optional category for the transaction
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    account: Option<String>,

    /// A vector of strings used to organise transactions
    tags: Vec<String>,

    /// An optional non-unique id
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<u16>,

    /// A globally unique id
    uuid: Uuid,

    /// If true, the budget has been reconciled past the date of this transaction. reconciled transactions should not be edited (lightly)
    reconciled: bool,

    /// The source of this transaction. This enum may be used for differentiating between transactions
    /// in a single ledger that came from different sources
    source: Source,
}

impl Default for Transaction {
    fn default() -> Self {
        Transaction {
            amount: d128::default(),
            description: None,
            payee: None,
            date_created: Utc::now(),
            date_transaction: None,
            category: None,
            account: None,
            tags: Vec::<String>::default(),
            id: None,
            uuid: Uuid::new_v4(),
            reconciled: false,
            source: Source::Manual,
        }
    }
}

impl Transaction {
    pub fn new<T: Into<d128>>(amount: T) -> Transaction {
        Transaction {
            amount: amount.into(),
            ..Self::default()
        }
    }

    pub fn amount(&self) -> d128 {
        self.amount
    }

    pub fn set_amount<T: Into<d128>>(&mut self, amount: T) {
        self.amount = amount.into();
    }

    pub fn with_amount<T: Into<d128>>(mut self, amount: T) -> Self {
        self.set_amount(amount.into());
        self
    }

    pub fn created(&self) -> DateTime<Utc> {
        self.date_created
    }

    pub fn date_transaction(&self) -> Option<DateTime<Utc>> {
        self.date_transaction
    }

    pub fn set_date_transaction<T: Into<DateTime<Utc>>>(&mut self, date: Option<T>) {
        self.date_transaction = date.map(T::into);
    }

    pub fn with_date_transaction<T: Into<DateTime<Utc>>>(mut self, date: T) -> Self {
        self.set_date_transaction(Some(date.into()));
        self
    }

    pub fn date(&self) -> DateTime<Utc> {
        self.date_transaction().unwrap_or_else(|| self.created())
    }

    pub fn description(&self) -> &Option<String> {
        &self.description
    }

    pub fn set_description<S: Into<String>>(&mut self, description: Option<S>) {
        self.description = description.map(S::into);
    }

    pub fn with_description<S: Into<String>>(mut self, description: Option<S>) -> Self {
        self.description = description.map(S::into);
        self
    }

    pub fn payee(&self) -> &Option<String> {
        &self.payee
    }

    pub fn set_payee<S: Into<String>>(&mut self, payee: Option<S>) {
        self.payee = payee.map(S::into);
    }

    pub fn with_payee<S: Into<String>>(mut self, payee: Option<S>) -> Self {
        self.payee = payee.map(S::into);
        self
    }

    pub fn category(&self) -> &Option<String> {
        &self.category
    }

    pub fn set_category<S: Into<String>>(&mut self, category: Option<S>) {
        self.category = category.map(S::into);
    }

    pub fn with_category<S: Into<String>>(mut self, category: S) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn account(&self) -> &Option<String> {
        &self.account
    }

        pub fn set_account<S: Into<String>>(&self) -> &Option<String> {
        &self.account
    }

    /// add tag to transaction, if its not already present
    pub fn tag<S: Into<String>>(&mut self, tag: S) {
        let t: String = tag.into();
        if !self.tags.contains(&t) {
            self.tags.push(t)
        }
    }

    pub fn with_tag<S: Into<String>>(mut self, tag: S) -> Self {
        self.tag(tag);
        self
    }

    /// removes a tag, if it exists
    pub fn untag<S: AsRef<String>>(&mut self, tag: S) {
        self.tags.retain(|x| x != tag.as_ref());
    }

    pub fn without_tag<S: AsRef<String>>(mut self, tag: S) -> Self {
        self.untag(tag);
        self
    }

    /// sets the transaction tags to exactly those supplied
    pub fn set_tags<S: Into<String>>(&mut self, tags: Vec<S>) {
        let mut t: Vec<String> = tags.into_iter().map(S::into).collect();
        t.sort();
        t.dedup();
        self.tags = t;
    }

    pub fn tags(&self) -> std::slice::Iter<String> {
        self.tags.iter()
    }

    pub fn id(&self) -> Option<u16> {
        self.id
    }

    pub fn set_id<T: Into<u16>>(&mut self, id: Option<T>) {
        self.id = id.map(T::into);
    }

    pub fn with_id<T: Into<u16>>(mut self, id: Option<T>) -> Self {
        self.set_id(id);
        self
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn reconciled(&self) -> bool {
        self.reconciled
    }

    pub fn set_reconciled(&mut self, b: bool) {
        self.reconciled = b;
    }

    pub fn source(&self) -> &Source {
        &self.source
    }

    pub fn set_source(&mut self, s: Source) {
        self.source = s;
    }

    pub fn with_source(mut self, s: Source) -> Self {
        self.set_source(s);
        self
    }

    /// returns true if two transactions have the same amount, description, category, tags, transaction date.
    /// ids, added date, source, and reconciled state are not considered.
    pub fn is_similar(&self, other: &Transaction) -> bool {
        self.amount() == other.amount()
            && self.description() == other.description()
            && self.category() == other.category()
            && self.date_transaction() == other.date_transaction()
            && self.tags().as_slice() == other.tags().as_slice()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Source {
    Manual,
    Reconciliation,
}

impl<T> std::ops::Add<T> for Transaction
where
    T: Into<d128>,
{
    type Output = Self;
    fn add(mut self, other: T) -> Self {
        self.amount += other.into();
        self
    }
}

impl<T> std::ops::AddAssign<T> for Transaction
where
    T: Into<d128>,
{
    fn add_assign(&mut self, other: T) {
        self.amount += other.into();
    }
}

impl<T> std::ops::Sub<T> for Transaction
where
    T: Into<d128>,
{
    type Output = Self;
    fn sub(mut self, other: T) -> Self {
        self.amount -= other.into();
        self
    }
}

impl<T> std::ops::SubAssign<T> for Transaction
where
    T: Into<d128>,
{
    fn sub_assign(&mut self, other: T) {
        self.amount -= other.into();
    }
}
