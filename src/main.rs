use std::io;

#[derive(Clone, Copy)]
struct ItemSimple {
    value: i32,
    fi: i32,
}

struct FrequencySimple {
    k: Option<i32>,
    items: Vec<ItemSimple>,
}

struct QuestionOption<'a> {
    name: &'a str,
    action: fn(i32) -> i32,
}

fn generate_question(title: &str, value: &mut String) {
    eprint!("\n{}: ", title);

    io::stdin().read_line(value).unwrap();
}

fn generate_choose_question(title: &str, options: Vec<QuestionOption>) -> i32 {
    for (i, e) in options.iter().enumerate() {
        eprint!("\n{}. {}", i + 1, e.name);
    }

    let mut answer_index = String::new();
    generate_question(title, &mut answer_index);

    match answer_index.trim().parse::<i32>() {
        Ok(n) => {
            let answer = options.get((n - 1) as usize).unwrap_or(&options[0]);

            (answer.action)(n)
        }
        Err(_) => {
            0
        }
    }
}

fn main() {
    println!("╔════════════════════════════════╗");
    println!("║  🍓 Frequency Distribution 🍓  ║");
    println!("╠════════════════════════════════╣");
    println!("╚════════════════════════════════╝");


    generate_choose_question(&String::from("Qual o tipo de frequência"), vec![QuestionOption {
        action: generate_frequency_simple,
        name: "Simples",
    }, QuestionOption {
        action: generate_frequency_interval,
        name: "Intervalo",
    }]);
}

fn generate_frequency_simple(_: i32) -> i32 {
    println!("\n╔════════════════════════════════════════╗");
    println!("\nFrequência Simples");


    let mut raw_data: Vec<i32> = Vec::new();

    loop {
        let answer = generate_choose_question("Escolha uma ação", vec![QuestionOption {
            name: "Novo dado",
            action: |_| {
                let mut data = String::new();
                generate_question("Valor do dado", &mut data);

                data.trim().parse::<i32>().unwrap_or(0)
            },
        }, QuestionOption {
            name: "Finalizar",
            action: |_| {
                -1
            },
        }]);

        if answer == -1 { break; } else {
            raw_data.push(answer);
        };
    };

    raw_data.sort();

    let mut frequency_items: Vec<ItemSimple> = Vec::new();
    let mut frequency_simple = FrequencySimple {
        k: Option::None,
        items: frequency_items,
    };


    raw_data.iter().for_each(|v| {
        let item_index = frequency_simple.items.iter().position(|item| {
            item.value == *v
        });

        if (item_index.is_none()) {
            frequency_simple.items.push(ItemSimple {
                value: *v,
                fi: 1,
            })
        } else {
            let mut new_frequency_simple = Vec::new();


            for (i, v) in frequency_simple.items.iter().enumerate() {
                if (item_index.unwrap() == i) {
                    new_frequency_simple.push(ItemSimple {
                        value: v.value,
                        fi: v.fi + 1,
                    })
                } else {
                    new_frequency_simple.push(*v);
                }
            }

            frequency_simple.items = new_frequency_simple;
        }
    });

    println!("\n\nTabela");
    println!(
        "\n{0: <10} | {1: <10}",
        "Valor", "Fi"
    );

    frequency_simple.items.iter().for_each(|e|
        {
            println!("{0: <10} | {1: <10}", e.value, e.fi);
        });

    0
}

fn generate_frequency_interval(_: i32) -> i32 {
    println!("Intervalo");
    // loop {
    //     let mut frequency_type = String::new();
    //
    //     eprint!("\nQual o tipo de frequência: ");
    //
    //     io::stdin().read_line(&mut frequency_type).unwrap();
    // }

    0
}

