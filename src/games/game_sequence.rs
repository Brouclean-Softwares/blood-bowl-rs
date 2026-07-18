use crate::actions::Success;
use crate::errors::Error;
use crate::events::GameEvent;
use crate::games::Game;
use crate::injuries::Injury;
use crate::positions::{Keyword, Position};
use crate::skills::Skill;
use crate::teams::Team;
use crate::translation::TranslatedName;

impl Game {
    pub fn push_injury(
        &mut self,
        team_id: i32,
        player_id: i32,
        injury: Injury,
    ) -> Result<(), Error> {
        self.process_event(GameEvent::Injury {
            team_id,
            player_id,
            injury,
        })
    }

    pub fn suffered_injuries(&self, team_id_for: i32, player_id_for: i32) -> Vec<Injury> {
        let mut injuries: Vec<Injury> = vec![];

        for event in self.events.iter() {
            if let GameEvent::Injury {
                team_id,
                player_id,
                injury,
            } = event
            {
                if team_id_for.eq(team_id) && player_id_for.eq(player_id) {
                    injuries.push(injury.clone());
                }
            }
        }

        injuries
    }

    pub fn suffered_injuries_names(
        &self,
        team_id_for: i32,
        player_id_for: i32,
        lang_id: &str,
    ) -> String {
        self.suffered_injuries(team_id_for, player_id_for)
            .iter()
            .map(|injury| injury.name(lang_id))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn push_hatred(
        &mut self,
        team_id: i32,
        player_id: i32,
        keyword: Keyword,
    ) -> Result<(), Error> {
        self.process_event(GameEvent::Hatred {
            team_id,
            player_id,
            keyword,
        })
    }

    pub fn suffered_hatred(&self, team_id_for: i32, player_id_for: i32) -> Vec<Keyword> {
        let mut keywords: Vec<Keyword> = vec![];

        for event in self.events.iter() {
            if let GameEvent::Hatred {
                team_id,
                player_id,
                keyword,
            } = event
            {
                if team_id_for.eq(team_id) && player_id_for.eq(player_id) {
                    keywords.push(keyword.clone());
                }
            }
        }

        keywords
    }

    pub fn suffered_hatred_names(
        &self,
        team_id_for: i32,
        player_id_for: i32,
        lang_id: &str,
    ) -> String {
        self.suffered_hatred(team_id_for, player_id_for)
            .iter()
            .map(|keyword| keyword.name(lang_id))
            .collect::<Vec<String>>()
            .join(", ")
    }

    pub fn push_sent_off(&mut self, team_id: i32, player_id: i32) -> Result<(), Error> {
        self.process_event(GameEvent::SentOff { team_id, player_id })
    }

    pub fn player_has_been_sent_off(&self, team_id_for: i32, player_id_for: i32) -> bool {
        for event in self.events.iter() {
            if let GameEvent::SentOff { team_id, player_id } = event {
                if team_id_for.eq(team_id) && player_id_for.eq(player_id) {
                    return true;
                }
            }
        }

        false
    }

    pub fn push_player_skill(
        &mut self,
        team_id: i32,
        player_id: i32,
        skill: Skill,
    ) -> Result<(), Error> {
        self.process_event(GameEvent::PlayerSkill {
            team_id,
            player_id,
            skill,
        })
    }

    pub fn push_resurrection(
        &mut self,
        team_id: i32,
        position: Option<Position>,
    ) -> Result<(), Error> {
        let resurrected_player = if self.first_team.id.eq(&team_id) {
            Some(
                self.first_team
                    .add_resurrected_player_with_number(0, position)?,
            )
        } else if self.second_team.id.eq(&team_id) {
            Some(
                self.second_team
                    .add_resurrected_player_with_number(0, position)?,
            )
        } else {
            None
        };

        if let Some(resurrected_player) = resurrected_player {
            self.process_event(GameEvent::Resurrection {
                team_id,
                position: resurrected_player.position,
            })?;
        }

        return Ok(());
    }

    pub fn push_success(
        &mut self,
        team_id: i32,
        player_id: i32,
        success: Success,
    ) -> Result<(), Error> {
        let roster_definition = if self.first_team.id.eq(&team_id) {
            self.first_team.roster_definition()
        } else if self.second_team.id.eq(&team_id) {
            self.second_team.roster_definition()
        } else {
            None
        }
        .ok_or(Error::RosterNotExist)?;

        let star_player_points = success.star_player_points(&roster_definition, &self.version);

        self.process_event(GameEvent::Success {
            team_id,
            player_id,
            success,
            star_player_points,
        })
    }

    pub fn push_penalties(
        &mut self,
        first_team_score: usize,
        second_team_score: usize,
    ) -> Result<(), Error> {
        self.process_event(GameEvent::Penalties {
            first_team_score,
            second_team_score,
        })
    }

    pub fn penalties_score(&self) -> Option<(usize, usize)> {
        let mut penalties_score = None;

        for event in self.events.iter() {
            match event {
                GameEvent::Penalties {
                    first_team_score,
                    second_team_score,
                } => penalties_score = Some((first_team_score.clone(), second_team_score.clone())),

                _ => {}
            }
        }

        penalties_score
    }

    pub fn is_having_winner(&self) -> bool {
        self.is_winning().0 || self.is_winning().1
    }

    pub fn is_winning(&self) -> (bool, bool) {
        if let Some(penalties_score) = self.penalties_score() {
            (
                penalties_score.0 > penalties_score.1,
                penalties_score.0 < penalties_score.1,
            )
        } else {
            let (first_score, second_score) = self.score();
            (first_score > second_score, first_score < second_score)
        }
    }

    pub fn winning_team(&self) -> Option<Team> {
        let (first_team_is_winner, second_team_is_winner) = self.winner();

        if first_team_is_winner {
            Some(self.first_team.clone())
        } else if second_team_is_winner {
            Some(self.second_team.clone())
        } else {
            None
        }
    }
}
