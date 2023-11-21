import { Deployer, Reporter } from "@solarity/hardhat-migrate";
import { Voting__factory, Groth16Verifier__factory } from "@ethers-v6";

export = async (deployer: Deployer) => {
  const verifier = await deployer.deploy(Groth16Verifier__factory);
  const voting = await deployer.deploy(Voting__factory, [6, verifier.address]);

  Reporter.reportContracts(["Verifier", verifier.address], ["Voting", voting.address]);
};
