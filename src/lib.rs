use scrypto::prelude::*;

#[derive(ScryptoSbor)]
struct RulePosition {
    level: u8,
    rule_number: u16,
}

#[derive(ScryptoSbor, Clone)]
struct Rule {
    threshold: u32,
    choice: u16,
    candy_address: ResourceAddress,
    amount: u8,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct DckslapDepositedEvent {
    account: Global<Account>,
    choice: u16,
}

#[derive(ScryptoSbor, ScryptoEvent)]
struct UserLevelUpEvent {
    account: Global<Account>,
    level: u8,
}

#[derive(ScryptoSbor, Clone)]
struct User {
    level: u8,
    deposited_gbof: u8,
}

#[blueprint]
#[types(
    ResourceAddress,
    Vault,
    Global<Account>,
    User,
    RulePosition,
    Rule,
)]
#[events(
    DckslapDepositedEvent,
    UserLevelUpEvent,
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
            deposit_dckslap => PUBLIC;
            deposit_gbof => PUBLIC;
            send_candies => restrict_to: [bot];
            deposit_xrd => PUBLIC;
        }
    }

    struct CandyDispenser {
        candy_vaults: KeyValueStore<ResourceAddress, Vault>,
        dckslap_vault: Vault,
        gbof_vault: Vault,
        users: KeyValueStore<Global<Account>, User>,
        rules: KeyValueStore<RulePosition, Rule>,
        xrd_vault: FungibleVault,
    }

    impl CandyDispenser {
        pub fn new(
            owner_badge_address: ResourceAddress,
            bot_badge_address: ResourceAddress,
            dckslap_address: ResourceAddress,
            gbof_address: ResourceAddress,
        ) -> Global<CandyDispenser> {
            Self {
                candy_vaults: KeyValueStore::new_with_registered_type(),
                dckslap_vault: Vault::new(dckslap_address),
                gbof_vault: Vault::new(gbof_address),
                users: KeyValueStore::new_with_registered_type(),
                rules: KeyValueStore::new_with_registered_type(),
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
            level: u8,
            rule_number: u16,
            threshold: u32,
            choice: u16,
            candy_address: ResourceAddress,
            amount: u8,
        ) {
            self.rules.insert(
                RulePosition {
                    level: level,
                    rule_number: rule_number,
                },
                Rule {
                    threshold: threshold,
                    choice: choice,
                    candy_address: candy_address,
                    amount: amount,
                }
            );
        }

        pub fn unset_rule(
            &mut self,
            level: u8,
            rule_number: u16,
        ) {
            self.rules.remove(
                &RulePosition {
                    level: level,
                    rule_number: rule_number,
                }
            );
        }

        pub fn deposit_dckslap(
            &mut self,
            mut dckslap_bucket: Bucket,
            account: Global<Account>,
            choice: u16,
        ) -> Bucket {
            self.pay_fees(dec!(1));

            Runtime::assert_access_rule(account.get_owner_role().rule);

            assert!(
                choice > 0,
                "Wrong choice"
            );

            self.dckslap_vault.put(
                dckslap_bucket.take(1)
            );

            let user = self.users.get(&account);
            if user.is_none() {
                self.users.insert(
                    account,
                    User {
                        level: 0,
                        deposited_gbof: 0,
                    }
                );
            }

            Runtime::emit_event(
                DckslapDepositedEvent {
                    account: account,
                    choice: choice,
                }
            );

            dckslap_bucket
        }
        
        pub fn deposit_gbof(
            &mut self,
            mut gbof_bucket: Bucket,
            account: Global<Account>,
        ) -> Bucket {
            self.pay_fees(dec!(1));

            Runtime::assert_access_rule(account.get_owner_role().rule);

            self.gbof_vault.put(
                gbof_bucket.take(1)
            );

            let user = self.users.get_mut(&account);
            match user {
                Some(mut user) => {

                    assert!(
                        user.level < u8::MAX,
                        "Maximum level reached"
                    );

                    if user.deposited_gbof == user.level {
                        user.level += 1;
                        user.deposited_gbof = 0;

                        Runtime::emit_event(
                            UserLevelUpEvent {
                                account: account,
                                level: user.level,
                            }
                        );
                    } else {
                        user.deposited_gbof += 1;
                    }
                },
                None => {
                    drop(user);

                    self.users.insert(
                        account,
                        User {
                            level: 1,
                            deposited_gbof: 0,
                        }
                    );
                    
                    Runtime::emit_event(
                        UserLevelUpEvent {
                            account: account,
                            level: 1,
                        }
                    );
                },
            }

            gbof_bucket
        }

        pub fn send_candies(
            &mut self,
            random_number: u32,
            mut account: Global<Account>,
            choice: u16,
        ) {
            self.pay_fees(dec!(10));

            let user = self.users.get(&account).unwrap().clone();

            let mut rule_position = RulePosition {
                level: user.level,
                rule_number: 0,
            };
            loop {
                match self.rules.get(&rule_position) {
                    Some(rule) => {
                        if random_number >= rule.threshold &&
                            (choice == rule.choice || rule.choice == 0) {
                                let mut vault = self.candy_vaults.get_mut(&rule.candy_address).unwrap();
                                let bucket = vault.take(rule.amount);
                                account.try_deposit_or_abort(bucket, None);
                        }

                        rule_position.rule_number += 1;
                    },
                    None => break,
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
