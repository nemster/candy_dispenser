# CandyDispenser

`CandyDispenser` is a blueprint to manage the distribution of any number of fungibles to people depositing a fungible (`GBOF`).  
When a user deposits a `GBOF` a `DckslapDepositedEvent` is emitted; this event can be intercepted by a bot that generates a random number from 0 to 999 and pass it to the `CandyDispenser` component.  

The owner can deposit any number of fungibles in `CandyDispenser` and define up to 10 rules accordig which the fungibles will be sent back to the depositor. 
 
## `new`
Use this function to instatiate a new `CandyDispenser` component.  

```
CALL_FUNCTION
    Address("<PACKAGE_ADDRESS>")
    "CandyDispenser"
    "new"
    Address("<OWNER_BADGE_ADDRESS>")
    Address("<BOT_BADGE_ADDRESS>")
    Address("<GBOF_ADDRESS>")
;
```

`<PACKAGE_ADDRESS>`: address of the package containing the `CandyDispenser` blueprint.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<BOT_BADGE_ADDRESS>`: the resource address of the bot badge.  
`<GBOF_ADDRESS>`: resource address of the coin users will deposit.  

## `deposit_candy`
Only the owner can invoke this method.  
It is possible to deposit any fungible coin.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("<COIN_ADDRESS>")
    Decimal("<COIN_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("<COIN_ADDRESS>")
    Bucket("candy")
;
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_amount"
    Address("<OWNER_BADGE_ADDRESS>")
    Decimal("1")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "deposit_candy"
    Bucket("candy")
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<COIN_ADDRESS>`: resource address of the coin to deposit.  
`<COIN_AMOUNT>`: amount of coins to deposit.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  

## `set_rule`
Only the owner can invoke this method.  
Insert a new rule or update an existing one.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_amount"
    Address("<OWNER_BADGE_ADDRESS>")
    Decimal("1")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "set_rule"
    <RULE_NUMBER>usize
    <MIN_DEPOSITS>u32
    <EXACT_DEPOSIT_ONLY>
    <THRESHOLD>u16
    Address("<COIN_ADDRESS>")
    <COIN_AMOUNT>u8
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
`<RULE_NUMBER>`: the number of the rule to set [0 - 9].  
`<MIN_DEPOSIT>`: this rule can be only matched when a user has deposited at least this number of `GBOF`.  
`<EXACT_DEPOSIT_ONLY>`: if `true` the rule will be matched only when a user has deposited exactly `<MIN_DEPOSIT>` `GBOF`.  
`<THRESHOLD>`: this rule will be matched only if the random number reported by the bot is bigger or equal than this (0 = always, 1000 = never).   
`<COIN_ADDRESS>`: resource address of the coin to send when this rule is matched.  
`<COIN_AMOUNT>`: amount of coins to send when this rule is matched.  
 
## `unset_rule`
Only the owner can invoke this method.  
Delete an existing rule.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "create_proof_of_amount"
    Address("<OWNER_BADGE_ADDRESS>")
    Decimal("1")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "unset_rule"
    <RULE_NUMBER>usize
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
`<RULE_NUMBER>`: the number of the rule to set [0 - 9].  

## `deposit_gbof`
A user can deposit a single `GBOF` by invoking this method.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("<GBOF_ADDRESS>")
    Decimal("1")
;
TAKE_ALL_FROM_WORKTOP
    Address("<GBOF_ADDRESS>")
    Bucket("gbof")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "deposit_gbof"
    Bucket("gbof")
    Address("<ACCOUNT_ADDRESS>")
;
```

## `deposit_xrd`
Use this method to deposit XRD to pay future users' transactions

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("<XRD_ADDRESS>")
    Decimal("<XRD_AMOUNT>")
;
TAKE_ALL_FROM_WORKTOP
    Address("<XRD_ADDRESS>")
    Bucket("xrd")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "deposit_xrd"
    Bucket("xrd")
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<XRD_ADDRESS>`: XRD resource address.  
`<XRD_AMOUNT>`: amount of XRD to deposit in the component.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
