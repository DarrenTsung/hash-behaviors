mod hash_router;
use colorful::Colorful;
use std::collections::HashMap;
use std::io::{self, Write};
use uuid::Uuid;

#[derive(Debug, Clone, PartialOrd, Ord, Copy, PartialEq, Eq, Hash)]
struct Target(usize);

#[derive(Debug)]
struct KeyIndex(usize);

fn main() {
    let keys = (0..100_000)
        .map(|_| Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    let mut routers = hash_router::routers();
    let targets = vec![Target(0), Target(1), Target(2), Target(3), Target(4)];
    let target_colors = {
        let step = 1.0 / targets.len() as f32;
        let mut colors = vec![];
        let mut hue = 0.0;
        while hue <= 1.0 {
            colors.push(colorful::HSL::new(hue, 1.0, 0.85));
            hue += step;
        }
        colors
    };

    for router in &mut routers {
        router.set_targets(targets.clone());
    }

    for router in routers {
        print!("Checking that router: {} is consistent.. ", router);
        let generate_routed_keys = || {
            keys.iter()
                .map(|key| router.route(&key))
                .collect::<Vec<_>>()
        };

        let routed_keys = generate_routed_keys();
        if routed_keys != generate_routed_keys() {
            println!("ERROR - not consistent!");
        } else {
            println!("done!");
        }

        let mut displayed_keys_grouped_by_target = HashMap::new();
        for (index, displayed_key) in keys.iter().enumerate().take(1_000) {
            let target = routed_keys[index];
            if !displayed_keys_grouped_by_target.contains_key(&target) {
                displayed_keys_grouped_by_target.insert(target, vec![]);
            }

            displayed_keys_grouped_by_target
                .get_mut(&target)
                .expect("exists")
                .push((KeyIndex(index), displayed_key));
        }

        print_grouped_keys(
            &displayed_keys_grouped_by_target,
            &routed_keys,
            &target_colors,
        );
    }
}

fn print_grouped_keys(
    grouped_keys: &HashMap<Target, Vec<(KeyIndex, &String)>>,
    routed_keys: &[Target],
    target_colors: &[colorful::HSL],
) {
    let mut target_order = grouped_keys.keys().collect::<Vec<_>>();
    target_order.sort();

    for (target_index, target) in target_order.iter().enumerate() {
        for key_chunk in grouped_keys.get(target).expect("exists").chunks(80) {
            for (key_index, _key) in key_chunk {
                // Note that routed target can differ from grouped target because
                // we group the keys at the initial state and then modify the set
                // of targets, etc.
                let routed_target = routed_keys[key_index.0];
                let color = target_colors[routed_target.0];
                print!("{}", "0".color(color));
            }
            print!("\n");
            io::stdout().flush().unwrap();
        }

        // If not last group.
        if target_index != target_order.len() - 1 {
            println!("");
        }
    }
}
