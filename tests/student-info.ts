import {
    Program,
    setProvider,
    workspace,
    AnchorProvider,
    web3
} from '@coral-xyz/anchor'
import { expect } from 'chai'
import { StudentInfo } from '../target/types/student_info'

describe('student-info', () => {
    const provider = AnchorProvider.env()
    setProvider(provider)

    const program = workspace.StudentInfo as Program<StudentInfo>

    const student = {
        name: 'Tiny',
        intro: 'He is a good boy'
    }

    console.log('creator address:', provider.wallet.publicKey.toBase58())

    const [pda] = web3.PublicKey.findProgramAddressSync(
        [Buffer.from(student.name), provider.wallet.publicKey.toBuffer()],
        program.programId
    )

    it('Student is added', async () => {
        const tx = await program.methods
            .addStudent(student.name, student.intro)
            .rpc()
        console.log('Created signature: ', tx)

        const account = await program.account.student.fetch(pda)
        expect(student.name === account.name)
        expect(student.intro === account.intro)
        expect(account.creator === provider.wallet.publicKey)
    })

    it('Student is updated', async () => {
        const newIntro = 'He is very good at solana'
        const tx = await program.methods
            .updateStudent(student.name, newIntro)
            .rpc()
        console.log('Updated signature: ', tx)

        const account = await program.account.student.fetch(pda)
        expect(student.name === account.name)
        expect(student.intro === newIntro)
        expect(account.creator === provider.wallet.publicKey)
    })

    it('Student is deleted', async () => {
        const tx = await program.methods.deleteStudent(student.name).rpc()
        console.log('Deleted signature: ', tx)
    })
})
