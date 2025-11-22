# CandyDispenser

`CandyDispenser` is a blueprint to manage the distribution of any number of fungibles to people depositing a fungible (`DCKSLAP`).  
When a user deposits a `DCKSLAP` a `DckslapDepositedEvent` is emitted; this event can be intercepted by a bot that generates a random number from 0 to 999 and pass it to the `CandyDispenser` component.  

The owner can deposit any number of fungibles in `CandyDispenser` and define rules accordig which the fungibles will be sent back to the depositor.  

A user can also deposit `GBOF` to increase his level; the level starts from 0 an can grow up to 12.  
To pass from level zero to level one a user has to deposit one `GBOF`.  
To pass from level one to level two a user has to deposit two more `GBOF` (total 3).  
To pass from level two to level three a user has to deposit three more `GBOF` (total 6).  
And so on...  

Each rule is identified by the level number (0-12) and a rule number ranging from 0 to 14; so there will be a rule 0 for the level 0 and a different rule 0 for the level 1 (195 rules in total).  

On each depotit the user must also specify a `choice` that can be any number from 1 to 255.  

A rule can apply to a specific `choice` value or to any `choice` (0).  

## `new`
Use this function to instatiate a new `CandyDispenser` component.  

```
CALL_FUNCTION
    Address("<PACKAGE_ADDRESS>")
    "CandyDispenser"
    "new"
    Address("<OWNER_BADGE_ADDRESS>")
    Address("<BOT_BADGE_ADDRESS>")
    Address("<DCKSLAP_ADDRESS>")
    Address("<GBOF_ADDRESS>")
;
```

`<PACKAGE_ADDRESS>`: address of the package containing the `CandyDispenser` blueprint.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<BOT_BADGE_ADDRESS>`: the resource address of the bot badge.  
`<DCKSLAP_ADDRESS>`: resource address of the coin users will deposit to get an eventual reward.  
`<GBOF_ADDRESS>`: resource address of the coin users will deposit to level up.  

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
    <LEVEL>u64
    <RULE_NUMBER>u64
    <MIN_DCKSLAP_DEPOSITS>u32
    <EXACT_DCKSLAP_DEPOSIT_ONLY>
    <THRESHOLD>u16
    <CHOICE>u8
    Address("<COIN_ADDRESS>")
    <COIN_AMOUNT>u8
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
`<LEVEL>`: user level this rule applies to [0 - 12].  
`<RULE_NUMBER>`: the number of the rule to set [0 - 14].  
`<MIN_DCKSLAP_DEPOSITs>`: this rule can be only matched when a user has deposited at least this number of `DCKSLAP`.  
`<EXACT_DCKSLAP_DEPOSIT_ONLY>`: if `true` the rule will be matched only when a user has deposited exactly `<MIN_DCKSLAP_DEPOSITS>` `DCKSLAP`.  
`<THRESHOLD>`: this rule will be matched only if the random number reported by the bot is bigger or equal than this (0 = always, 1000 = never).   
`<CHOICE>`: this rule only applies to deposits where the user made the same choice [1 - 255], or to any possible choice (0).  
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
    <LEVEL>u64
    <RULE_NUMBER>u64
;
```

`<ACCOUNT_ADDRESS>`: address of the owner account.  
`<OWNER_BADGE_ADDRESS>`: the resource address of the owner badge.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
`<LEVEL>`: the level of the rule to delete [0 - 12].  
`<RULE_NUMBER>`: the number of the rule to delete [0 - 14].  

## `deposit_dckslap`
A user can deposit a single `DCKSLAP` by invoking this method.  

```
CALL_METHOD
    Address("<ACCOUNT_ADDRESS>")
    "withdraw"
    Address("<DCKSLAP_ADDRESS>")
    Decimal("1")
;
TAKE_ALL_FROM_WORKTOP
    Address("<DCKSLAP_ADDRESS>")
    Bucket("dckslap")
;
CALL_METHOD
    Address("<COMPONENT_ADDRESS>")
    "deposit_dckslap"
    Bucket("dckslap")
    Address("<ACCOUNT_ADDRESS>")
    <choice>u8
;
```

`<ACCOUNT_ADDRESS>`: account address of the depositing user.  
`<DCKSLAP_ADDRESS>`: resource address of the coin to deposit.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  
`<CHOICE>`: choice made by the user [1 - 255].  

This method emits a `DckslapDepositedEvent` which must be intercepted by a bot that will trigger the rewards distribution.  

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

`<ACCOUNT_ADDRESS>`: account address of the depositing user.  
`<GBOF_ADDRESS>`: resource address of the coin users will deposit to level up.  
`<COMPONENT_ADDRESS>`: the component created by the `new` function.  

This method emits a `UserLevelUpEvent` if the deposit causes the user to grow one level.  

## `deposit_xrd`
Use this method to deposit XRD to pay future users' transactions.  

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
