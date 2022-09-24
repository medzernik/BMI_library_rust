use BMI_minesweeper;

fn main() {
    /*loop {
            let user_input = get_user_input(
                "Select what you want to do:
            1 to add a new animal
            2 to find an animal by name
            3 to delete an animal by name",
            );

            let user_input: u8 = match user_input.trim().parse::<u8>() {
                Ok(T) => T,
                Err(E) => {
                    println!("Error: {}", E);
                    continue;
                }
            };




        }
    */
    BMI_minesweeper::test();
}
