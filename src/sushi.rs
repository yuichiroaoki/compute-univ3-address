use ethers::types::Address;

use super::common;

// ref. https://github.com/sushiswap/sushiswap/blob/da2f7ecc6da132a740bc14eb3dab9c19cc485b37/packages/v2-sdk/src/constants.ts

const FACTORY_ADDRESS_MAINNET: &str = "0xC0AEe478e3658e2610c5F7A4A2E1777cE9e4f2Ac";
const FACTORY_ADDRESS_OTHER: &str = "0xc35DADB65012eC5796536bD9864eD8773aBc74C4";
const POOL_INIT_CODE_HASH: &str =
    "0xe18a34eb0e04b04f7a0ac29a6e80748dca96319b42c54d679cb821dca90c6303";

pub fn get_pool_address(token_in: Address, token_out: Address) -> Address {
    common::get_pool_address_with_encode_packed(
        FACTORY_ADDRESS_OTHER,
        POOL_INIT_CODE_HASH,
        token_in,
        token_out,
    )
}

pub fn get_pool_address_mainnet(token_in: Address, token_out: Address) -> Address {
    common::get_pool_address_with_encode_packed(
        FACTORY_ADDRESS_MAINNET,
        POOL_INIT_CODE_HASH,
        token_in,
        token_out,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_pool_address() {
        // Ethereum mainnet
        // WETH
        let token_a = "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"
            .parse::<Address>()
            .unwrap();
        // USDC
        let token_b = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
            .parse::<Address>()
            .unwrap();
        let pool_address = get_pool_address_mainnet(token_a, token_b);
        assert_eq!(
            pool_address,
            "0x397FF1542f962076d0BFE58eA045FfA2d347ACa0"
                .parse::<Address>()
                .unwrap()
        );

        // Polygon
        // WETH
        let token_a = "0x7ceB23fD6bC0adD59E62ac25578270cFf1b9f619"
            .parse::<Address>()
            .unwrap();
        // USDC
        let token_b = "0x2791Bca1f2de4661ED88A30C99A7a9449Aa84174"
            .parse::<Address>()
            .unwrap();
        let pool_address = get_pool_address(token_a, token_b);
        assert_eq!(
            pool_address,
            "0x34965ba0ac2451A34a0471F04CCa3F990b8dea27"
                .parse::<Address>()
                .unwrap()
        );
    }
}
