use crate::utils::sort_tokens;
use ethers::{
    abi::{self, ethabi, Token},
    prelude::*,
    utils::{get_create2_address_from_hash, keccak256},
};
use std::str::FromStr;

pub fn get_pool_address_with_encode_packed(
    factory: &str,
    init_code_hash: &str,
    token_in: Address,
    token_out: Address,
) -> Address {
    let (token_a, token_b) = sort_tokens(token_in, token_out);
    let factory_addr = Address::from_str(factory).unwrap();
    let init_code = Bytes::from_str(init_code_hash).unwrap();
    get_create2_address_from_hash(
        factory_addr,
        keccak256(abi::encode_packed(&[Token::Address(token_a), Token::Address(token_b)]).unwrap()),
        init_code,
    )
}

pub fn get_pool_address_with_encode(
    factory: &str,
    init_code_hash: &str,
    token_in: Address,
    token_out: Address,
) -> Address {
    let (token_a, token_b) = sort_tokens(token_in, token_out);
    let factory_addr = Address::from_str(factory).unwrap();
    let init_code = Bytes::from_str(init_code_hash).unwrap();
    get_create2_address_from_hash(
        factory_addr,
        keccak256(ethabi::encode(&[
            Token::Address(token_a),
            Token::Address(token_b),
        ])),
        init_code,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pool_address() {
        // Algebra
        const DEPLOYER: &str = "0x2D98E2FA9da15aa6dC9581AB097Ced7af697CB92";
        const POOL_INIT_CODE_HASH: &str =
            "0x6ec6c9c8091d160c0aa74b2b14ba9c1717e95093bd3ac085cee99a49aab294a4";

        // Polygon
        // WETH
        let token_a = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"
            .parse::<Address>()
            .unwrap();
        // USDC
        let token_b = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
            .parse::<Address>()
            .unwrap();
        let pool_address =
            get_pool_address_with_encode(DEPLOYER, POOL_INIT_CODE_HASH, token_a, token_b);
        assert_eq!(
            pool_address,
            "0x55CAaBB0d2b704FD0eF8192A7E35D8837e678207"
                .parse::<Address>()
                .unwrap()
        );
    }
}
