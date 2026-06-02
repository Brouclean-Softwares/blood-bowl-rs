use crate::errors::Error;
use crate::events::GameEvent;
use crate::games::{Game, GameStatus};
use crate::inducements::{Inducement, TreasuryAndPettyCash};
use crate::prayers::PrayerToNuffle;
use crate::teams::Team;
use crate::weather::Weather;

impl Game {
    pub fn generate_fans(&mut self) -> Result<u32, Error> {
        let mut game_fans: u32 = 0;

        let fan_factor = GameEvent::roll_fan_factor(&self.first_team.clone());
        game_fans += fan_factor as u32;
        self.set_team_fan_factor(self.first_team.clone(), fan_factor)?;

        let fan_factor = GameEvent::roll_fan_factor(&self.second_team.clone());
        game_fans += fan_factor as u32;
        self.set_team_fan_factor(self.second_team.clone(), fan_factor)?;

        Ok(game_fans * 10000)
    }

    pub fn set_team_fan_factor(&mut self, team_for: Team, new_fan_factor: u8) -> Result<u8, Error> {
        let position = self.events.iter().position(|event| match event {
            GameEvent::FanFactor { team_id, .. } => team_for.id.eq(team_id),
            _ => false,
        });

        if let Some(position) = position {
            self.events[position] = GameEvent::FanFactor {
                team_id: team_for.id,
                fan_factor: new_fan_factor,
            };
            Ok(new_fan_factor)
        } else {
            self.process_event(GameEvent::FanFactor {
                team_id: team_for.id,
                fan_factor: new_fan_factor,
            })?;
            Ok(new_fan_factor)
        }
    }

    pub fn team_fan_factor(&self, team_for: &Team) -> Option<u8> {
        for event in self.events.iter() {
            if let GameEvent::FanFactor {
                team_id,
                fan_factor,
            } = event
                && team_id.eq(&team_for.id)
            {
                return Some(fan_factor.clone());
            }
        }

        None
    }

    pub fn fans(&self) -> Option<u32> {
        if let (Some(first_team_fan_factor), Some(second_team_fan_factor)) = (
            self.team_fan_factor(&self.first_team),
            self.team_fan_factor(&self.second_team),
        ) {
            Some(10000 * (first_team_fan_factor as u32 + second_team_fan_factor as u32))
        } else {
            None
        }
    }

    pub fn generate_weather(&mut self) -> Result<Weather, Error> {
        let weather = Weather::roll();
        self.push_weather(weather)
    }

    pub fn push_weather(&mut self, new_weather: Weather) -> Result<Weather, Error> {
        self.process_event(GameEvent::Weather(new_weather.clone()))?;
        Ok(new_weather)
    }

    pub fn weather(&self) -> Option<Weather> {
        let mut weather = None;

        for event in self.events.iter() {
            if let GameEvent::Weather(weather_from_event) = event {
                weather = Some(weather_from_event.clone());
            }
        }

        weather
    }

    pub fn generate_journeymen(&mut self) -> Result<(u8, u8), Error> {
        let players = self.first_team.number_of_available_players();
        let first_team_journeymen_number = if players < 11 { 11 - players } else { 0 };

        for _ in 0..first_team_journeymen_number {
            let _ = self.first_team.add_journeyman_with_number(0);

            self.process_event(GameEvent::Journeyman {
                team_id: self.first_team.id,
            })?;
        }

        let players = self.second_team.number_of_available_players();
        let second_team_journeymen_number = if players < 11 { 11 - players } else { 0 };

        for _ in 0..second_team_journeymen_number {
            let _ = self.second_team.add_journeyman_with_number(0);

            self.process_event(GameEvent::Journeyman {
                team_id: self.second_team.id,
            })?;
        }

        Ok((first_team_journeymen_number, second_team_journeymen_number))
    }

    pub fn journeymen_ok(&self) -> bool {
        self.first_team.available_players().len() >= 11
            && self.second_team.available_players().len() >= 11
    }

    pub fn petty_cash(&self) -> Result<(u32, u32), Error> {
        if matches!(self.status(), GameStatus::PreGameSequence) {
            let (first_team_cost_for_added_players, second_team_cost_for_added_players) =
                self.teams_inducements_cost_for_added_players();

            let (first_team_treasury_used, second_team_treasury_used) =
                self.teams_treasury_used_for_inducements();

            let first_team_value =
                self.first_team.current_value()? - first_team_cost_for_added_players as u32;

            let second_team_value =
                self.second_team.current_value()? - second_team_cost_for_added_players as u32;

            let first_team_petty_cash = if first_team_value < second_team_value {
                second_team_value + second_team_treasury_used as u32 - first_team_value
            } else {
                0
            };

            let second_team_petty_cash = if second_team_value < first_team_value {
                first_team_value + first_team_treasury_used as u32 - second_team_value
            } else {
                0
            };

            Ok((first_team_petty_cash, second_team_petty_cash))
        } else {
            Ok((0, 0))
        }
    }

    pub fn teams_money_left(&self) -> Result<(TreasuryAndPettyCash, TreasuryAndPettyCash), Error> {
        if matches!(self.status(), GameStatus::PreGameSequence) {
            let (mut first_team_petty_cash_left, mut second_team_petty_cash_left) =
                self.petty_cash()?;

            for event in self.events.iter() {
                if let GameEvent::BuyInducement {
                    team_id,
                    used_money,
                    ..
                } = event
                {
                    if self.first_team.id.eq(team_id) {
                        if first_team_petty_cash_left > used_money.petty_cash {
                            first_team_petty_cash_left -= used_money.petty_cash;
                        } else {
                            first_team_petty_cash_left = 0;
                        }
                    }

                    if self.second_team.id.eq(team_id) {
                        if second_team_petty_cash_left > used_money.petty_cash {
                            second_team_petty_cash_left -= used_money.petty_cash;
                        } else {
                            second_team_petty_cash_left = 0;
                        }
                    }
                }
            }

            Ok((
                TreasuryAndPettyCash {
                    treasury: self.first_team.treasury,
                    petty_cash: first_team_petty_cash_left,
                },
                TreasuryAndPettyCash {
                    treasury: self.second_team.treasury,
                    petty_cash: second_team_petty_cash_left,
                },
            ))
        } else {
            Ok((
                TreasuryAndPettyCash {
                    treasury: self.first_team.treasury,
                    petty_cash: 0,
                },
                TreasuryAndPettyCash {
                    treasury: self.second_team.treasury,
                    petty_cash: 0,
                },
            ))
        }
    }

    pub fn teams_inducements(&self) -> (Vec<Inducement>, Vec<Inducement>) {
        let mut first_inducements: Vec<Inducement> = vec![];
        let mut second_inducements: Vec<Inducement> = vec![];

        for event in self.events.iter() {
            if let GameEvent::BuyInducement {
                team_id,
                inducement,
                ..
            } = event
            {
                if team_id.eq(&self.first_team.id) {
                    first_inducements.push(inducement.clone());
                }

                if team_id.eq(&self.second_team.id) {
                    second_inducements.push(inducement.clone());
                }
            }
        }

        (first_inducements, second_inducements)
    }

    pub fn teams_treasury_used_for_inducements(&self) -> (i32, i32) {
        let mut first_team_treasury_used: i32 = 0;
        let mut second_team_treasury_used: i32 = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::BuyInducement {
                    team_id,
                    used_money,
                    ..
                } => {
                    if team_id.eq(&self.first_team.id) {
                        first_team_treasury_used += used_money.treasury;
                    }

                    if team_id.eq(&self.second_team.id) {
                        second_team_treasury_used += used_money.treasury;
                    }
                }

                _ => {}
            }
        }

        (first_team_treasury_used, second_team_treasury_used)
    }

    pub fn teams_inducements_cost_for_added_players(&self) -> (i32, i32) {
        let mut first_team_cost: i32 = 0;
        let mut second_team_cost: i32 = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::BuyInducement {
                    team_id,
                    used_money,
                    inducement: Inducement::StarPlayer(_) | Inducement::MegaStarPlayer(_),
                } => {
                    if team_id.eq(&self.first_team.id) {
                        first_team_cost += used_money.total();
                    }

                    if team_id.eq(&self.second_team.id) {
                        second_team_cost += used_money.total();
                    }
                }

                _ => {}
            }
        }

        (first_team_cost, second_team_cost)
    }

    pub fn teams_inducements_cost(&self) -> (i32, i32) {
        let mut first_team_cost: i32 = 0;
        let mut second_team_cost: i32 = 0;

        for event in self.events.iter() {
            match event {
                GameEvent::BuyInducement {
                    team_id,
                    used_money,
                    ..
                } => {
                    if team_id.eq(&self.first_team.id) {
                        first_team_cost += used_money.total();
                    }

                    if team_id.eq(&self.second_team.id) {
                        second_team_cost += used_money.total();
                    }
                }

                _ => {}
            }
        }

        (first_team_cost, second_team_cost)
    }

    pub fn team_inducement_type_number(
        &self,
        team_id_for: i32,
        inducement_to_check: &Inducement,
    ) -> usize {
        self.events
            .iter()
            .filter(|&event| {
                if let GameEvent::BuyInducement {
                    team_id,
                    inducement,
                    ..
                } = event
                {
                    team_id_for.eq(team_id) && inducement.eq(&inducement_to_check)
                } else {
                    false
                }
            })
            .count()
    }

    pub fn inducements_buyable_by_teams(
        &self,
    ) -> Result<(Vec<Inducement>, Vec<Inducement>), Error> {
        let (first_team_money_left, second_team_money_left) = self.teams_money_left()?;

        let mut first_team_inducements_buyable: Vec<Inducement> =
            Inducement::list_buyable_for_team(&self.first_team, &first_team_money_left);

        first_team_inducements_buyable.retain(|inducement| {
            self.team_inducement_type_number(self.first_team.id.clone(), inducement)
                .lt(&inducement.maximum_for_team(&self.first_team))
        });

        let mut second_team_inducements_buyable: Vec<Inducement> =
            Inducement::list_buyable_for_team(&self.second_team, &second_team_money_left);

        second_team_inducements_buyable.retain(|inducement| {
            self.team_inducement_type_number(self.second_team.id.clone(), inducement)
                .lt(&inducement.maximum_for_team(&self.second_team))
        });

        Ok((
            first_team_inducements_buyable,
            second_team_inducements_buyable,
        ))
    }

    pub fn team_buy_inducement(
        &mut self,
        team_id: i32,
        inducement: Inducement,
    ) -> Result<Inducement, Error> {
        if !self.journeymen_ok() {
            return Err(Error::JourneymenShouldBeOkBeforeBuyingInducements);
        }

        let (buyable_by_first_team, buyable_by_second_team) =
            self.inducements_buyable_by_teams()?;
        let (first_team_money_left, second_team_money_left) = self.teams_money_left()?;

        if team_id.eq(&self.first_team.id) && buyable_by_first_team.contains(&inducement) {
            self.process_event(GameEvent::BuyInducement {
                team_id,
                inducement: inducement.clone(),
                used_money: first_team_money_left
                    .money_used_to_buy(inducement.price_for_team(&self.first_team))?,
            })?;

            match inducement {
                Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                    let _ = self.first_team.add_special_players_with_number(0, position);
                }

                Inducement::FamousCoachingStaff(famous_coaching_staff) => {
                    if let Some(position) = famous_coaching_staff.position(&self.version) {
                        self.first_team.add_special_players_with_number(0, position);
                    }
                }

                _ => {}
            };

            return Ok(inducement);
        }

        if team_id.eq(&self.second_team.id) && buyable_by_second_team.contains(&inducement) {
            self.process_event(GameEvent::BuyInducement {
                team_id,
                inducement: inducement.clone(),
                used_money: second_team_money_left
                    .money_used_to_buy(inducement.price_for_team(&self.second_team))?,
            })?;

            match inducement {
                Inducement::StarPlayer(position) | Inducement::MegaStarPlayer(position) => {
                    let _ = self
                        .second_team
                        .add_special_players_with_number(0, position);
                }

                Inducement::FamousCoachingStaff(famous_coaching_staff) => {
                    if let Some(position) = famous_coaching_staff.position(&self.version) {
                        self.second_team
                            .add_special_players_with_number(0, position);
                    }
                }

                _ => {}
            };

            return Ok(inducement);
        }

        Err(Error::NotAPlayingTeam)
    }

    pub fn recalculated_current_team_values(&self) -> Result<(u32, u32), Error> {
        let (first_team_inducement_cost, second_team_inducement_cost) =
            self.teams_inducements_cost();

        let (
            first_team_inducement_cost_for_added_players,
            second_team_inducement_cost_for_added_players,
        ) = self.teams_inducements_cost_for_added_players();

        Ok((
            self.first_team.current_value()? + first_team_inducement_cost as u32
                - first_team_inducement_cost_for_added_players as u32,
            self.second_team.current_value()? + second_team_inducement_cost as u32
                - second_team_inducement_cost_for_added_players as u32,
        ))
    }

    pub fn push_prayer(
        &mut self,
        team_id: i32,
        new_prayer: PrayerToNuffle,
    ) -> Result<PrayerToNuffle, Error> {
        self.process_event(GameEvent::PrayerToNuffle {
            team_id,
            prayer_to_nuffle: new_prayer.clone(),
        })?;
        Ok(new_prayer)
    }

    pub fn teams_prayers(&self) -> (Vec<PrayerToNuffle>, Vec<PrayerToNuffle>) {
        let mut first_prayers: Vec<PrayerToNuffle> = vec![];
        let mut second_prayers: Vec<PrayerToNuffle> = vec![];

        for event in self.events.iter() {
            if let GameEvent::PrayerToNuffle {
                team_id,
                prayer_to_nuffle,
                ..
            } = event
            {
                if team_id.eq(&self.first_team.id) {
                    first_prayers.push(prayer_to_nuffle.clone());
                }

                if team_id.eq(&self.second_team.id) {
                    second_prayers.push(prayer_to_nuffle.clone());
                }
            }
        }

        (first_prayers, second_prayers)
    }

    pub fn generate_toss_winner(&mut self) -> Result<i32, Error> {
        let team_id = GameEvent::roll_toss_winner(self);
        self.push_toss_winner(team_id)
    }

    pub fn push_toss_winner(&mut self, team_id: i32) -> Result<i32, Error> {
        self.process_event(GameEvent::TossWinner {
            team_id: team_id.clone(),
        })?;
        Ok(team_id)
    }

    pub fn toss_winner(&self) -> Option<&Team> {
        for event in self.events.iter() {
            if let GameEvent::TossWinner { team_id } = event {
                return if self.first_team.id.eq(team_id) {
                    Some(&self.first_team)
                } else if self.second_team.id.eq(team_id) {
                    Some(&self.second_team)
                } else {
                    None
                };
            }
        }

        None
    }

    pub fn push_kicking_team(&mut self, team_id: i32) -> Result<i32, Error> {
        self.process_event(GameEvent::KickingTeam { team_id })?;
        Ok(team_id)
    }

    pub fn kicking_team(&self) -> Option<&Team> {
        for event in self.events.iter() {
            if let GameEvent::KickingTeam { team_id } = event {
                return if self.first_team.id.eq(team_id) {
                    Some(&self.first_team)
                } else if self.second_team.id.eq(team_id) {
                    Some(&self.second_team)
                } else {
                    None
                };
            }
        }

        None
    }

    pub fn pre_game_sequence_is_finished(&self) -> bool {
        self.kicking_team().is_some()
    }
}
