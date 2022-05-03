use uuid::Uuid;
pub struct Participant {
    id: String,
    pub name: String,
    points: i32,
}

impl Participant {
    pub fn new(name: String) -> Participant {
        Participant {
            id: Uuid::new_v4().to_string(),
            name: name,
            points: 0,
        }
    }

    pub fn get_id(&self) -> &String {
        &self.id
    }

    pub fn get_points(&self) -> &i32 {
        &self.points
    }

    pub fn give_points(&mut self, points: i32) {
        self.points += points;
    }
}
