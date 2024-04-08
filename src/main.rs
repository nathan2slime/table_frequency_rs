use std::io;

#[derive(Clone, Copy)]
struct ItemSimple {
    value: i32,
    fi: i32,
    fi_acc: i32,
    fri: f32,
    fri_acc: f32,
}

struct FrequencySimple {
    k: i32,
    total: i32,
    media: f32,
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

    let frequency_items: Vec<ItemSimple> = Vec::new();
    let mut frequency_simple = FrequencySimple {
        k: 0,
        media: 0.0,
        total: frequency_items.len() as i32,
        items: frequency_items,
    };


    raw_data.iter().for_each(|v| {
        let item_index = frequency_simple.items.iter().position(|item| {
            item.value == *v
        });

        if item_index.is_none() {
            frequency_simple.items.push(ItemSimple {
                value: *v,
                fi: 1,
                fi_acc: 1,
                fri: 0.0,
                fri_acc: 0.0,
            })
        } else {
            frequency_simple.items = frequency_simple.items.iter().enumerate().map(|(i, v)| {
                if item_index.unwrap() == i {
                    let fi = v.fi + 1;
                    let fi_acc = fi;

                    ItemSimple {
                        value: v.value,
                        fi,
                        fi_acc,
                        fri: 0.0,
                        fri_acc: 0.0,
                    }
                } else {
                    *v
                }
            }).collect();
        }
    });

    frequency_simple.items.iter().for_each(|item| {
        frequency_simple.total += item.fi;
    });


    frequency_simple.items = frequency_simple.items.iter().map(|item| {
        let fri = (item.fi as f32 / frequency_simple.total as f32) * 100.0;

        ItemSimple {
            fri,
            fi: item.fi,
            value: item.value,
            fi_acc: item.fi_acc,
            fri_acc: fri,
        }
    }).collect();

    let mut last_fi_acc: Option<i32> = Option::None;
    let mut last_fri_acc: Option<f32> = Option::None;

    let mut pre_frequency_media: f32 = 0.0;

    frequency_simple.items.iter().for_each(|item| pre_frequency_media += (item.value * item.fi) as f32);

    frequency_simple.media = pre_frequency_media / frequency_simple.total as f32;


    frequency_simple.items = frequency_simple.items.iter().enumerate().map(|(i, item)| {
        if i > 0 {
            let fi_acc = item.fi + last_fi_acc.unwrap();
            let fri_acc = item.fri_acc + last_fri_acc.unwrap();

            last_fi_acc = Option::from(fi_acc);
            last_fri_acc = Option::from(fri_acc);

            ItemSimple {
                fri: item.fri,
                fi: item.fi,
                value: item.value,
                fri_acc,
                fi_acc,
            }
        } else {
            last_fi_acc = Option::from(item.fi);
            last_fri_acc = Option::from(item.fri);

            *item
        }
    }).collect();


    println!("\n\nTABELA");
    println!("╔════════════════════════════════════════╗");

    println!("k = {}", frequency_simple.k);
    println!("Total = {}", frequency_simple.total);
    println!("Média ponderada = {}", frequency_simple.media);
    println!(
        "\n{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}",
        "Valor", "fi", "fri (%)", "Fi", "Fri (%)"
    );

    frequency_simple.items.iter().for_each(|e|
        {
            println!("{0: <10} | {1: <10} | {2: <10} | {3: <10} | {4: <10}", e.value, e.fi, e.fri, e.fi_acc, e.fri_acc);
        });

    0
}

fn generate_frequency_interval(_: i32) -> i32 {
    0
}

