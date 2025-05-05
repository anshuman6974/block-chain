#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Symbol, String, Vec, log, symbol_short};

// Data structure to store challenge details
#[contracttype]
#[derive(Clone)]
pub struct Challenge {
    pub id: u64,
    pub creator: Address,
    pub description: String,
    pub reward: u64,
    pub is_completed: bool,
    pub winner: Option<Address>,
}

// Challenge storage key
#[contracttype]
pub enum ChallengeKey {
    Challenge(u64),
    Count,
}

#[contract]
pub struct TokenizedSkillChallenges;

#[contractimpl]
impl TokenizedSkillChallenges {
    // Create a new skill challenge
    pub fn create_challenge(env: Env, creator: Address, description: String, reward: u64) -> u64 {
        creator.require_auth();

        let mut count: u64 = env.storage().instance().get(&ChallengeKey::Count).unwrap_or(0);
        count += 1;

        let challenge = Challenge {
            id: count,
            creator: creator.clone(),
            description,
            reward,
            is_completed: false,
            winner: None,
        };

        env.storage().instance().set(&ChallengeKey::Challenge(count), &challenge);
        env.storage().instance().set(&ChallengeKey::Count, &count);

        log!(&env, "Challenge {} created by {}", count, creator);

        count
    }

    // Mark challenge as completed and set winner
    pub fn complete_challenge(env: Env, creator: Address, challenge_id: u64, winner: Address) -> bool {
        creator.require_auth();

        let mut challenge: Challenge = env
            .storage()
            .instance()
            .get(&ChallengeKey::Challenge(challenge_id))
            .expect("Challenge not found");

        if challenge.creator != creator || challenge.is_completed {
            return false;
        }

        challenge.is_completed = true;
        challenge.winner = Some(winner.clone());

        env.storage().instance().set(&ChallengeKey::Challenge(challenge_id), &challenge);

        log!(&env, "Challenge {} completed. Winner: {}", challenge_id, winner);
        true
    }

    // View challenge details
    pub fn get_challenge(env: Env, challenge_id: u64) -> Challenge {
        env.storage()
            .instance()
            .get(&ChallengeKey::Challenge(challenge_id))
            .expect("Challenge not found")
    }
}
