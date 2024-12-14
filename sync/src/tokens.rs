use crate::entities::token_on_ledger;
use crate::{chains::*, with_canister, Mutation, Query};
use candid::{Decode, Encode};
use candid::{Nat, Principal};
use icrc_ledger_types::icrc1::account::Account;
use log::info;
use sea_orm::DbConn;
use std::error::Error;

pub async fn sync_cketh(db: &DbConn) -> Result<(), Box<dyn Error>> {
    with_canister("CKETH_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKETH canister ledgers... ");

		let cketh_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&cketh_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let cketh_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckETH".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x242BbcB4f4F1b752Ae30757DC9AE9C24d9A9B640").await?;
		let e_amount = bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKETH".to_string(),
			18_i16,
			e_amount.to_string(),
			cketh_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_ckbtc(db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("CKBTC_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKBTC canister ledgers... ");

		let ckbtc_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&ckbtc_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let ckbtc_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckBTC".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let osmosis = sync_with_osmosis(
			"factory%2Fosmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm%2FsICP-icrc-ckBTC",
		)
		.await?;
		let bitfinity = sync_with_bitfinity("0xFD4dE66ECA49799bDdE66eB33654E2198Ab7bba4").await?;
		let e_amount = osmosis.parse::<u128>().unwrap() + bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKBTC".to_string(),
			8_i16,
			e_amount.to_string(),
			ckbtc_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_ckusdt(db: &DbConn) -> Result<(), Box<dyn Error>> {
    with_canister("CKUSDT_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on CKUSDT canister ledgers... ");

		let ckusdt_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&ckusdt_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let ckusdt_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-ckUSDT".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0xe613EBD1eAe99D824Da8A6C33eC833A62bC04B5a").await?;
		let e_amount = bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"CKUSDT".to_string(),
			6_i16,
			e_amount.to_string(),
			ckusdt_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_neuron_icp(db: &DbConn) -> Result<(), Box<dyn Error>> {
    with_canister("NEURON_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on NEURON canister ledgers... ");

		let nicp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&nicp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let nicp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-nICP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x2a78A5f819393105a54F21AdeB4a8b68C5030b02").await?;
		let e_amount = bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"neuron ICP".to_string(),
			8_i16,
			e_amount.to_string(),
			nicp_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_dragginz(db: &DbConn) -> Result<(), Box<dyn Error>> {
    with_canister("DRAGGIN_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on NEURON canister ledgers... ");

		let nicp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&nicp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let nicp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-icrc-DKP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let bitfinity = sync_with_bitfinity("0x6286e8464E2817818EF8c3353e91824f680354d2").await?;
		let e_amount = bitfinity.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"Draggin Karma Points".to_string(),
			8_i16,
			e_amount.to_string(),
			nicp_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_icp(db: &DbConn) -> Result<(), Box<dyn Error>> {
    with_canister("ICP_CANISTER_ID", |agent, canister_id| async move {
		info!("syncing tokens on ICP canister ledgers... ");

		let icp_reqst = Account {
			owner: Principal::from_text("nlgkm-4qaaa-aaaar-qah2q-cai".to_string())?,
			subaccount: None,
		};
		let arg = Encode!(&icp_reqst)?;
		let ret = agent
			.query(&canister_id, "icrc1_balance_of")
			.with_arg(arg)
			.call()
			.await?;
		let icp_amount = Decode!(&ret, Nat)?.to_string().replace("_", "");

		let mut hub_amount = 0;
		for tamount in Query::get_all_amount_by_token(db, "sICP-native-ICP".to_string()).await? {
			hub_amount += tamount.amount.parse::<u128>().unwrap_or(0)
		}

		let osmosis = sync_with_osmosis(
			"factory/osmo10c4y9csfs8q7mtvfg4p9gd8d0acx0hpc2mte9xqzthd7rd3348tsfhaesm/sICP-native-ICP",
		)
		.await?;
		let bitfinity = sync_with_bitfinity("0x51cCdE9Ca75d95BB55eCe1775fCBFF91324B18A6").await?;
        let ethereum = sync_with_ethereum("0x51cCdE9Ca75d95BB55eCe1775fCBFF91324B18A6", "275CTXW29UE4Q7219PX6AQ1I1PJZRH9H7P").await?;
		let e_amount = osmosis.parse::<u128>().unwrap() + bitfinity.parse::<u128>().unwrap() + ethereum.parse::<u128>().unwrap();

		let token_on_ledger = token_on_ledger::Model::new(
			"sICP".to_string(),
			"ICP".to_string(),
			8_i16,
			e_amount.to_string(),
			icp_amount,
			hub_amount.to_string(),
		);
		Mutation::save_token_on_ledger(db, token_on_ledger).await?;

		Ok(())
	})
	.await
}

pub async fn sync_rich(_db: &DbConn) -> Result<(), Box<dyn Error>> {
	with_canister("EICP_HOPE_YOU_GET_RICH", |agent, canister_id| async move {
		info!("syncing tokens on HOPE_YOU_GET_RICH canister ledgers... ");

		let arg = Encode!(&Vec::<u8>::new())?;
		let ret = agent
			.query(&canister_id, "icrc1_total_supply")
			.with_arg(arg)
			.call()
			.await?;
		let _amount = Decode!(&ret, Nat)?.to_string().replace("_", "");
		// println!("{:?}", amount);
		Ok(())
	})
	.await
}