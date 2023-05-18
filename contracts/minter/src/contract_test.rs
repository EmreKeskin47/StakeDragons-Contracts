#[cfg(test)]
mod tests {
    //use super::*;
    //use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    //use cosmwasm_std::{coins, from_binary};
    //use schemars::_serde_json::to_string;

    /*
    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(1000, "earth"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(17, value.count);
    }

    #[test]
    fn increment() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Increment {};
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // should increase counter by 1
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(18, value.count);
    }

    #[test]
    fn reset() {
        let mut deps = mock_dependencies();

        let msg = InstantiateMsg { count: 17 };
        let info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        // beneficiary can release it
        let unauth_info = mock_info("anyone", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let res = execute(deps.as_mut(), mock_env(), unauth_info, msg);
        match res {
            Err(ContractError::Unauthorized {}) => {}
            _ => panic!("Must return unauthorized error"),
        }

        // only the original creator can reset the counter
        let auth_info = mock_info("creator", &coins(2, "token"));
        let msg = ExecuteMsg::Reset { count: 5 };
        let _res = execute(deps.as_mut(), mock_env(), auth_info, msg).unwrap();

        // should now be 5
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetCount {}).unwrap();
        let value: GetCountResponse = from_binary(&res).unwrap();
        assert_eq!(5, value.count);
    }


     */

    #[test]
    fn split() {
        let random_key = 79235;
        let id = "G2CVVGS5725B2V2NV5NSG215G2G5S5SCV95NSNG75BNNC2V29S1V2G5GV72VG25297L5572GVNBL2VSN5NGV9VNVVC925B9722GV".to_string();

        //dragon type
        let key1 = random_key / 10000;
        let key2 = random_key % 100;
        let secret = key1 + key2;
        let type_id = id.chars().nth(secret as usize).unwrap();

        let type_name;
        match type_id {
            'G' => type_name = "fire",
            '2' => type_name = "fire",
            'N' => type_name = "storm",
            '5' => type_name = "storm",
            'V' => type_name = "udin",
            'S' => type_name = "udin",
            '7' => type_name = "divine",
            '1' => type_name = "divine",
            _ => type_name = "ice",
        };

        println!("{}", type_name);

        /* let egg_id = "8";
        let a: i32 = egg_id.parse().unwrap();
        let img_id = a % 17;
        println!("image id test {}", img_id);

        let url = "https://bafybeihuaoctl3lhtnzg26swjud742i4kwxlrtm63n6r57353oi7sqyohy.ipfs.nftstorage.link/".to_string() + &*(img_id + 1).to_string() + &*".png".to_string();
        println!("image url test {}", url);

        let split = "AJWPJP538P3AJ3W4234W4W24P34Q2PJWWQA2534PW5QKPPJ53J00009";
        let key = 47;

        let test1 = "8349026128";
        let test2 = &test1[test1.len() - 5..test1.len() - 3];
        println!("test2 print {}", test2);

        let test47 = split.chars().nth(key.parse().unwrap()).unwrap();
        println!("split chars type {}", test47);

        let asd = "asdasdasd ".to_string() + split.clone();
        println!("{}", asd);

        let ch3 = &split[split.len() - 5..split.len()];
        println!("{}", ch3);

        let type_id = split.chars().nth(key).unwrap();
        let type_name;
        match type_id {
            '2' => type_name = "uncommon",
            '3' => type_name = "rare",
            '4' => type_name = "epic",
            '5' => type_name = "legendary",
            'P' => type_name = "legendary",
            'K' => type_name = "epic",
            'G' => type_name = "rare",
            'W' => type_name = "uncommon",
            _ => type_name = "common",
        };
        println!("{}", type_name);

        */
    }
}
