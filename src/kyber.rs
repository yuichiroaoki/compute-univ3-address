use crate::utils::sort_tokens;
use ethers::{
    abi::{ethabi, Token},
    prelude::*,
    utils::{get_create2_address_from_hash, keccak256},
};
use std::str::FromStr;

const FACTORY_ADDRESS: &str = "0xC7a590291e07B9fe9E64b86c58fD8fC764308C4A";
const POOL_INIT_CODE_HASH: &str =
    "0x00e263aaa3a2c06a89b53217a9e7aad7e15613490a72e0f95f303c4de2dc7045";

pub fn get_pool_address(token_a: Address, token_b: Address, fee: u32) -> Address {
    let (token0, token1) = sort_tokens(token_a, token_b);
    let factory_addr = Address::from_str(FACTORY_ADDRESS).unwrap();
    let init_code = Bytes::from_str(POOL_INIT_CODE_HASH).unwrap();
    get_create2_address_from_hash(
        factory_addr,
        keccak256(ethabi::encode(&[
            Token::Address(token0),
            Token::Address(token1),
            Token::Uint(U256::from(fee)),
        ])),
        init_code,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_univ3_address_usdc_dai() {
        let token_a = Address::from_str("0x7F5c764cBc14f9669B88837ca1490cCa17c31607").unwrap();
        let token_b = Address::from_str("0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1").unwrap();
        let fee = 8;
        let expected = Address::from_str("0x80d39502FC199A5094cf231413F5c20D9ee244c4").unwrap();
        let actual = get_pool_address(token_a, token_b, fee);
        assert_eq!(expected, actual);
        let actual = get_pool_address(token_b, token_a, fee);
        assert_eq!(expected, actual);
    }
}
