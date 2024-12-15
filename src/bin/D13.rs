
const A_COST: i64 = 3;
const B_COST: i64 = 1;

fn main() {

    let fstr: String = std::fs::read_to_string("./input/D13.txt").expect("Error Reading File!");

    let mechs = Machine::list_from_fstr(fstr);

    part1(&mechs);
    part2(&mechs);
}

#[derive(PartialEq, Eq, Clone, Copy)]
struct Vec2<T> {
    x: T,
    y: T
}

impl std::fmt::Display for Vec2<i64> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl std::ops::Add for Vec2<i64> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y
        }
    }
}

impl Vec2<i64> {
    fn mul(&self, s: i64) -> Vec2<i64> {
        Vec2 { 
            x: self.x*s,
            y: self.y*s
        }
    }
}

struct Machine {
    a_move: Vec2<i64>,
    b_move: Vec2<i64>,
    prize:  Vec2<i64>
}

impl Machine {
    fn list_from_fstr(fstr: String) -> Vec<Self> {

        fn vec_from_button_str(st: &str) -> Vec2<i64> {

            let x_ind = st.chars().position(|c| c=='X').unwrap();
            let y_ind = st.chars().position(|c| c=='Y').unwrap();
            let comma_ind = st.chars().position(|c| c==',').unwrap();

            Vec2 {
                x: st[x_ind+2..comma_ind].parse().unwrap(),
                y: st[y_ind+2..].parse().unwrap()
            }
        }

        let mut lines = fstr.lines().peekable();

        let mut machines: Vec<Machine> = Vec::new();

        while lines.peek().is_some() {

            let new_mech = Machine{ 
                a_move: vec_from_button_str(lines.next().unwrap()),
                b_move: vec_from_button_str(lines.next().unwrap()),
                prize: vec_from_button_str(lines.next().unwrap())
            };
            lines.next();

            machines.push(new_mech);
        }

        machines
    }

    fn det(&self) -> i64 {
        (self.a_move.x * self.b_move.y) - (self.b_move.x * self.a_move.y)
    }

    fn invert_prize(&self) -> Option<Vec2<i64>> {
        let det = self.det();

        if det == 0 {
            return None;
        }

        return Some(Vec2 { 
            x: (self.prize.x*self.b_move.y - self.prize.y*self.b_move.x)/det,
            y: (self.prize.x*-self.a_move.y + self.prize.y*self.a_move.x)/det
        });
    }
}


fn part1(mechs: &Vec<Machine>) {

    let mut cost = 0;

    for mech in mechs {

        let comb = mech.invert_prize();

        match comb {
            Some(combo) => {

                let checked = mech.a_move.mul(combo.x) + mech.b_move.mul(combo.y);

                if checked == mech.prize {
                    cost += A_COST*combo.x + B_COST*combo.y;
                }
            }
            None => println!("Prize: {} not invertable", mech.prize),
        }
    }

    println!("Total cost: {} tokens", cost);

}

fn part2(mechs: &Vec<Machine>) {

    let new_mechs: Vec<Machine> = mechs.iter().map(|m| Machine {
        a_move: m.a_move,
        b_move: m.b_move,
        prize:  m.prize + Vec2 { x: 10000000000000, y: 10000000000000 },
    }).collect();

    println!("Now checking with new prize locs:");
    part1(&new_mechs);
}
