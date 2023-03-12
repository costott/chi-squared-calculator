use crossterm::{terminal, ExecutableCommand};
use crossterm::event::{self, Event, KeyEvent, KeyCode};
use costottorama::{text, back, style};
use std::f64::consts::E;
use std::io;

fn factorial(n: u32) -> u32 {
    let mut result = 1;
    for i in 2..n+1 {
        result *= i;
    }
    result
}

fn choose(n: u32, r: u32) -> u32 {
    assert!(n >= r);
    factorial(n) / (factorial(r)*factorial(n-r))
}

pub enum Mode {
    OE,
    Binomial,
    Poisson,
    ContingencyTable
}

impl Mode {
    pub fn get_mode() -> Mode {
        _get_mode()
    }
}

fn _get_mode() -> Mode {
    display_title("Chi-Squared Calculator");
    println!(" [1] O vs E\n [2] Binomial\n [3] Poisson\n [4] Contingency Table\n\n");

    loop {
        io::stdout().execute(crossterm::cursor::MoveUp(1)).unwrap();
        io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();

        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).unwrap();

        user_input.pop();
        user_input.pop();

        let mode = match user_input.as_str() {
            "1" => Some(Mode::OE),
            "2" => Some(Mode::Binomial),
            "3" => Some(Mode::Poisson),
            "4" => Some(Mode::ContingencyTable),
            _ => None
        };

        if let Some(m) = mode {
            return m;
        } 
    }
}

/// clears screen and prints given title
fn display_title(text: &str) {
    io::stdout().execute(crossterm::cursor::MoveTo(0,0)).unwrap();
    io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
   
    println!(
        "{}{}{} {text} {}\n", 
        style::BOLD, 
        back::WHITE,
        text::BLACK,
        style::RESET_ALL
    );
}

/// continually asks the user for input until they enter a valid integer
fn int_input(prompt: &str) -> u32 {
    println!("{prompt}\n");
    loop {
        io::stdout().execute(crossterm::cursor::MoveUp(1)).unwrap();
        io::stdout().execute(terminal::Clear(terminal::ClearType::CurrentLine)).unwrap();

        let mut user_input: String = String::new();
        io::stdin().read_line(&mut user_input).unwrap();
        match user_input.trim().parse() {
            Ok(num) => {return num},
            Err(_) => continue
        }
    }
}

/// int input which clears after the input is given
fn clearing_int_input(prompt: &str) -> u32 {
    let output = int_input(prompt);
    io::stdout().execute(crossterm::cursor::MoveUp(2)).unwrap();
    io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

    output
}

/// pause current thread and return which key the
/// user presses when they dos
fn get_key_pressed() -> KeyCode {
    match event::read().unwrap() {
        Event::Key(KeyEvent {
            code: c,
            ..
        }) => c,
        _ => KeyCode::Null
    }
}

fn print_table<T: std::fmt::Display>(
    table: &Vec<Vec<T>>, 
    column_labels: &Vec<String>, 
    row_labels: &Vec<String>, 
    pos: (usize, usize),
) {
    if row_labels.len() != table.len() + 1 {
        panic!("row lengths do not match");
    }
    if column_labels.len() != table[0].len() {
        panic!("column lengths do not match");
    }

    let mut row_label_len = 0;
    for row_label in row_labels.iter() {
        if row_label.len() > row_label_len {
            row_label_len = row_label.chars().count();
        }
    }

    let mut col_len = 0;
    for i in 0..table.len() {
        for j in table[i].iter() {
            let length = j.to_string().chars().count();
            if length > col_len {
                col_len = length;
            }
        }
    }
    for column_label in column_labels.iter() {
        if column_label.chars().count() > col_len {
            col_len = column_label.chars().count();
        }
    }

    let pos_style = &format!("{}{}{}",
        back::WHITE, 
        text::BLACK, 
        style::BOLD,
    );
    let reset_pos_style = &format!("{}{}{}", back::RESET, text::RESET, style::RESET_ALL);
    
    for row in 0..table.len() + 1 {
        let at_pos = pos.0 == row && pos.1 == 0;
        print!(
            "{}{}{}{}{} │ {}", 
            if row == 0 {style::UNDERLINED} else {style::RESET_UNDERLINED},
            if at_pos {pos_style} else {""},
            row_labels[row], 
            " ".repeat(row_label_len - row_labels[row].chars().count()),
            if at_pos {reset_pos_style} else {""},
            style::RESET_ALL
        );
        if row == 0 {
            for (i, column) in column_labels.iter().enumerate() {
                let at_pos = pos.0 == 0 && pos.1  == i + 1;
                print!(
                    "{}{}{column}{}{} {}", 
                    style::UNDERLINED,
                    if at_pos {pos_style} else {""},
                    " ".repeat(col_len - column.chars().count()),
                    if at_pos {reset_pos_style} else {""},
                    style::RESET_ALL
                );
            }
        }
        else {
            for (i, column) in table[row-1].iter().enumerate() {
                let at_pos = pos.0 == row && pos.1 == i + 1;
                print!("{}{column}{}{} ", 
                    if at_pos {pos_style} else {back::LIGHT_BLACK},
                    " ".repeat(col_len - column.to_string().chars().count()),
                    if at_pos {reset_pos_style} else {back::RESET}
                );
            }
        }
        println!("");
    }
}

fn delete_item_in_table(
    table: &mut Vec<Vec<String>>, 
    column_labels: &mut Vec<String>,
    row_labels: &mut Vec<String>, 
    current_pos: (usize, usize)
) {
    match current_pos {
        (r, 0) => {
            if row_labels[r].len() > 0 {
                row_labels[r] = row_labels[r][0..row_labels[r].len()-1].to_string();
            }   
        },
        (0, c) => {
            if column_labels[c-1].len() > 0 {
                column_labels[c-1] = column_labels[c-1][0..column_labels[c-1].len()-1].to_string();
            }
        },
        (r, c) => {
            let item_string = &table[r-1][c-1];
            if item_string.len() > 0 {
                table[r-1][c-1] = (&item_string[0..item_string.len()-1]).to_string();
            }
        }
    };
}

fn add_to_table(
    table: &mut Vec<Vec<String>>, 
    column_labels: &mut Vec<String>,
    row_labels: &mut Vec<String>, 
    current_pos: (usize, usize),
    ch: char,
) {
    match current_pos {
        (r, 0) => {
            let mut label = row_labels[r].to_string();
            label.push(ch);
            row_labels[r] = label;
        },
        (0, c) => {
            let mut label = column_labels[c-1].to_string();
            label.push(ch);
            column_labels[c-1] = label;
        },
        (r, c) => {
            let mut item = table[r-1][c-1].to_string();
            item.push(ch);
            table[r-1][c-1] = item;
        }
    }
}

fn edit_table(table: &mut Vec<Vec<String>>, column_labels: &mut Vec<String>, row_labels: &mut Vec<String>) {
    let mut current_pos = (1, 1);

    io::stdout().execute(crossterm::cursor::Hide).unwrap();

    loop {
        print_table(&table, &column_labels, &row_labels, current_pos);
        println!("\npress {}[esc]{} to finish",
            text::LIGHT_BLUE,
            text::RESET,
        );

        match get_key_pressed() {
            KeyCode::Up => {
                if current_pos.0 > 0 {current_pos.0 -= 1}
            },
            KeyCode::Down => {
                if current_pos.0 < table.len() {current_pos.0 += 1}
            },
            KeyCode::Left => {
                if current_pos.1 > 0 {current_pos.1 -= 1}
            },
            KeyCode::Right => {
                if current_pos.1 < table[0].len() {current_pos.1 += 1}
            },
            KeyCode::Backspace  => delete_item_in_table(table, column_labels, row_labels, current_pos),
            KeyCode::Char(ch) => add_to_table(table, column_labels, row_labels, current_pos, ch),
            KeyCode::Esc => {
                io::stdout().execute(crossterm::cursor::Show).unwrap();
                io::stdout().execute(crossterm::cursor::MoveUp(table.len() as u16 + 3)).unwrap();
                io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
                return;
            },
            _ => {}
        }
        
        io::stdout().execute(crossterm::cursor::MoveUp(table.len() as u16 + 3)).unwrap();
        io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();
    }
}

pub fn observed_expected() {
    display_title("O vs E");

    let columns = int_input("enter the number of columns:") as usize;
    let mut table = vec![vec![String::from(""); columns]; 2];
    let mut column_labels = vec![String::from("---"); columns];
    let mut row_labels = vec![String::from("type"); 3];
    row_labels[1] = String::from("Observed");
    row_labels[2] = String::from("Expected");

    // get rid of the enter columns text
    io::stdout().execute(crossterm::cursor::MoveUp(2)).unwrap();
    io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

    let mut float_table = vec![vec![0.; columns]; 2];
    loop {
        edit_table(&mut table, &mut column_labels, &mut row_labels);

        let mut table_valid = true;
        for i in 0..table.len() {
            for j in 0..table[i].len() {
                match table[i][j].parse() {
                    Ok(float) => float_table[i][j] = float,
                    Err(_) => {table_valid = false}
                }
            }
        }

        if table_valid {break}
    }

    print_table(&float_table, &column_labels, &row_labels, (table.len()+1,table[0].len()+1));

    // let mut gof: f64 = 0.;
    // for i in 0..float_table[0].len() {
    //     gof += (((float_table[0][i] - float_table[1][i]) as f64).powi(2)) / (float_table[1][i]);
    // }
    let gof = float_table[0].iter().zip(float_table[1].iter()).map(|(o, e )| (*o as f64 - e).powi(2) / e).sum::<f64>();

    println!("\nX² = {}{}{}", text::GREEN, gof, text::RESET);
}

fn edit_binomial_distribution() -> (String, String) {
    let mut pos = 0;
    let mut n = String::from("");
    let mut p = String::from("");

    let pos_style = &format!("{}{}{}",
        back::WHITE, 
        text::BLACK, 
        style::BOLD,
    );
    let reset_pos_style = &format!("{}{}{}", back::RESET, text::RESET, style::RESET_ALL);

    io::stdout().execute(crossterm::cursor::Hide).unwrap();
    loop {
        println!(
            "X ~ B(n: {} {} {}, p: {} {} {}) {}(leave p blank for estimation){}",
            if pos == 0 {pos_style} else {back::LIGHT_BLACK},
            if n.len() > 0 {&n} else {" "},
            reset_pos_style,
            if pos == 1 {pos_style} else {back::LIGHT_BLACK},
            if p.len() > 0 {&p} else {" "},
            reset_pos_style,
            text::MAGENTA,
            text::RESET
        );
        println!("\npress {}[esc]{} to finish",
            text::LIGHT_BLUE,
            text::RESET,
        );

        let key_pressed = get_key_pressed();

        if pos == 0 {
            if key_pressed == KeyCode::Right {pos = 1}
            else if key_pressed == KeyCode::Backspace {
                if n.chars().count() > 0 {n.pop();}
            }
            else if let KeyCode::Char(ch) = key_pressed {
                n.push(ch);
            }
        } else {
            if key_pressed == KeyCode::Left {pos = 0}
            else if key_pressed == KeyCode::Backspace {
                if p.chars().count() > 0 {p.pop();}
            }
            else if let KeyCode::Char(ch) = key_pressed {
                p.push(ch);
            }
        }

        io::stdout().execute(crossterm::cursor::MoveUp(3)).unwrap();
        io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        if key_pressed == KeyCode::Esc {break}
    }

    (n, p)
}

enum BinomialP {
    P(f64),
    Estimate
}

fn create_binomial_distribution() -> (usize, BinomialP) {
    loop {
        let (s_n, s_p) = edit_binomial_distribution();
        #[allow(unused_assignments)]
        let (mut n, mut p) = (0, BinomialP::Estimate); // wont be used, just so the compiler's happy

        if let Ok(num) = s_n.parse() {
            n = num;
        } else {continue}

        if let Ok(prob) = s_p.parse() {
            if 0. <= prob && prob <= 1. { 
                p = BinomialP::P(prob);
            } else {continue}
        } else if s_p.len() == 0 {
            p = BinomialP::Estimate;
        } else {continue}

        return (n, p);
    }
}

fn group_expecteds(expecteds: Vec<f64>) -> (Vec<f64>, usize, usize) {
    // last index of start expecteds that need to be grouped
    let mut group_start = 0; 
    while expecteds[group_start] < 5. {
        group_start += 1;
    }
    // see if an additional grouping is required (don't sum to >5)
    let mut start_expected_sum: f64 = 0.;
    for i in 0..group_start+1 {
        start_expected_sum += expecteds[i];
    }
    if start_expected_sum < 5. {
        group_start += 1;
    }

    // first index of end expecteds that need to be grouped
    let mut group_end = expecteds.len()-1;
    while expecteds[group_end] < 5. && group_end > 0 {
        group_end -= 1;
    }
    // see if an additional grouping is required (don't sum to >5)
    let mut end_expected_sum: f64 = 0.;
    for i in group_end+1..expecteds.len() {
        end_expected_sum += expecteds[i];
    }
    if end_expected_sum < 5. {
        group_end -= 1;
    }

    let mut grouped_expecteds = Vec::with_capacity(group_end-group_start+1);
    let mut start_expected_sum: f64 = 0.;
    for i in 0..group_start+1 {
        start_expected_sum += expecteds[i];
    }
    grouped_expecteds.push(start_expected_sum);
    for i in group_start+1..group_end+1 {
        grouped_expecteds.push(expecteds[i]);
    }
    let mut end_expected_sum: f64 = 0.;
    for i in group_end+1..expecteds.len() {
        end_expected_sum += expecteds[i];
    }
    grouped_expecteds.push(end_expected_sum);

    (grouped_expecteds, group_start, group_end)
}

fn group_observed(observed: &Vec<i32>, group_start: usize, group_end: usize) -> Vec<i32> {
    let mut grouped_observed = Vec::with_capacity(group_end-group_start+1);
    let mut start_group = 0;
    for i in 0..group_start+1 {
        start_group += observed[i];
    }
    grouped_observed.push(start_group);
    for i in group_start+1..group_end+1 {
        grouped_observed.push(observed[i]);
    }
    let mut end_group = 0;
    for i in group_end+1..observed.len() {
        end_group += observed[i];
    }
    grouped_observed.push(end_group);

    grouped_observed
}

fn create_binomial_expecteds(n: usize, p: f64, freq_sum: f64) -> (Vec<f64>, usize, usize) {
    let mut expecteds = Vec::with_capacity(n+1);
    for i in 0..n+1 {
        expecteds.push(
            choose(n as u32, i as u32) as f64*
            (p.powi(i as i32))*
            (1.-p).powi((n-i) as i32) 
            * freq_sum
        );
    }  

    group_expecteds(expecteds)
}

pub fn binomial () {
    display_title("Binomial");

    let (n, p) = create_binomial_distribution();

    let mut observed_table = vec![vec![String::from(""); n+1]];
    let mut column_labels = Vec::with_capacity(n+1);
    for i in 0..n+1 {
        column_labels.push(i.to_string());
    }
    let mut row_labels = vec![String::from("type"), String::from("Observed")];

    let mut int_observed_table = vec![0; n+1];
    loop {
        edit_table(&mut observed_table, &mut column_labels, &mut row_labels);
        
        let mut table_valid = true;
        for i in 0..observed_table[0].len() {
            match observed_table[0][i].parse::<i32>() {
                Ok(int) => {int_observed_table[i] = int},
                Err(_) => {table_valid = false}
            }
        }
        for col_label in column_labels.iter() {
            match col_label.parse::<i32>() {
                Ok(_) => {},
                Err(_) => {table_valid = false}
            }
        }

        if table_valid {break}
    }

    let mut freq_sum: i32 = 0;
    for observed in int_observed_table.iter() {
        freq_sum += *observed;
    }
    let freq_sum: f64 = freq_sum as f64;

    let p = match p {
        BinomialP::P(p) => p,
        BinomialP::Estimate => {
            let mut temp_p = 0.;
            for i in 0..int_observed_table.len() {
                temp_p += (column_labels[i].parse::<i32>().unwrap() * int_observed_table[i]) as f64;
            }
            temp_p / (freq_sum * n as f64)
        }
    };

    let (expecteds, group_start, group_end) = create_binomial_expecteds(n, p, freq_sum);

    let grouped_observed = group_observed(&int_observed_table, group_start, group_end);

    // let mut gof = 0.;
    // for i in 0..(&expecteds).len() {
    //     gof += (grouped_observed[i] as f64 - expecteds[i]).powi(2) / expecteds[i];
    // }
    let gof = grouped_observed.iter().zip(expecteds.iter()).map(|(o, e )| (*o as f64 - e).powi(2) / e).sum::<f64>();

    let mut display_table = Vec::with_capacity(2);
    display_table.push(grouped_observed.iter().map(|o| *o as f64).collect());
    display_table.push(expecteds);
    let mut column_labels = vec![String::from(""); grouped_observed.len()];
    if group_start > 0 {
        column_labels[0] = String::from("<= ");
    }
    if group_end < int_observed_table.len()-1 {
        column_labels[grouped_observed.len()-1] = String::from(">= ")
    }
    for i in 0..grouped_observed.len() {
        column_labels[i] += (group_start + i).to_string().as_str();
    }
    let row_labels = vec![format!("X ~ B({n}, {p})"), String::from("Observed"), String::from("Expected")];

    print_table(&display_table, &column_labels, &row_labels, (display_table.len()+1,display_table[0].len()+1));
    println!("\nX² = {}{}{}", text::GREEN, gof, text::RESET);
}

enum PoissonMean {
    Mean(f64),
    Estimate
}

fn create_poission_distribution() -> PoissonMean {
    let mut mean: String = String::from("");

    io::stdout().execute(crossterm::cursor::Hide).unwrap();
    loop {
        println!("X ~ Po(λ: {}{}{}{}) {}(leave λ blank for esitmation){}",
        back::WHITE,
        text::BLACK,
        if mean.len() > 0 {&mean} else {" "},
        style::RESET_ALL,
        text::MAGENTA,
        style::RESET_ALL
        );
        println!("\npress {}[esc]{} to finish",
            text::LIGHT_BLUE,
            text::RESET,
        );

        let key_pressed = get_key_pressed();
        if let KeyCode::Char(ch) = key_pressed {
            mean.push(ch);
        }
        else if key_pressed == KeyCode::Backspace && mean.len() > 0 {
            mean.pop();
        }

        io::stdout().execute(crossterm::cursor::MoveUp(3)).unwrap();
        io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

        if key_pressed == KeyCode::Esc {
            match mean.parse::<f64>() {
                Ok(m) => if m >= 0. {return PoissonMean::Mean(m)} else {continue},
                Err(_) => if mean.len() == 0 {return PoissonMean::Estimate} else {continue}
            }
        }
    }
}

fn create_poisson_expecteds(mean: f64, freq_sum: f64, columns: usize) -> (Vec<f64>, usize, usize) {
    let mut expecteds = Vec::with_capacity(columns);
    for i in 0..columns-1 { // -1 as last is >=
        expecteds.push( freq_sum * ((E.powf(-1.*mean)*mean.powi(i as i32))/ factorial(i as u32) as f64) )
    }
    expecteds.push(freq_sum - expecteds.iter().sum::<f64>());

    group_expecteds(expecteds)
}

pub fn poission() {
    display_title("Possion");

    let mean = create_poission_distribution();

    io::stdout().execute(crossterm::cursor::Show).unwrap();
    let columns = int_input("enter the number of columns:") as usize;
    io::stdout().execute(crossterm::cursor::MoveUp(2)).unwrap();
    io::stdout().execute(terminal::Clear(terminal::ClearType::FromCursorDown)).unwrap();

    let mut observed_table = vec![vec![String::from(""); columns]];
    let mut column_labels = Vec::with_capacity(columns);
    for i in 0..columns {
        column_labels.push(i.to_string());
    }
    let mut row_labels = vec![String::from("type"), String::from("Observed")];

    let mut int_observed_table = vec![0; columns];
    loop {
        edit_table(&mut observed_table, &mut column_labels, &mut row_labels);

        let mut table_valid = true;
        for i in 0..observed_table[0].len() {
            match observed_table[0][i].parse::<i32>() {
                Ok(int) => {int_observed_table[i] = int},
                Err(_) => {table_valid = false}
            }
        }
        for col_label in column_labels.iter() {
            match col_label.parse::<i32>() {
                Ok(_) => {},
                Err(_) => {table_valid = false}
            }
        }

        if table_valid {break}
    }

    let mut freq_sum: i32 = 0;
    for observed in int_observed_table.iter() {
        freq_sum += *observed;
    }
    let freq_sum: f64 = freq_sum as f64;

    let mean = match mean {
        PoissonMean::Mean(m) => m,
        PoissonMean::Estimate => {
            let mut sum_r_f = 0.;
            for i in 0..int_observed_table.len() {
                sum_r_f += (column_labels[i].parse::<i32>().unwrap() * int_observed_table[i]) as f64;
            }
            sum_r_f / freq_sum
        }
    };

    let (expecteds, group_start, group_end) = create_poisson_expecteds(mean, freq_sum, columns);

    let grouped_observed = group_observed(&int_observed_table, group_start, group_end);

    // let mut gof = 0.;
    // for i in 0..(&expecteds).len() {
    //     gof += (grouped_observed[i] as f64 - expecteds[i]).powi(2) / expecteds[i];
    // }
    let gof = grouped_observed.iter().zip(expecteds.iter()).map(|(o, e )| (*o as f64 - e).powi(2) / e).sum::<f64>();

    let mut display_table = Vec::with_capacity(2);
    display_table.push(grouped_observed.iter().map(|o| *o as f64).collect());
    display_table.push(expecteds);
    let mut column_labels = vec![String::from(""); grouped_observed.len()];
    if group_start > 0 {
        column_labels[0] = String::from("<= ");
    }
    column_labels[grouped_observed.len()-1] = String::from(">= ");
    for i in 0..grouped_observed.len() {
        column_labels[i] += (group_start + i).to_string().as_str();
    }
    let row_labels = vec![format!("X ~ Po({mean})"), String::from("Observed"), String::from("Expected")];

    print_table(&display_table, &column_labels, &row_labels, (display_table.len()+1,display_table[0].len()+1));
    println!("\nX² = {}{}{}", text::GREEN, gof, text::RESET);
}

pub fn contingency_table() {
    display_title("Contingency Table");

    let rows = clearing_int_input("enter the number of rows:") as usize;
    let columns = clearing_int_input("enter the number of columns:") as usize;

    let mut table = vec![vec![String::from(""); columns]; rows];
    let mut column_labels = vec![String::from("---"); columns];
    let mut row_labels = vec![String::from("---"); rows+1];

    let mut int_observed_table = vec![vec![0u32; columns]; rows];
    loop {
        edit_table(&mut table, &mut column_labels, &mut row_labels);

        let mut table_valid = true;
        for i in 0..table.len() {
            for j in 0..table[i].len() {
                match table[i][j].parse::<u32>() {
                    Ok(int) => int_observed_table[i][j] = int,
                    Err(_) => {table_valid = false}
                }
            }
        }
        if table_valid {break}
    }
    
    // make totals
    let mut column_totals = vec![0; columns];
    let mut row_totals = vec![0; rows];
    for i in 0..int_observed_table.len() {
        row_totals[i] = int_observed_table[i].iter().sum();
        for j in 0..int_observed_table[i].len() {
            column_totals[j] += int_observed_table[i][j];
        }
    }
    let grand_total = row_totals.iter().sum::<u32>();

    let mut gof: f64 = 0.;
    let mut display_table = vec![vec![String::from(""); columns]; rows];
    for i in 0..rows {
        for j in 0..columns {
            let o = int_observed_table[i][j] as f64;
            let e = (row_totals[i]*column_totals[j]) as f64 / grand_total as f64;
            gof += (o - e).powi(2) / e;

            display_table[i][j] = format!("{o}, {e:.2}");
        }
    }

    print_table(&display_table, &column_labels, &row_labels, (rows+1, columns+1));
    println!("\nX² = {}{}{}", text::GREEN, gof, text::RESET);
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn factorial_works() {
        assert_eq!(factorial(5), 120);
    }
}