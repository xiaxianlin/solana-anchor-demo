import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { BurryEscrow } from "../target/types/burry_escrow";
import { Big } from "@switchboard-xyz/common";
import {
  AggregatorAccount,
  AnchorWallet,
  SwitchboardProgram,
  SwitchboardTestContext,
  Callback,
  PermissionAccount,
} from "@switchboard-xyz/solana.js";
import { NodeOracle } from "@switchboard-xyz/oracle";
import { assert } from "chai";

export const solUsedSwitchboardFeed = new anchor.web3.PublicKey("GvDMxPzN1sCj7L26YDK2HnMRXEQmQ2aemov8YBtPS7vR");

function delay(ms: number) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

describe("burry-escrow-vrf", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());
  const provider = anchor.AnchorProvider.env();
  const program = anchor.workspace.BurryEscrow as Program<BurryEscrow>;
  const payer = (provider.wallet as AnchorWallet).payer;

  it("Create Burry Escrow Above Price", async () => {
    // fetch switchboard devnet program object
    const switchboardProgram = await SwitchboardProgram.load(
      "devnet",
      new anchor.web3.Connection("https://api.devnet.solana.com"),
      payer
    );
    const aggregatorAccount = new AggregatorAccount(switchboardProgram, solUsedSwitchboardFeed);

    // derive escrow state account
    const [escrowState] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("MICHAEL BURRY"), payer.publicKey.toBuffer()],
      program.programId
    );
    console.log("Escrow Account: ", escrowState.toBase58());

    // fetch latest SOL price
    const solPrice: Big | null = await aggregatorAccount.fetchLatestValue();
    if (solPrice === null) {
      throw new Error("Aggregator holds no value");
    }
    const failUnlockPrice = solPrice.plus(10).toNumber();
    const amountToLockUp = new anchor.BN(100);

    // Send transaction
    try {
      const tx = await program.methods
        .deposit(amountToLockUp, failUnlockPrice)
        .accounts({
          user: payer.publicKey,
          escrowAccount: escrowState,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer])
        .rpc();

      await provider.connection.confirmTransaction(tx, "confirmed");
      console.log("Your transaction signature", tx);

      // Fetch the created account
      const newAccount = await program.account.escrowState.fetch(escrowState);

      const escrowBalance = await provider.connection.getBalance(escrowState, "confirmed");
      console.log("Onchain unlock price:", newAccount.unlockPrice);
      console.log("Amount in escrow:", escrowBalance);

      // Check whether the data onchain is equal to local 'data'
      assert(failUnlockPrice == newAccount.unlockPrice);
      assert(escrowBalance > 0);
    } catch (error) {
      console.log(error);
      assert.fail(error);
    }
  });

  it("Attempt to withdraw while price is below UnlockPrice", async () => {
    let didFail = false;

    // derive escrow address
    const [escrowState] = await anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from("MICHAEL BURRY"), payer.publicKey.toBuffer()],
      program.programId
    );

    // send tx
    try {
      const tx = await program.methods
        .withdraw()
        .accounts({
          user: payer.publicKey,
          escrowAccount: escrowState,
          feedAggregator: solUsedSwitchboardFeed,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .signers([payer])
        .rpc();

      await provider.connection.confirmTransaction(tx, "confirmed");
      console.log("Your transaction signature", tx);
    } catch (error) {
      didFail = true;

      assert(
        error.message.includes("Current SOL price is not above Escrow unlock price."),
        "Unexpected error message: " + error.message
      );
    }

    assert(didFail);
  });
});
