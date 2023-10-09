import fs from "fs";
import hre from "hardhat";
import { ethers } from "hardhat";
import { getChainId } from "../../../common/blockchain-utils";
import { getDeploymentFilename } from "../common";

async function main() {
  const [sender] = await hre.ethers.getSigners();
  const chainId = await getChainId(hre);
  // console.log(hre.network.config.url)

  let deployments: any = {};
  const deploymentFilename = getDeploymentFilename(
    chainId,
    "-delta-deployments.json"
  );
  deployments = JSON.parse(fs.readFileSync(deploymentFilename).toString());

  const provider = new ethers.providers.JsonRpcProvider(
    (hre.network.config as any).url
  );
  const nonce = await provider.getTransactionCount(sender.address); // Correction
  console.debug("Nonce:", nonce);
  let i = 0;

  // const Multicall2 = await ethers.getContractFactory("Multicall2")
  // const multicall2 = await Multicall2.deploy({ nonce: nonce + i++ })
  // deployments.Multicall2 = multicall2.address

  // const Faucet = await ethers.getContractFactory("Faucet")
  // const faucet = await Faucet.deploy( { nonce: nonce + i++ })
  // deployments.FAUCET = faucet.address

  const MockERC20 = await ethers.getContractFactory("MockERC20");
  const MockAave = await ethers.getContractFactory("MockAave");
  const MockICY = await ethers.getContractFactory("MockICY");

  // const mockaDai = await MockERC20.deploy("aDAI", "aDAI", 18, { nonce: nonce + i++ })
  // const mockaave = await MockAave.deploy({ nonce: nonce + i++ })
  // const mockicy = await MockICY.deploy({ nonce: nonce + i++ })
  // deployments.MOCK_ADAI = mockaDai.address
  // deployments.MOCK_AAVE = mockaave.address
  // deployments.MOCK_ICY = mockicy.address

  const DeltaInsurance = await ethers.getContractFactory("DeltaInsurance");
  const insurance = await DeltaInsurance.deploy(
    deployments.MOCK_AAVE,
    deployments.MOCK_ADAI,
    deployments.MOCK_ICY,
    { nonce: nonce + i++ }
  );
  deployments.DELTA_INSURANCE = insurance.address;

  const DLT = await MockERC20.deploy("DLT", "DLT", 18, { nonce: nonce + i++ });
  const rMAV = await MockERC20.deploy("MAV", "MAV", 18, { nonce: nonce + i++ });
  const rFRA = await MockERC20.deploy("FRA", "FRA", 18, { nonce: nonce + i++ });
  deployments.DLT = DLT.address;
  deployments.rMAV = rMAV.address;
  deployments.rFRA = rFRA.address;

  const USDC = await MockERC20.deploy("USDC", "USDC", 6, {
    nonce: nonce + i++,
  });
  const USDT = await MockERC20.deploy("USDT", "USDT", 6, {
    nonce: nonce + i++,
  });
  const DAI = await MockERC20.deploy("DAI", "DAI", 18, { nonce: nonce + i++ });
  deployments.USDC = USDC.address;
  deployments.USDT = USDT.address;
  deployments.DAI = DAI.address;

  // Oracles
  const Oracle = await ethers.getContractFactory("MockAggregatorV3");
  const rMAVOracle = await Oracle.deploy(18, { nonce: nonce + i++ });
  const rFRAOracle = await Oracle.deploy(18, { nonce: nonce + i++ });
  deployments.rMAV_ORACLE = rMAVOracle.address;
  deployments.rFRA_ORACLE = rFRAOracle.address;

  // Chainlink
  const ChainLink = await ethers.getContractFactory("ChainLink");
  const chainlink = await ChainLink.deploy({ nonce: nonce + i++ });
  deployments.CHAINLINK = chainlink.address;

  const DeltaOpenFactory = await ethers.getContractFactory("DeltaOpen");
  const deltaOpen = await DeltaOpenFactory.deploy(
    chainlink.address, // oracle
    (0.5 * 10 ** 18).toString(), // 500000000000000000
    { nonce: nonce + i++ }
  );
  deployments.DELTA_OPEN = deltaOpen.address;

  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // const DeltaVerifiedFactory = await ethers.getContractFactory("DeltaVerified")
  // // // // const deltaVerified = await DeltaVerifiedFactory.deploy(
  // // // //     chainlink.address, // oracle
  // // // //     (.5 * 10**18).toString()
  // // // // )
  const deltaVerified = { address: deltaOpen.address };
  deployments.DELTA_VERIFIED = deltaVerified.address;
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD
  // // // // TODO TODOD

  const RewardControl = await ethers.getContractFactory("RewardControl");
  const rewardControl = await RewardControl.deploy(
    deployments.DELTA_OPEN, // deltaVerified
    deployments.DELTA_OPEN, // deltaVerified
    deployments.DLT, // dltAddress
    { nonce: nonce + i++ }
  );
  deployments.REWARD_CONTROL = rewardControl.address;

  const RateModel = await ethers.getContractFactory("RateModel");
  const rateModel = await RateModel.deploy(100, 2000, 100, 3000, 8000, 400, {
    nonce: nonce + i++,
  });
  deployments.RATE_MODEL = rateModel.address;

  fs.writeFileSync(deploymentFilename, JSON.stringify(deployments, null, 4));
}

main().catch((error) => {
  console.error(error);
  process.exitCode = 1;
});
