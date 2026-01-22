mod tests {
    use solana_program::hash::{hashv, Hash};
    use solana_sdk::{
        signature::{Keypair, Signer},
        transaction::Transaction,
        system_instruction,
    };
    use anchor_lang::prelude::*;
    use crate::*;

    #[test]
    fn test_raffle() {
        let program_id = Pubkey::new_unique();
        let admin = Keypair::new();
        let participant1 = Keypair::new();
        let participant2 = Keypair::new();

        // Initialize raffle
        let state = RaffleState {
            admin: admin.pubkey(),
            participants: vec![],
            pot: 0,
        };

        let mut test_env = TestEnvironment::start_new(program_id, vec![]);
        test_env.declare_program(admin);

        // Participant 1 joins
        let join_ix = system_instruction::transfer(&participant1.pubkey(), &state.key, 1_000_000);
        test_env.process_transaction(Transaction::new(&[&admin, &participant1]), &[join_ix]);

        // Participant 2 joins
        let join_ix = system_instruction::transfer(&participant2.pubkey(), &state.key, 1_000_000);
        test1_env.process_transaction(Transaction::new(&[&admin, &participant2]), &[join_ix]);

        // Draw winner (randomly selects based on hash)
        let draw_winner = RaffleInstruction::DrawWinner;
        let result = test_env.execute_instruction(draw_winner, state.key, participant1.pubkey());
        assert!(result.is_ok());

        // Check if funds were transferred
        let final_balance = test_env.get_account(&participant1).unwrap().lamports;
        assert!(final_balance > 0);
    }
}
