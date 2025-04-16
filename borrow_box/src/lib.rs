#[derive(Debug, Clone, Eq, PartialEq)]
pub struct GameSession {
    pub id: u32,
    pub p1: (String, u16),
    pub p2: (String, u16),
    pub nb_games: u16,
    pub finished: bool,
}

impl GameSession {
    pub fn new(id: u32, p1_name: String, p2_name: String, nb_games: u16) -> Box<GameSession> {
        Box::new(GameSession {
            id,
            p1: (p1_name, 0),
            p2: (p2_name, 0),
            nb_games,
            finished: false,
        })
    }

    pub fn read_winner(&self) -> (String, u16) {
        if self.p1.1 > self.p2.1 {
            (self.p1.0.clone(), self.p1.1)
        } else if self.p2.1 > self.p1.1 {
            (self.p2.0.clone(), self.p2.1)
        } else {
            (String::from("Same score! tied"), self.p1.1)
        }
    }

    pub fn update_score(&mut self, user_name: String) {
        if self.finished {
            return;
        }

        let winning_score = (self.nb_games / 2) + 1;

        if self.p1.0 == user_name {
            self.p1.1 += 1;
            if self.p1.1 >= winning_score {
                self.finished = true;
            }
        } else if self.p2.0 == user_name {
            self.p2.1 += 1;
            if self.p2.1 >= winning_score {
                self.finished = true;
            }
        }
    }

    pub fn delete(self) -> String {
        format!("game deleted: id -> {}", self.id)
    }
}