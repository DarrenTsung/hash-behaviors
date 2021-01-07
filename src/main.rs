mod hash_router;
use uuid::Uuid;

fn main() {
    let keys = (0..100_000)
        .map(|_| Uuid::new_v4().to_string())
        .collect::<Vec<_>>();
    let mut routers = hash_router::routers();
    let targets = vec!["A", "B", "C", "D", "E"];
    for router in &mut routers {
        router.set_targets(targets.iter().map(|s| s.to_string()).collect());
    }

    println!("Checking that all hash routers are consistent..");
    for router in routers {
        let route_keys = || {
            keys.iter()
                .map(|key| router.route(&key))
                .collect::<Vec<_>>()
        };

        if route_keys() != route_keys() {
            println!("ERROR - router: {} is not consistent!", router);
        }
    }
    println!("done!");
}
