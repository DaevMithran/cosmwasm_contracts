use cosmwasm_std::{coin, coins, Addr};
use cw_multi_test::App;

use crate::{error::ContractError, msg::ValueResp, multitest::contract::CountingContract};

const CHEQ: &str = "cheq";

#[test]
fn query_value() {
    let sender = Addr::unchecked("sender");
    // blockchain simulator
    let mut app = App::new(|router, _api, storage| {
        router
            .bank
            .init_balance(storage, &sender, coins(1000, CHEQ))
            .unwrap()
    });

    let contract_id = CountingContract::store_code(&mut app);

    let contract =
        CountingContract::instantiate(&mut app, contract_id, &sender, 10, coin(10, CHEQ)).unwrap();

    let resp: ValueResp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 10 });

    let _ = contract.increment(&mut app, &sender);

    let resp: ValueResp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 11 });

    let _ = contract.donate(&mut app, &sender, &[]);

    let resp: ValueResp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 11 });

    let _ = contract.donate(&mut app, &sender, &coins(100, CHEQ));

    let resp: ValueResp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 111 });

    let err = contract
        .withdraw(&mut app, &Addr::unchecked("random"), &[])
        .unwrap_err();

    assert_eq!(
        ContractError::Unauthorized {
            owner: sender.clone().into()
        },
        err
    );

    let _res = contract.withdraw(&mut app, &sender, &[]).unwrap();
    let resp: ValueResp = contract.query_value(&app).unwrap();
    assert_eq!(resp, ValueResp { value: 0 });
}
