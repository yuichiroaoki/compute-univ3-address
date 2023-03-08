use crate::utils::sort_tokens;
use ethers::{
    abi::{ethabi, Token},
    prelude::*,
    utils::{get_create2_address_from_hash, keccak256},
};
use std::str::FromStr;

const FACTORY_ADDRESS: &str = "0x1F98431c8aD98523631AE4a59f267346ea31F984";
// ref. https://github.com/Uniswap/v3-sdk/blob/08a7c050cba00377843497030f502c05982b1c43/src/constants.ts#L5
const POOL_INIT_CODE_HASH: &str =
    "0xe34f199b19b2b4f47f68442619d555527d244f78a3297ea89325f843f87b8b54";

// ref. https://github.com/Uniswap/v3-sdk/blob/main/src/utils/computePoolAddress.ts
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
    fn test_get_univ3_address_usdc_wbtc() {
        let token_a = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
        let token_b = Address::from_str("0x2260FAC5E5542a773Aa44fBCfeDf7C193bc2C599").unwrap();
        let fee = 500;
        let expected = Address::from_str("0x9a772018FbD77fcD2d25657e5C547BAfF3Fd7D16").unwrap();
        let actual = get_pool_address(token_a, token_b, fee);
        assert_eq!(expected, actual);
        let actual = get_pool_address(token_b, token_a, fee);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_get_univ3_address_usdc_weth() {
        let token_a = Address::from_str("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48").unwrap();
        let token_b = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
        let fee = 500;
        let expected = Address::from_str("0x88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640").unwrap();
        let actual = get_pool_address(token_a, token_b, fee);
        assert_eq!(expected, actual);
        let actual = get_pool_address(token_b, token_a, fee);
        assert_eq!(expected, actual);
    }
}
