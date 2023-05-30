use guessing_game_lib::libutil::print_flush;
use std::{cmp::min, collections::BTreeMap, io};

pub fn exercise_3() {
    // 1. Given a list of integers, use a vector and return the median
    // (when sorted, the value in the middle position) and mode
    // (the value that occurs most often; a hash map will be helpful here) of the list.
    let integer_list = vec![8, 9, 5, 1, 2, 3, 4, 6, 7, 8, 9, 1]; // [1,1,2,3,4,5,6,7,8,8,9,9]
    let median = median(&integer_list);
    println!(
        "Median: {}",
        match median {
            Some(median) => median.to_string(),
            None => String::from("None"),
        }
    );
    let mode = mode(&integer_list);
    println!(
        "Mode: {}",
        match mode {
            Some(mode) => mode.to_string(),
            None => String::from("None"),
        }
    );

    let sentence = String::from(
        r#"
        2. Convert strings to pig latin.
        The first consonant of each word is moved to the end of the word and "ay" is added,
        so "first" becomes "irst-fay"
        Words that start with a vowel have "hay" added to the end instead ("apple" becomes "apple-hay")
        Keep in mind the details about UTF-8 encoding!
        السلام عليكم Dobrý den Hello שָׁלוֹם नमस्ते こんにちは 안녕하세요 你好 Olá Здравствуйте Hola
        @"🌘 アルバイト Aiken 🌒"
        "#,
    );
    let pig_latin = to_pig_latin(&sentence);
    println!("{pig_latin}");
    /*
        2. Onvert-cay trings-say o-tay ig-pay atin-lay.
        He-tay irst-fay onsonant-cay of-hay each-hay ord-way is-hay oved-may o-tay he-tay end-hay of-hay he-tay ord-way and-hay "ay-hay" is-hay added-hay,
        o-say "irst-fay" ecomes-bay "irst-hay-ay-fay"
        Ords-way hat-tay tart-say ith-way a-hay owel-vay ave-hay "ay-hay" added-hay o-tay he-tay end-hay instead-hay ("apple-hay" ecomes-bay "apple-hay-ay-hay")
        Eep-kay in-hay ind-may he-tay etails-day about-hay UTF-hay-8 encoding-hay!
        السلام عليكم Obrý-day en-day Ello-hay שָׁלוֹם नमस्ते こんにちは 안녕하세요 你好 Olá-hay Здравствуйте Ola-hay
        @"🌘 アルバイト Aiken-hay 🌒"
    */

    // 3. Using a hash map and vectors, create a text interface to allow a user to add employee names to a department in a company.
    // For example, "Add Sally to Engineering" or "Add Amir to Sales"
    // Then let the user retrieve a list of all people in a department or all people in the company by department, sorted alphabetically.
    company_employees();
    /* 0
    COMPANY EMPLOYEES
    1. View data
    2. Add data
    3. Edit data
    4. Delete data
    0. Quit
    Select [1-4]:
     */

    /* 1
    VIEW DATA
    1. View all employees
    2. View all employees in a department
    0. Back
    Select [1-2]:
     */
    /* 1.2
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */

    /* 2
    ADD DATA
    1. Add employee
    2. Add department
    0. Back
    Select [1-2]:
     */
    /* 2.1
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 2.1..
    {Department_Name}
    Employee name:
    Confirm employee name [Y/n]:
     */
    /* 2.2
    ADD DEPARTMENT
    Department name:
    Confirm department name [Y/n]:
     */

    /* 3
    EDIT DATA
    1. Edit employee
    2. Edit department
    0. Back
    Select [1-2]:
     */
    /* 3.1
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 3.1..
    SELECT EMPLOYEE
    1. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 3.1....
    {Employee_Name}
    Department name: {Department_Name}
    Change employee name to:
    Confirm edit employee name [Y/n]:
     */
    /* 3.2
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 3.2..
    {Department_Name}
    Change department name to:
    Confirm edit department name [Y/n]:
     */

    /* 4
    DELETE DATA
    1. Delete employee
    2. Delete department
    0. Back
    Select [1-2]:
     */
    /* 4.1
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 4.1..
    SELECT EMPLOYEE
    1. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 4.1....
    {Employee_Name}
    Department name: {Department_Name}
    Confirm delete employee [Y/n]:
     */
    /* 4.2
    SELECT DEPARTMENT
    1. Engineering
    2. Sales
    3. ..
    .. ..
    0. Back
    Select [1-..]:
     */
    /* 4.2..
    {Department_Name}
    Confirm delete department [Y/n]:
     */
}

fn median<T: Copy + Ord>(list: &Vec<T>) -> Option<T> {
    match list.len() {
        0 => None,
        1 => list.first().copied(),
        _ => {
            let mut list = list.clone();
            list.sort();

            return list.get((list.len() - 1) / 2).copied();
        }
    }
}

fn mode<T: Copy + Ord>(list: &Vec<T>) -> Option<T> {
    match list.len() {
        0 => None,
        1 => list.first().copied(),
        _ => {
            let mut map = BTreeMap::new();
            let mut max = 0;
            for &t in list {
                let count = map.entry(t).or_insert(0);
                *count += 1;

                if *count > max {
                    max = *count;
                }
            }

            return median(
                &map.iter()
                    .filter_map(|(&t, &count)| if count == max { Some(t) } else { None })
                    .collect(),
            );
        }
    }
}

fn to_pig_latin(sentence: &String) -> String {
    let mut pig_latin = String::new();

    let mut word = String::new();
    let mut non_word = String::new();
    for character in sentence.chars() {
        if character.is_alphabetic() {
            if non_word.len() != 0 {
                pig_latin = format!("{pig_latin}{non_word}");
                non_word.clear();
            }
            word.push(character);
        } else {
            if word.len() != 0 {
                process_word(&mut word);
                pig_latin = format!("{pig_latin}{word}");
                word.clear();
            }
            non_word.push(character);
        }
    }

    return match (word.len(), non_word.len()) {
        (0, _) => format!("{pig_latin}{non_word}"),
        (_, 0) => format!("{pig_latin}{}", process_word(&mut word)),
        _ => pig_latin,
    };
}

fn process_word(word: &mut String) -> String {
    if starts_with_consonant(word) {
        let word_length = word.len();
        if let Some(letter) = word.chars().next() {
            let a = min(1, word_length);
            *word = if letter.is_uppercase() {
                let b = min(2, word_length);
                format!(
                    "{}{}-{}ay",
                    &word[a..b].to_uppercase(),
                    &word[b..],
                    letter.to_lowercase(),
                )
            } else {
                format!("{}-{letter}ay", &word[a..])
            }
            .trim_start_matches('-')
            .to_string();
        }
    } else if starts_with_vowel(word) {
        *word = format!("{word}-hay");
    }

    return word.to_string();
}

fn starts_with_vowel(word: &String) -> bool {
    starts_with_any(&String::from("aeiouAEIOU"), word)
}

fn starts_with_consonant(word: &String) -> bool {
    let mut consonants = String::new();
    for letter in ('b'..='z').filter(|&letter| !"eiou".contains(letter)) {
        consonants = format!("{}{}{}", letter, letter.to_uppercase(), consonants);
    }

    return starts_with_any(&consonants, word);
}

fn starts_with_any(any: &String, word: &String) -> bool {
    match word.chars().next() {
        Some(letter) => any.contains(letter),
        None => false,
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
enum State {
    Menu0,
    Menu1,
    Menu11,
    Menu12,
    Menu2,
    Menu21,
    Menu22,
    Menu3,
    Menu31,
    Menu32,
    Menu4,
    Menu41,
    Menu42,
}

enum Previous {
    Quit,
    Back,
}

struct Screen {
    title: String,
    menus: Vec<(Option<State>, String)>,
}

impl Screen {
    fn new(title: &str, menus: Vec<(Option<State>, &str)>) -> Self {
        Screen {
            title: title.to_string(),
            menus: menus
                .iter()
                .map(|&(state, menu)| (state, menu.to_string()))
                .collect(),
        }
    }
}

fn get_screen(state: State) -> Screen {
    match state {
        State::Menu0 => Screen::new(
            "Company Employees",
            vec![
                (Some(State::Menu1), "View data"),
                (Some(State::Menu2), "Add data"),
                (Some(State::Menu3), "Edit data"),
                (Some(State::Menu4), "Delete data"),
            ],
        ),
        State::Menu1 => Screen::new(
            "View Data",
            vec![
                (Some(State::Menu11), "View all employees"),
                (Some(State::Menu12), "View all employees in a department"),
            ],
        ),
        State::Menu2 => Screen::new(
            "Add Data",
            vec![
                (Some(State::Menu21), "Add an employee"),
                (Some(State::Menu22), "Add a department"),
            ],
        ),
        State::Menu3 => Screen::new(
            "Edit Data",
            vec![
                (Some(State::Menu31), "Edit an employee"),
                (Some(State::Menu32), "Edit a department"),
            ],
        ),
        State::Menu4 => Screen::new(
            "Delete Data",
            vec![
                (Some(State::Menu41), "Delete an employee"),
                (Some(State::Menu42), "Delete a department"),
            ],
        ),
        _ => Screen {
            title: String::new(),
            menus: vec![],
        },
    }
}

fn get_parent_state(state: &State) -> Option<State> {
    match state {
        State::Menu1 => Some(State::Menu0),
        State::Menu2 => Some(State::Menu0),
        State::Menu3 => Some(State::Menu0),
        State::Menu4 => Some(State::Menu0),
        State::Menu11 => Some(State::Menu1),
        State::Menu12 => Some(State::Menu1),
        State::Menu21 => Some(State::Menu2),
        State::Menu22 => Some(State::Menu2),
        State::Menu31 => Some(State::Menu3),
        State::Menu32 => Some(State::Menu3),
        State::Menu41 => Some(State::Menu4),
        State::Menu42 => Some(State::Menu4),
        _ => None,
    }
}

fn select_department(db: &BTreeMap<Department, Employees>) -> Option<Department> {
    println!("SELECT DEPARTMENT");

    let departments: Vec<Department> = db.keys().cloned().collect();
    let selections_length = departments.len();
    for selection in 1..=selections_length {
        println!("{}. {}", selection, departments[selection - 1]);
    }
    println!("0. Back");

    loop {
        match selections_length {
            0 => println!("Select [0]: "),
            1 => println!("Select [1]: "),
            _ => println!("Select [1-{selections_length}]: "),
        }

        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                let selection: Result<usize, _> = selection.trim().parse();
                match selection {
                    Ok(selection) => {
                        if selection == 0 {
                            break;
                        } else if selection > selections_length {
                            println!("invalid selection");
                        } else {
                            let department = departments[selection - 1].to_string();
                            return Some(department);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
            }
            Err(error) => println!("{error}"),
        }
    }

    return None;
}

fn select_employee(employees: &Employees) -> Option<usize> {
    println!("SELECT EMPLOYEE");

    let selections_length = employees.len();
    for selection in 1..=selections_length {
        println!("{}. {}", selection, employees[selection - 1]);
    }
    println!("0. Back");

    loop {
        match selections_length {
            0 => println!("Select [0]: "),
            1 => println!("Select [1]: "),
            _ => println!("Select [1-{selections_length}]: "),
        }

        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                let selection: Result<usize, _> = selection.trim().parse();
                match selection {
                    Ok(selection) => {
                        if selection == 0 {
                            break;
                        } else if selection > selections_length {
                            println!("invalid selection");
                        } else {
                            return Some(selection - 1);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
            }
            Err(error) => println!("{error}"),
        }
    }

    return None;
}

fn view_all_employees(company_employees: &BTreeMap<Department, Employees>) {
    println!("VIEW ALL EMPLOYEES");

    println!("{:#?}", &company_employees);

    println!("0. Back");
    loop {
        println!("Select [0]: ");
        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                let selection: Result<usize, _> = selection.trim().parse();
                match selection {
                    Ok(selection) => match selection {
                        0 => return,
                        _ => continue,
                    },
                    Err(error) => println!("{error}"),
                }
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn view_all_employees_in_a_department(
    department: &Department,
    company_employees: &BTreeMap<Department, Employees>,
) {
    println!("{}", department.to_uppercase());

    println!(
        "{:#?}",
        &company_employees.get(department).unwrap_or(&vec![])
    );

    println!("0. Back");
    loop {
        println!("Select [0]: ");
        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                let selection: Result<usize, _> = selection.trim().parse();
                match selection {
                    Ok(selection) => match selection {
                        0 => return,
                        _ => continue,
                    },
                    Err(error) => println!("{error}"),
                }
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn add_employee(department: &Department, company_employees: &mut BTreeMap<Department, Employees>) {
    println!("{}", department.to_uppercase());

    let mut employee = String::new();
    loop {
        print_flush("Employee name: ");
        match io::stdin().read_line(&mut employee) {
            Ok(_) => {
                if employee.trim().len() == 0 {
                    println!("invalid employee name");
                    continue;
                }
                print_flush("Confirm employee name [Y/n]: ");
                let mut confirmation = String::new();
                match io::stdin().read_line(&mut confirmation) {
                    Ok(_) => {
                        if confirmation.trim().len() == 0
                            || confirmation.to_uppercase().starts_with("Y")
                        {
                            company_employees
                                .entry(department.to_string())
                                .and_modify(|employees| {
                                    employees.push(employee.trim().to_string());
                                    employees.sort();
                                })
                                .or_insert(vec![]);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
                return;
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn add_department(company_employees: &mut BTreeMap<Department, Employees>) {
    println!("ADD DEPARTMENT");

    let mut department = String::new();
    loop {
        print_flush("Department name: ");
        match io::stdin().read_line(&mut department) {
            Ok(_) => {
                if department.trim().len() == 0 {
                    println!("invalid department name");
                    continue;
                }
                print_flush("Confirm department name [Y/n]: ");
                let mut confirmation = String::new();
                match io::stdin().read_line(&mut confirmation) {
                    Ok(_) => {
                        if confirmation.trim().len() == 0
                            || confirmation.to_uppercase().starts_with("Y")
                        {
                            company_employees
                                .entry(department.trim().to_string())
                                .or_insert(vec![]);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
                return;
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn edit_employee(
    employee: usize,
    department: &Department,
    company_employees: &mut BTreeMap<Department, Employees>,
) {
    let default = Vec::new();
    let employee_name = &company_employees.get(department).unwrap_or(&default)[employee];
    println!("{}", employee_name.to_uppercase());

    println!("Department name: {department}");

    let mut employee_name = String::new();
    loop {
        print_flush("Change employee name to: ");
        match io::stdin().read_line(&mut employee_name) {
            Ok(_) => {
                if employee_name.trim().len() == 0 {
                    println!("invalid employee name");
                    continue;
                }
                print_flush("Confirm employee name [Y/n]: ");
                let mut confirmation = String::new();
                match io::stdin().read_line(&mut confirmation) {
                    Ok(_) => {
                        if confirmation.trim().len() == 0
                            || confirmation.to_uppercase().starts_with("Y")
                        {
                            company_employees
                                .entry(department.to_string())
                                .and_modify(|employees| {
                                    employees[employee] = employee_name.trim().to_string()
                                })
                                .or_insert(vec![]);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
                return;
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn edit_department(
    department: &Department,
    company_employees: &mut BTreeMap<Department, Employees>,
) {
    println!("{}", department.to_uppercase());

    let mut department_name = String::new();
    loop {
        print_flush("Change department name to: ");
        match io::stdin().read_line(&mut department_name) {
            Ok(_) => {
                if department_name.trim().len() == 0 {
                    println!("invalid department name");
                    continue;
                }
                print_flush("Confirm department name [Y/n]: ");
                let mut confirmation = String::new();
                match io::stdin().read_line(&mut confirmation) {
                    Ok(_) => {
                        if confirmation.trim().len() == 0
                            || confirmation.to_uppercase().starts_with("Y")
                        {
                            let employees = company_employees.remove(department).unwrap_or(vec![]);
                            company_employees.insert(department_name.trim().to_string(), employees);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
                return;
            }
            Err(error) => println!("{error}"),
        }
    }
}

fn delete_employee(
    employee: usize,
    department: &Department,
    company_employees: &mut BTreeMap<Department, Employees>,
) {
    let default = Vec::new();
    let employee_name = &company_employees.get(department).unwrap_or(&default)[employee];
    println!("{}", employee_name.to_uppercase());

    println!("Department name: {department}");

    print_flush("Confirm delete employee [Y/n]: ");
    let mut confirmation = String::new();
    match io::stdin().read_line(&mut confirmation) {
        Ok(_) => {
            if confirmation.trim().len() == 0 || confirmation.to_uppercase().starts_with("Y") {
                company_employees
                    .entry(department.to_string())
                    .and_modify(|employees| {
                        employees.remove(employee);
                    });
            }
        }
        Err(error) => println!("{error}"),
    }
}

fn delete_department(
    department: &Department,
    company_employees: &mut BTreeMap<Department, Employees>,
) {
    println!("{}", department.to_uppercase());

    print_flush("Confirm delete department [Y/n]: ");
    let mut confirmation = String::new();
    match io::stdin().read_line(&mut confirmation) {
        Ok(_) => {
            if confirmation.trim().len() == 0 || confirmation.to_uppercase().starts_with("Y") {
                company_employees.remove(department);
            }
        }
        Err(error) => println!("{error}"),
    }
}

type Department = String;
type Employees = Vec<String>;
fn company_employees() {
    let mut company_employees: BTreeMap<Department, Employees> = BTreeMap::new();

    let mut state = State::Menu0;
    loop {
        let mut process_back = true;

        let screen = get_screen(state);
        match state {
            State::Menu11 => view_all_employees(&company_employees),
            State::Menu12 => {
                if let Some(department) = select_department(&company_employees) {
                    view_all_employees_in_a_department(&department, &company_employees);
                }
            }
            State::Menu21 => {
                if let Some(department) = select_department(&company_employees) {
                    add_employee(&department, &mut company_employees);
                }
            }
            State::Menu22 => add_department(&mut company_employees),
            State::Menu31 => {
                if let Some(department) = select_department(&company_employees) {
                    let default = Vec::new();
                    let employees = company_employees.get(&department).unwrap_or(&default);
                    if let Some(employee) = select_employee(employees) {
                        edit_employee(employee, &department, &mut company_employees);
                    }
                }
            }
            State::Menu32 => {
                if let Some(department) = select_department(&company_employees) {
                    edit_department(&department, &mut company_employees);
                }
            }
            State::Menu41 => {
                if let Some(department) = select_department(&company_employees) {
                    let default = Vec::new();
                    let employees = company_employees.get(&department).unwrap_or(&default);
                    if let Some(employee) = select_employee(employees) {
                        delete_employee(employee, &department, &mut company_employees);
                    }
                }
            }
            State::Menu42 => {
                if let Some(department) = select_department(&company_employees) {
                    delete_department(&department, &mut company_employees);
                }
            }
            _ => process_back = false,
        }

        if process_back {
            match get_parent_state(&state) {
                Some(parent) => state = parent,
                None => break,
            }
            continue;
        }

        let mut states = Vec::new();
        let mut menus = Vec::new();
        for (state, menu) in screen.menus {
            states.push(state);
            menus.push(menu);
        }

        match prompt_selectable_menus(
            &screen.title,
            &menus,
            &if state == State::Menu0 {
                Previous::Quit
            } else {
                Previous::Back
            },
        ) {
            Some(selected_index) => state = states[selected_index].unwrap_or(state),
            None => match get_parent_state(&state) {
                Some(parent) => state = parent,
                None => break,
            },
        }
    }
}

fn prompt_selectable_menus(
    title: &String,
    selections: &Vec<String>,
    prev: &Previous,
) -> Option<usize> {
    println!("{}", title.to_uppercase());

    let selections_length = selections.len();
    for selection in 1..=selections_length {
        println!("{}. {}", selection, selections[selection - 1]);
    }
    println!(
        "0. {}",
        match prev {
            Previous::Quit => "Quit",
            Previous::Back => "Back",
        }
    );

    loop {
        println!("Select [1-{selections_length}]: ");

        let mut selection = String::new();
        match io::stdin().read_line(&mut selection) {
            Ok(_) => {
                let selection: Result<usize, _> = selection.trim().parse();
                match selection {
                    Ok(selection) => {
                        if selection == 0 {
                            break;
                        } else if selection > selections_length {
                            println!("invalid selection");
                        } else {
                            return Some(selection - 1);
                        }
                    }
                    Err(error) => println!("{error}"),
                }
            }
            Err(error) => println!("{error}"),
        }
    }

    return None;
}
