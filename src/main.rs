mod ui;

fn main() {
    let head = (4, 4);
    let apple = (9, 7);
    let body = vec![(4, 5), (4, 6), (5, 6)];

    ui::display_game(&head, &apple, &body);
}