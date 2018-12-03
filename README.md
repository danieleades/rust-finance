# budget-cli
a command line application for budgeting, implemented in Rust

This is starting life as an excuse to learn Rust. With a little luck it will actually become usable.

Contributions and comments welcome.

- to build-

  `cargo build`

- to install-

  `cargo install`

- to use-

  see syntax.md (note, this is a work in progress. for full syntax run "`budget --help`")

data is saved in $HOME/.budget/

# The Plan:


## Model

- the budget is an 'envelope' or 'zero-based' budget. Spending is broken down into categories, and each category has a separate pool of cash available. as income flows into the account, it should be distributed amongst these category 'envelopes'.
- No category should be below zero (though it may be temporarily) as this would imply spending more money than you have.
- In order to cover over-spending in one category, money must be transferred from another category

## Subcommands

- **add** - add a new transaction to the budget
- **reconcile**- if the current balance of the account matches the current balance of the budget, than we know all transactions are present and accounted for up to this point. We can therefore freeze all the transactions and archive them.
- **category**- category command should be used for managing categories, (list, add, remove, rename, etc.)
- **transfer**- transfer money between categories. (default to the current month, but should be able to tranfer future months as well, and one day maybe the past as well)
- **summary**- this should provide a quick report of the current state of the budget. this should be the default if no arguments supplied
- **modify**- this command will be used to edit existing transactions. command takes a filter and new fields. The fields are applied to every transaction that matches the filter.
- **import**- import transactions from a file (maybe .ofx?)
- **export**- export transactions to file