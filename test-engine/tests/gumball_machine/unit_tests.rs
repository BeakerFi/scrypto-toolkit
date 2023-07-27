mod gumball_machine_tests {
    use radix_engine::types::{dec, Decimal};
    use sdt_test_engine::env_args;
    use sdt_test_engine::environment::Environment;
    use sdt_test_engine::outcomes::Outcome;
    use sdt_test_engine::test_engine::TestEngine;

    #[test]
    fn test_buy_gumball_success() {
        let mut test_engine = TestEngine::new();
        test_engine.new_package("gumball package", "tests/gumball_machine/package");
        test_engine.new_component("gumball comp", "GumballMachine", "instantiate_gumball_machine", env_args!(dec!(5)));
        let receipt = test_engine.call_method("buy_gumball", env_args!(Environment::FungibleBucket("XRD", dec!(10))));
        receipt.assert_is_success();
        let amount_owned = test_engine.current_balance("GUM");
        assert_eq!(amount_owned, Decimal::one())
    }

    #[test]
    fn test_buy_gumball_fail() {
        let mut test_engine = TestEngine::new();
        test_engine.new_package("gumball package", "tests/gumball_machine/package");
        test_engine.new_component("gumball comp", "GumballMachine", "instantiate_gumball_machine", env_args!(dec!(5)));
        test_engine.call_method("buy_gumball", env_args!(Environment::FungibleBucket("XRD", dec!(1))));
        let amount_owned = test_engine.current_balance("GUM");
        assert_eq!(amount_owned, Decimal::zero())
    }


}