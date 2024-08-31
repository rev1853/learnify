use cosmwasm_std::{Binary, Deps, Env, to_binary};
use crate::core::error::{ContractResult};
use crate::core::handler::QueryHandler;
use crate::core::messages::token_burner::query_leaderboard::{LeaderboardItem, LeaderboardParams, LeaderboardResponse};
use crate::state::state::State;

pub struct LeaderboardHandler {}

impl QueryHandler<LeaderboardParams, State<'_>> for LeaderboardHandler {
    fn handle(
        deps: Deps,
        _env: Env,
        state: State<'_>,
        msg: LeaderboardParams
    ) -> ContractResult<Binary> {
        let mut leaderboard = state
            .leaderboard
            .range(deps.storage, None, None, cosmwasm_std::Order::Ascending)
            .filter_map(|item| {
                let (address, amount) = item.unwrap();
                Some(LeaderboardItem { address, amount })
            })
            .collect::<Vec<LeaderboardItem>>();

        leaderboard.sort_by(|a, b| b.amount.cmp(&a.amount));

        let limit = msg.limit.unwrap_or(10) as usize;
        let start_position = msg.start_after
            .and_then(|addr| {
                leaderboard.iter().position(|entry| entry.address == addr).map(|p| p + 1)
            })
            .unwrap_or(0);

        let entries = leaderboard
            .into_iter()
            .skip(start_position)
            .take(limit)
            .collect::<Vec<LeaderboardItem>>();

        let response = to_binary(&LeaderboardResponse {
            leaderboard: entries
        })?;

        Ok(response)
    }
}