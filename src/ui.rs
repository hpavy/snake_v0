pub fn display_game(
    head_position: &(i16, i16),
    apple_position: &(i16, i16),
    entire_body: &Vec<(i16, i16)>
){
    let mut string_to_show = String::from("");
    for y in 0..15{
        for x in 0..15 {
            let current_pos = (x, y);
            if *head_position == current_pos{
                string_to_show.push_str("\x1b[32mS \x1b[0m");
            } else if *apple_position == current_pos{
                string_to_show.push_str("\x1b[31mA \x1b[0m")
            } else if entire_body.contains(&current_pos) {
                string_to_show.push_str("\x1b[32mo \x1b[0m");
            } else{
                string_to_show.push_str("# ");
            }
        }
        string_to_show.push('\n');
    }
    println!("{}", string_to_show);   
} 