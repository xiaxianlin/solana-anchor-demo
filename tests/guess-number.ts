import { Program, setProvider, workspace, AnchorProvider, web3, AnchorError } from "@coral-xyz/anchor";
import { expect } from "chai";
import { GuessNumber } from "../target/types/guess_number";

describe("guess_number", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);

  const program = workspace.GuessNumber as Program<GuessNumber>;
  console.log("creator address:", provider.wallet.publicKey.toBase58());

  const guessingPdaPubkey = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("guessing pda"), provider.wallet.publicKey.toBuffer()],
    program.programId
  );

  it("initialize", async () => {
    const tx = await program.methods.initialize().accounts({ payer: provider.wallet.publicKey }).rpc();
    console.log("Initialize successfully!\n Your transaction signature is:", tx);
  });

  it("guess", async () => {
    try {
      const _tx = await program.methods.guess(5).accounts({ payer: provider.wallet.publicKey }).rpc();
      console.log("Congratulation you're right!");
    } catch (err) {
      expect(err).to.be.instanceOf(AnchorError);
      expect((err as AnchorError).error.errorCode.number).to.equal(6000);
    }
  });
});
