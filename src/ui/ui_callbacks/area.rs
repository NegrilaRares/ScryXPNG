use ratatui::{
    layout::{Layout, Rect},
    prelude::Constraint::{Fill, Length, Percentage},
};

pub fn application_area(area: Rect) -> Rect {
    let application_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, application_veritcal, _] = application_vertical_layout.areas(area);

    let application_horizontal_layout =
        Layout::horizontal([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, application_area, _] = application_horizontal_layout.areas(application_veritcal);
    application_area
}

//
// 0 - destination
//
pub fn partition_application_area_0(application_area: Rect) -> Rect {
    let destination_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, destination_veritcal, _] = destination_vertical_layout.areas(application_area);

    let destination_horizontal_layout =
        Layout::horizontal([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, destination_area, _] = destination_horizontal_layout.areas(destination_veritcal);
    destination_area
}

//
// 1 - destination + help
//
pub fn partition_application_area_1(application_area: Rect) -> [Rect; 2] {
    let application_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, application_veritcal, _] = application_vertical_layout.areas(application_area);

    let application_horizontal_layout = Layout::horizontal([
        Percentage(2),
        Percentage(28),
        Percentage(1),
        Percentage(67),
        Percentage(2),
    ]);

    let [_, help_area, _, destination_area, _] =
        application_horizontal_layout.areas(application_veritcal);
    [destination_area, help_area]
}

//
// 2 - destination + list
//
pub fn partition_application_area_2(application_area: Rect) -> [Rect; 2] {
    let application_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, application_veritcal, _] = application_vertical_layout.areas(application_area);

    let application_horizontal_layout = Layout::horizontal([
        Percentage(2),
        Percentage(47),
        Percentage(1),
        Percentage(48),
        Percentage(2),
    ]);

    let [_, destination_area, _, list_area, _] =
        application_horizontal_layout.areas(application_veritcal);
    [destination_area, list_area]
}

//
// 3 - destination + list + help
//
pub fn partition_application_area_3(application_area: Rect) -> [Rect; 3] {
    let application_horizontal_layout = Layout::horizontal([
        Percentage(2),
        Percentage(28),
        Percentage(1),
        Percentage(67),
        Percentage(2),
    ]);

    let [_, help_horizontal, _, application_horizontal, _] =
        application_horizontal_layout.areas(application_area);

    let help_vertical_layout = Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, help_area, _] = help_vertical_layout.areas(help_horizontal);

    let application_vertical_layout = Layout::vertical([
        Percentage(2),
        Percentage(47),
        Percentage(1),
        Percentage(48),
        Percentage(2),
    ]);

    let [_, destination_area, _, list_area, _] =
        application_vertical_layout.areas(application_horizontal);

    [destination_area, list_area, help_area]
}

//
// 4 - destination + list + pick_card
//
pub fn partition_application_area_4(application_area: Rect) -> [Rect; 3] {
    let application_horizontal_layout = Layout::horizontal([
        Percentage(2),
        Percentage(47),
        Percentage(1),
        Percentage(48),
        Percentage(2),
    ]);

    let [_, destination_list_horizontal, _, pick_card_horizontal, _] =
        application_horizontal_layout.areas(application_area);

    let destination_list_vertical_layout = Layout::vertical([
        Percentage(2),
        Percentage(47),
        Percentage(1),
        Percentage(48),
        Percentage(2),
    ]);

    let [_, destination_area, _, list_area, _] =
        destination_list_vertical_layout.areas(destination_list_horizontal);

    let pick_card_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, pick_card_area, _] = pick_card_vertical_layout.areas(pick_card_horizontal);

    [destination_area, list_area, pick_card_area]
}

//
// 5 - destination + list + pick_card + help
//
pub fn partition_application_area_5(application_area: Rect) -> [Rect; 4] {
    let application_horizontal_layout = Layout::horizontal([
        Percentage(2),
        Percentage(31),
        Percentage(1),
        Percentage(32),
        Percentage(1),
        Percentage(31),
        Percentage(2),
    ]);

    let [_, help_horizontal, _, destination_list_horizontal, _, pick_card_horizontal, _] =
        application_horizontal_layout.areas(application_area);

    let help_vertical_layout = Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, help_area, _] = help_vertical_layout.areas(help_horizontal);

    let destination_list_vertical_layout = Layout::vertical([
        Percentage(2),
        Percentage(47),
        Percentage(1),
        Percentage(48),
        Percentage(2),
    ]);

    let [_, destination_area, _, list_area, _] =
        destination_list_vertical_layout.areas(destination_list_horizontal);

    let pick_card_vertical_layout =
        Layout::vertical([Percentage(2), Percentage(96), Percentage(2)]);

    let [_, pick_card_area, _] = pick_card_vertical_layout.areas(pick_card_horizontal);

    [destination_area, list_area, pick_card_area, help_area]
}

//
// Destination - Input Area
//
pub fn input_area(destination_area: Rect) -> Rect {
    let input_vertical_layout = Layout::vertical([Fill(1), Length(3), Fill(1)]);

    let [_, input_veritcal, _] = input_vertical_layout.areas(destination_area);

    let input_horizontal_layout = Layout::horizontal([Fill(1), Length(60), Fill(1)]);

    let [_, input_area, _] = input_horizontal_layout.areas(input_veritcal);
    input_area
}

//
// List - Inner Area
//
pub fn list_inner_area(list_area: Rect) -> Rect {
    let list_inner_vertical_layout = Layout::vertical([Fill(1), Length(10), Fill(1)]);

    let [_, list_inner_veritcal, _] = list_inner_vertical_layout.areas(list_area);

    let list_inner_horizontal_layout = Layout::horizontal([Fill(1), Length(40), Fill(1)]);

    let [_, list_inner_area, _] = list_inner_horizontal_layout.areas(list_inner_veritcal);
    list_inner_area
}
