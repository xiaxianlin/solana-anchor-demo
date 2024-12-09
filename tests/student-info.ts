import { Program, setProvider, workspace, AnchorProvider, web3 } from "@coral-xyz/anchor";
import { expect } from "chai";
import { StudentInfo } from "../target/types/student_info";
import { getAccount, getAssociatedTokenAddress } from "@solana/spl-token";

describe("student-info", () => {
  const provider = AnchorProvider.env();
  setProvider(provider);

  const program = workspace.StudentInfo as Program<StudentInfo>;

  const student = {
    name: "Tiny",
    intro: "He is a good boy",
  };

  console.log("creator address:", provider.wallet.publicKey.toBase58());

  const [pda] = web3.PublicKey.findProgramAddressSync(
    [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
    program.programId
  );
  const [mint] = web3.PublicKey.findProgramAddressSync([Buffer.from("mint")], program.programId);

  console.log("mint address: ", mint.toBase58());

  it("Initializes the reward token", async () => {
    const tx = await program.methods.initializeTokenMint().rpc();
  });

  it("Student is added", async () => {
    const tokenAccount = await getAssociatedTokenAddress(mint, provider.wallet.publicKey);

    // @ts-ignore
    const tx = await program.methods.addStudent(student.name, student.intro).accounts({ tokenAccount }).rpc();
    console.log("Created signature: ", tx);

    const account = await program.account.student.fetch(pda);
    expect(student.name === account.name);
    expect(student.intro === account.intro);
    expect(account.creator === provider.wallet.publicKey);

    const userAta = await getAccount(provider.connection, tokenAccount);
    expect(Number(userAta.amount)).to.equal((10 * 10) ^ 6);
  });

  it("Student is updated", async () => {
    const newIntro = "He is good";
    const tx = await program.methods.updateStudent(student.name, newIntro).rpc();
    console.log("Updated signature: ", tx);

    const account = await program.account.student.fetch(pda);
    expect(student.name === account.name);
    expect(student.intro === newIntro);
    expect(account.creator === provider.wallet.publicKey);
  });

  it("Student is deleted", async () => {
    const tx = await program.methods.deleteStudent(student.name).rpc();
    console.log("Deleted signature: ", tx);
  });
});
