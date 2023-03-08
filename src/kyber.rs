use crate::utils::sort_tokens;
use ethers::{
    abi::{ethabi, Token},
    prelude::*,
    utils::{get_create2_address_from_hash, keccak256},
};
use std::str::FromStr;

const FACTORY_ADDRESS: &str = "0x5F1dddbf348aC2fbe22a163e30F99F9ECE3DD50a";
const POOL_INIT_CODE_HASH: &str =
    "0xc597aba1bb02db42ba24a8878837965718c032f8b46be94a6e46452a9f89ca01";

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
    fn test_get_univ3_address_usdc_weth() {
        let token_a = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
        let token_b = Address::from_str("0x7f39C581F595B53c5cb19bD0b3f8dA6c935E2Ca0").unwrap();
        let fee = 8;
        let expected = Address::from_str("0xebfe63ba0264ad639b3c41d2bfe1ad708f683bc8").unwrap();
        let actual = get_pool_address(token_a, token_b, fee);
        assert_eq!(expected, actual);
        let actual = get_pool_address(token_b, token_a, fee);
        assert_eq!(expected, actual);
    }
}
