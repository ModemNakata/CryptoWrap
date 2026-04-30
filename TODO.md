add finalizer.py script that would run in background (crontab)

cronjob frequency = 30 minutes

it's purpose is to (passively) check deposits table, update statuses of deposits until they are finalized.
on finalization -> wallet address can be reused again (add some graceful cool-down period, 30 minutes - specific for taking function - e.g. deposit/create)
    (e.g. table monero_wallet, column is_available becomes = true)

but actually address reuse is questionable


### ROADMAP:

 - before adding any more coins - let's complete front-end logic, improve api, swagger docs, dashboard should become usable


---

amounts in the dashboard are shown with `unconfirmed` too, though it's not spandable
(just count balances using deposits history ? or have additional table for assets ?)
> if counting balance by deposits, then how to deal with spending?
> adding new table for assets adds abstration layer and adds complexity
> using direct (to-blockchain) balance check sounds like the solution, but how to deal with wallets with many addresses?
(litecoin -> electrumx protocol server)

> consolidate all UTXOs on spend -> limit address check script hashes to 1 (change address)
though we need to keep track if `deposit` address has been `consolidated` yes or not :::
we can add another column to litecoin addresses for this or we can use `is_available` column
but we don't have a `finalizer` script yet, that we wanted to implement for address re-use
and apperently we can make address re-use after outgoing payment.
though adding a new column to litecoin addresses will be more `isolated` and `safer` way.
but when does it have to toggle on and off? -> update deposits.rs to toggle on keep_track if balance is `detected` or `confirmed`, after transaction is consolidated -> keep_track - to - false ::: change address is always tracked


- add `recent transaction` on dashboard at the bottom
- allow user to change fiat currency to calculate profits in dashboard (rub,eur,usd - supported fiat conversion currencies in database)




