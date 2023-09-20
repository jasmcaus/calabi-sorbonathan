import fs from "fs"
import hre from "hardhat"
import { ethers } from "hardhat"
import { getChainId } from "../../../common/blockchain-utils"
import { getDeploymentFilename } from "../common"

async function main() {
    const [sender] = await hre.ethers.getSigners()
    const chainId = await getChainId(hre)

    // let deployments: any = {}
    const deploymentFilename = getDeploymentFilename(chainId, "-delta-deployments.json")
    const deployments = require("../../../scripts/addresses/5001-delta-deployments.json")

    const provider = new ethers.providers.JsonRpcProvider((hre.network.config as any).url)
    const nonce = await provider.getTransactionCount(sender.address) // Correction 
    console.debug("Nonce:", nonce)
    let i = 0

    const rMAV = deployments.rMAV
    const rFRA = deployments.rFRA

    // Oracles
    const rmavOracle = await ethers.getContractAt("MockAggregatorV3", deployments.rMAV_ORACLE)
    const rfraOracle = await ethers.getContractAt("MockAggregatorV3", deployments.rFRA_ORACLE)

    // Chainlink
    const chainlink = await ethers.getContractAt("ChainLink", deployments.CHAINLINK)

    const deltaOpen = await ethers.getContractAt("DeltaOpen", deployments.DELTA_OPEN)

    // TODO TODOD
    // TODO TODOD
    // TODO TODOD
    // TODO TODOD
    // TODO TODOD
    // const deltaVerified = await ethers.getContractAt("DeltaVerified", deployments.DELTA_OPEN)
    // TODO TODOD
    // TODO TODOD
    // TODO TODOD
    // TODO TODOD
    // TODO TODOD

    const rewardControl = await ethers.getContractAt("RewardControl", deployments.REWARD_CONTROL)

    // Set Reward control
    await deltaOpen.connect(sender).setRewardControlAddress(rewardControl.address, { nonce: nonce + i++ })
    console.debug("Set Reward Control")

    await chainlink.connect(sender).addAsset(rMAV, rmavOracle.address, { nonce: nonce + i++ })
    console.debug("Added asset rMAV")

    await chainlink.connect(sender).addAsset(rFRA, rfraOracle.address, { nonce: nonce + i++ })
    console.debug("Added asset rFRA")

    // await new Promise(resolve => setTimeout(resolve, 3000));

    // Support markets
    await deltaOpen.connect(sender)._supportMarket(rMAV, deployments.RATE_MODEL, { nonce: nonce + i++ })
    console.debug("Supported market rMAV")

    await deltaOpen.connect(sender)._supportMarket(rFRA, deployments.RATE_MODEL, { nonce: nonce + i++ })
    console.debug("Supported market rFRA")
}

main().catch((error) => {
    console.error(error)
    process.exitCode = 1
})
