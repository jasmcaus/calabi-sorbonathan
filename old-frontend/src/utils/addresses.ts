import { AddressMap } from "types"
import { ChainId } from "utils/chains"

export const ZERO_ADDRESS = "0x0000000000000000000000000000000000000000"

export const Multicall2 = "0x9Be8177Ab72E8904eE01a6db0B3c9695a8b3AA9F"
export const FAUCET = "0x83Ec95175F070682c2647986E74170d0F94a0DEA"
export const MOCK_ADAI = "0x6E14e096c41b267466733429654acA6bf783104d"
export const MOCK_AAVE = "0xA0Ebb346a1a65502Af191CC75f079b0D3540f631"
export const MOCK_ICY = "0x0FD242BC3Fd926dac287e7a0236ae88cc230F6ce"
export const DELTA_INSURANCE = "0x3c949aC99D2B5fC7D2e32e3D5035Ed81008839f6"
export const DLT = "0xF58CdB87c6108703f8A6585507Eb3e2CF628A5C7"
export const rMAV = "0x0662e6AdACA761b94c8447F5846601c3a446C17e"
export const rFRA = "0xbC48C36A8a0b404ee7cd2E9A8D5220355Bd02897"
export const USDC = "0x61460B92116D3381bD4D396b3652C8866a6fd480"
export const USDT = "0x90b4B0653769B3c87900999aa55E0D17E0628B57"
export const DAI = "0x7361fC34b8fb29c930Ca7b2E7dAfeD8fe8E16cD2"
export const rMAV_ORACLE = "0xB62C3dBDb6A19dc2aF25c85c0665498D35889Ff4"
export const rFRA_ORACLE = "0xAa46bF640fC58fa5aF093C88b156426D41Ecf17D"
export const CHAINLINK = "0x8c8e3EfeF6D3951DecC3b59806eCe94e9cf693B6"
export const DELTA_OPEN = "0xcacF82757DF9716f4E69dDE5be8B1545e21c58f4"
export const DELTA_VERIFIED = "0xcacF82757DF9716f4E69dDE5be8B1545e21c58f4"
export const REWARD_CONTROL = "0x603518Ad1081966fd877a8E6cf9A5DF064e3b52B"
export const RATE_MODEL = "0x09cE8E01839261E40b7F3eb2315601eF1991Ea69"

// The deployer version is also related to the protocol version
export const DEPLOYER_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x9cAc8ED3297040626D8aA6317F5e29813A6A8fc6",
    [ChainId.Goerli]: "0x6D80CEE7065848233d81c7621C736149a6666979",
    [ChainId.DeltaSaga]: "0x0000000000000000000000000000000000000000",
    [ChainId.Hardhat]: "0x139e1D41943ee15dDe4DF876f9d0E7F85e26660A",
}

export const FACADE_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x3DAf5a7681a9cfB92fB38983EB3998dFC7963B28",
    [ChainId.Goerli]: "0x8B84160CF8d9066Ba45f71471a06F2BFAc364626",
    [ChainId.Hardhat]: "0xD6b040736e948621c5b6E0a494473c47a6113eA8",
}

export const FACADE_WRITE_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0xF7bd1F8FdE9fBdc8436D45594e792e014c5ac966",
    [ChainId.Goerli]: "0x261bccA3a9E67bDd1e5A1a3D72c5e3393843c824",
    [ChainId.Hardhat]: "0x7B4f352Cd40114f12e82fC675b5BA8C7582FC513",
}

export const STAKE_AAVE_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x1895534f4D0fF3E632407DB5327cE3eAfCe2A9f6",
    [ChainId.Goerli]: "0x7b5109144EA8cC4903dE7F85012515D603226d93",
    [ChainId.Hardhat]: "0x82EdA215Fa92B45a3a76837C65Ab862b6C7564a8",
}

export const COMPOUND_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0xd9e49644f35b5233B5191b14882242bd0bdF58E8",
    [ChainId.Goerli]: "0x7e1Ee9185877Aaa0bF9689478Ff8e706eea1BD31",
    [ChainId.Hardhat]: "0x87006e75a5B6bE9D1bbF61AC8Cd84f05D9140589",
}

export const RSR_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x320623b8e4ff03373931769a31fc52a4e78b5d70",
    [ChainId.Goerli]: "0xB58b5530332D2E9e15bfd1f2525E6fD84e830307",
    [ChainId.Hardhat]: "0x5FbDB2315678afecb367f032d93F642f64180aa3",
}

export const MULTICALL_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x5ba1e12693dc8f9c48aad8770482f4739beed696",
    [ChainId.Goerli]: "0x5BA1e12693Dc8F9c48aAD8770482f4739bEeD696",
    [ChainId.DeltaSaga]: "0x28629083fC4E1e4CBf6e268Bf628F44d9dDcEdD8",
    [ChainId.Hardhat]: "0x5ba1e12693dc8f9c48aad8770482f4739beed696",
}

export const RSV_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x196f4727526eA7FB1e17b2071B3d8eAA38486988",
    [ChainId.Goerli]: "0xC54cA3D2A4fE68D079b45c92D703DADfE3Ad0AA0",
    [ChainId.DeltaSaga]: "0x0000000000000000000000000000000000000000",
    [ChainId.Hardhat]: "0x196f4727526eA7FB1e17b2071B3d8eAA38486988",
}

export const RSV_MANAGER_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x4B481872f31bab47C6780D5488c84D309b1B8Bb6",
    [ChainId.Goerli]: "0x08d95a020cE6FCfF46ACb323E2416Bc847D68b9a",
    [ChainId.Hardhat]: "0x4B481872f31bab47C6780D5488c84D309b1B8Bb6",
}

export const USDC_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
    [ChainId.Goerli]: "0xfd7201C314532c4eF42CBF3fcB4A2f9CfCe0f57A",
    [ChainId.Hardhat]: "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48",
}

export const TUSD_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x0000000000085d4780B73119b644AE5ecd22b376",
    [ChainId.Goerli]: "0xc6aA873112Ff1628a4b8512c5Cb666F2E3B4FD6A",
    [ChainId.Hardhat]: "0x0000000000085d4780B73119b644AE5ecd22b376",
}

export const PAX_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x8E870D67F660D95d5be530380D0eC0bd388289E1",
    [ChainId.Goerli]: "0x1e0D00502E0DB65084EEaf95b525574E30DE41C5",
    [ChainId.Hardhat]: "0x8E870D67F660D95d5be530380D0eC0bd388289E1",
}

export const BUSD_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x4Fabb145d64652a948d72533023f6E7A623C7C53",
    [ChainId.Goerli]: "0x4Fabb145d64652a948d72533023f6E7A623C7C53",
    [ChainId.Hardhat]: "0x4Fabb145d64652a948d72533023f6E7A623C7C53",
}

export const WETH_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    [ChainId.Goerli]: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
    [ChainId.Hardhat]: "0xc02aaa39b223fe8d0a0e5c4f27ead9083c756cc2",
}

export const ORACLE_ADDRESS: AddressMap = {
    [ChainId.Mainnet]: "0x8263e161A855B644f582d9C164C66aABEe53f927",
    [ChainId.Goerli]: "0x8263e161A855B644f582d9C164C66aABEe53f927",
    [ChainId.Hardhat]: "0x8263e161A855B644f582d9C164C66aABEe53f927",
}

// Fixed tokens used in the rtoken selector screen and dashboard
export const DEFAULT_TOKENS = {
    [ChainId.Mainnet]: [RSV_ADDRESS[ChainId.Mainnet]],
    [ChainId.Goerli]: [RSV_ADDRESS[ChainId.Goerli]],
    [ChainId.Hardhat]: [],
}
