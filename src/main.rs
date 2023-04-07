#[derive(Debug, Copy, Clone)]
struct SumFromPos {
    total: u64,
    ending_pos: usize,
}

impl SumFromPos {
    fn new() -> Self {
        SumFromPos {
            total: 0,
            ending_pos: 0,
        }
    }
}

fn main() {
    //#1
    // let number_places = 3; //L
    // let number_times_runs = 3; //C
    // let people = Vec::<i32>::from([3, 1, 1, 2]);

    //#2
    let number_places = 10000; //L
    let number_times_runs = 10; //C
    let people_vec = Vec::<i32>::from([100, 200, 300, 400, 500]);
    let num_groups = people_vec.len();
    let mut people = [0; 1000];

    for (i, val) in people_vec.iter().enumerate() {
        people[i] = *val;
    }

    let mut all_totals = [SumFromPos::new(); 1000];

    for i in 0..num_groups {
        let (number_people, ending_pos) =
            calculate_earnings_for_pos(
                &people,
                num_groups,
                number_places,
                i,
            );

        let mut sum_from_pos = &mut all_totals[i];
        sum_from_pos.total = number_people as u64;
        sum_from_pos.ending_pos = ending_pos;
    }

    let mut earnings = 0 as u64;
    let mut pos = 0 as usize;
    for _ in 0..number_times_runs {
        let sum_from_pos = &all_totals[pos];
        earnings += sum_from_pos.total;
        pos = sum_from_pos.ending_pos;
    }

    println!("{earnings}");
}

fn calculate_earnings_for_pos(
    people: &[i32],
    num_groups: usize,
    number_places: i32,
    pos: usize,
) -> (i32, usize) {
    let mut groups = 0;
    let mut number_people = 0;
    let mut ending_pos = pos;
    loop {
        let current_group = people[ending_pos];

        if current_group + number_people > number_places {
            break;
        } else {
            number_people += current_group;
            ending_pos += 1;
            if ending_pos >= num_groups {
                ending_pos = 0;
            }
            groups += 1;
        }

        if groups == num_groups {
            break;
        }
    }

    (number_people, ending_pos)
}
