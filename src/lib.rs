use scrypto::prelude::*;

static RULES: usize = 10;

#[derive(ScryptoSbor)]
struct Rule {
    min_deposits: u32,
    exact_deposit_only: bool,
    threshold: u16,
    candy_address: ResourceAddress,
    amount: u8,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct DckslapDepositedEvent {
    account: Global<Account>,
}

#[blueprint]
#[types(
    ResourceAddress,
    Vault,
    Global<Account>,
    u32,
)]
#[events(
    DckslapDepositedEvent,
)]
mod candy_dispenser {
    enable_method_auth! {
        roles {
            bot => updatable_by: [OWNER];
        },
        methods {
            deposit_candy => restrict_to: [OWNER];
            set_rule => restrict_to: [OWNER];
            unset_rule => restrict_to: [OWNER];
            deposit_gbof => PUBLIC;
            send_candies => restrict_to: [bot];
            deposit_xrd => PUBLIC;
        }
    }

    struct CandyDispenser {
        candy_vaults: KeyValueStore<ResourceAddress, Vault>,
        gbof_vault: Vault,
        deposited_gbof: KeyValueStore<Global<Account>, u32>,
        rules: Vec<Option<Rule>>,
        xrd_vault: FungibleVault,
    }

    impl CandyDispenser {
        pub fn new(
            owner_badge_address: ResourceAddress,
            bot_badge_address: ResourceAddress,
            gbof_address: ResourceAddress,
        ) -> Global<CandyDispenser> {
            let mut rules:Vec<Option<Rule>> = vec![];
            for _i in 0..RULES {
                rules.push(None);
            }
            Self {
                candy_vaults: KeyValueStore::new_with_registered_type(),
                gbof_vault: Vault::new(gbof_address),
                deposited_gbof: KeyValueStore::new_with_registered_type(),
                rules: rules,
                xrd_vault: FungibleVault::new(XRD),
            }
            .instantiate()
            .prepare_to_globalize(OwnerRole::Updatable(rule!(require(owner_badge_address))))
            .roles(roles!(
                bot => rule!(require(bot_badge_address));
            ))
            .globalize()
        }

        fn pay_fees(
            &mut self,
            amount: Decimal,
        ) {
            if self.xrd_vault.amount() >= amount {
                self.xrd_vault.lock_contingent_fee(amount);
            }
        }

        pub fn deposit_candy(
            &mut self,
            candy_bucket: Bucket,
        ) {
            let candy_address = candy_bucket.resource_address();

            let opt_vault = self.candy_vaults.get_mut(&candy_address);
            match opt_vault {
                Some(mut vault) => vault.put(candy_bucket),
                None => {
                    drop(opt_vault);
                    let vault = Vault::with_bucket(candy_bucket);
                    self.candy_vaults.insert(candy_address, vault);
                },
            }
        }

        pub fn set_rule(
            &mut self,
            rule_number: usize,
            min_deposits: u32,
            exact_deposit_only: bool,
            threshold: u16,
            candy_address: ResourceAddress,
            amount: u8,
        ) {
            assert!(
                rule_number < RULES,
                "Rule number out of range"
            );

            self.rules[rule_number] = Some(Rule {
                min_deposits: min_deposits,
                exact_deposit_only: exact_deposit_only,
                threshold: threshold,
                candy_address: candy_address,
                amount: amount,
            });
        }

        pub fn unset_rule(
            &mut self,
            rule_number: usize,
        ) {
            assert!(
                rule_number < RULES,
                "Rule number out of range"
            );

            self.rules[rule_number] = None;
        }

        pub fn deposit_gbof(
            &mut self,
            mut gbof_bucket: Bucket,
            account: Global<Account>,
        ) -> Bucket {
            self.pay_fees(dec!(1));

            self.gbof_vault.put(
                gbof_bucket.take(1)
            );

            let deposited = self.deposited_gbof.get_mut(&account);
            match deposited {
                Some(mut number) => {
                    *number += 1;
                },
                None => {
                    drop(deposited);
                    let number = 1;
                    self.deposited_gbof.insert(account, number);
                },
            }

            Runtime::emit_event(
                DckslapDepositedEvent {
                    account: account,
                }
            );

            gbof_bucket
        }
        
        pub fn send_candies(
            &mut self,
            random_number: u16,
            mut account: Global<Account>,
        ) {
            self.pay_fees(dec!(10));

            let deposited = *self.deposited_gbof.get(&account).unwrap();

            for i in 0..RULES {
                match &self.rules[i] {
                    Some(rule) => {
                        if deposited >= rule.min_deposits &&
                            random_number >= rule.threshold &&
                            ((deposited == rule.min_deposits) >= rule.exact_deposit_only) {
                                let mut vault = self.candy_vaults.get_mut(&rule.candy_address).unwrap();
                                let bucket = vault.take(rule.amount);
                                account.try_deposit_or_abort(bucket, None);
                        }
                    },
                    None => {},
                }
            }
        }

        pub fn deposit_xrd(
            &mut self,
            xrd_bucket: FungibleBucket,
        ) {
            self.xrd_vault.put(xrd_bucket);
        }
    }
}
