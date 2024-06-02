use super::{dictionary::Dict, fruchterman_reingold::Graph};
use crate::app::structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem};
use leptos::{RwSignal, SignalGet, SignalGetUntracked, SignalSet};
use leptos_use::core::Position;
use log::debug;
use std::{collections::HashMap, vec};

pub fn organize_positions(
    items: Vec<RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
) -> Vec<RwSignal<MoveBoxItem>> {
    let mut to_rank_dict = Dict::<String, i32>::new();
    let mut from_rank_dict = Dict::<String, i32>::new();

    items.iter().for_each(|x| {
        let key = x.get().key.to_string();
        to_rank_dict.insert(key.clone(), 0);
        from_rank_dict.insert(key.clone(), 0);
    });

    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();
        to_rank_dict.insert(to.clone(), to_rank_dict.get(to).unwrap() + 1);
        from_rank_dict.insert(from.clone(), from_rank_dict.get(from.clone()).unwrap() + 1);
    });

    let mut to_rankneighbour_weighted = Dict::<String, i32>::new();
    let mut from_rankneighbour_weighted = Dict::<String, i32>::new();

    items.iter().for_each(|x| {
        let key = x.get().key.to_string();
        to_rankneighbour_weighted.insert(key.clone(), *to_rank_dict.get(key.clone()).unwrap());
        from_rankneighbour_weighted.insert(key.clone(), *from_rank_dict.get(key.clone()).unwrap());
    });

    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();
        let to_neighbour_value = to_rank_dict.get(from.clone());
        let from_neighbour_value = from_rank_dict.get(to.clone());
        if to_neighbour_value.is_some() {
            to_rankneighbour_weighted.insert(
                to.clone(),
                *to_neighbour_value.unwrap() + to_rankneighbour_weighted.get(to.clone()).unwrap(),
            );
        }
        if from_neighbour_value.is_some() {
            from_rankneighbour_weighted.insert(
                from.clone(),
                *from_neighbour_value.unwrap()
                    + from_rankneighbour_weighted.get(from.clone()).unwrap(),
            );
        }
    });

    let mut to_rank_over_view = Dict::<i32, Vec<String>>::new();
    let mut from_rank_over_view = Dict::<i32, Vec<String>>::new();

    to_rank_dict.into_iter().for_each(|x| {
        if to_rank_over_view.get(x.1).is_none() {
            to_rank_over_view.insert(x.1, vec![x.0]);
        } else {
            to_rank_over_view[x.1].push(x.0);
        }
    });

    from_rank_dict.into_iter().for_each(|x| {
        if from_rank_over_view.get(x.1).is_none() {
            from_rank_over_view.insert(x.1, vec![x.0]);
        } else {
            from_rank_over_view[x.1].push(x.0);
        }
    });

    let mut xlevels = vec![];
    to_rank_over_view.clone().into_iter().for_each(|y| {
        xlevels.push(y.1.len() as i32);
    });
    let spacing = 350.0;
    let (xOrdering, _x_ranks) = organize_xpositions(to_rank_over_view.clone(), connections.clone());
    items.iter().for_each(|item| {
        let ranks = to_rank_over_view
            .clone()
            .into_iter()
            .find(|x| x.1.contains(&item.get().key))
            .unwrap();
        let x_level = xOrdering.get(item.get().key.clone()).unwrap();
        let y_level = ranks.0;

        let size = item.get().size.get();
        let y_position = spacing / 2.0 + (size.y + 145.0) * y_level as f64;
        let x_position = spacing * 2.0 + x_level * (size.x + 145.0);
        item.get().realPosition.set(Position {
            x: x_position,
            y: y_position,
        });
        item.get().position.set(Position {
            x: x_position,
            y: y_position,
        });
    });
    items
}

fn get_neighbours(connections: Vec<RwSignal<ConnectionItem>>) -> Dict<String, Vec<String>> {
    let mut neighbours = Dict::<String, Vec<String>>::new();
    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();

        if neighbours.get(from.clone()).is_none() {
            neighbours.insert(from.clone(), vec![to.clone()]);
        } else {
            neighbours[from.clone()].push(to.clone());
        }
        if neighbours.get(to.clone()).is_none() {
            neighbours.insert(to.clone(), vec![from.clone()]);
        } else {
            neighbours[to.clone()].push(from.clone());
        }
    });
    neighbours
}

fn get_xrank(item_id: String, x_ranks: &Dict<String, f64>) -> f64 {
    let rank = x_ranks.get(item_id);
    if rank.is_some() {
        return *rank.unwrap();
    }
    0.0
}

fn organize_xpositions(
    to_rank_over_view: Dict<i32, Vec<String>>,
    connections: Vec<RwSignal<ConnectionItem>>,
) -> (Dict<String, f64>, Dict<String, f64>) {
    let mut to_rank_keys = to_rank_over_view
        .clone()
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<i32>>();
    if to_rank_keys.is_empty() {
        return (Dict::<String, f64>::new(), Dict::<String, f64>::new());
    }
    let mut x_ranks = Dict::<String, f64>::new();
    let neighbour_dict = get_neighbours(connections);
    let mut ordering = Dict::<String, f64>::new();

    to_rank_keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let first_key = to_rank_keys.first().unwrap();
    initialize(&to_rank_over_view[*first_key], &mut x_ranks);
    to_rank_keys.into_iter().for_each(|key| {
        let items = to_rank_over_view[key].clone();
        populate_with_neighbour_values(&items, &neighbour_dict, &mut x_ranks);
        populate_with_neighbour_values(&items, &neighbour_dict, &mut x_ranks);
        order(&mut ordering, &items, &mut x_ranks)
    });
    (ordering, x_ranks)
}

fn populate_with_neighbour_values(
    items: &Vec<String>,
    neighbour_dict: &Dict<String, Vec<String>>,
    x_ranks: &mut Dict<String, f64>,
) {
    items.iter().for_each(|item| {
        let neighbours = neighbour_dict.get(item.clone());
        let mut xrank = get_xrank(item.clone(), x_ranks);
        if neighbours.is_some() {
            neighbours.unwrap().iter().for_each(|neighbour| {
                let neigh_bour_xrank = get_xrank(neighbour.clone(), x_ranks);
                xrank += neigh_bour_xrank;
            });
        }
        x_ranks.insert(item.clone(), xrank);
    });
}
fn initialize(items: &Vec<String>, x_ranks: &mut Dict<String, f64>) {
    let mut order = vec![];
    items.iter().for_each(|item| {
        order.push((item.clone(), get_xrank(item.clone(), x_ranks)));
    });
    let mut index = 0.0;
    let increments = 100.0;
    order.iter().for_each(|x| {
        let median = (order.len() / 2) as f64;
        x_ranks.insert(x.0.clone(), (index - median) * increments);
        index += 1.0;
    });
}

fn order(ordering: &mut Dict<String, f64>, items: &Vec<String>, x_ranks: &mut Dict<String, f64>) {
    let mut order = vec![];
    items.iter().for_each(|item| {
        order.push((item.clone(), get_xrank(item.clone(), x_ranks)));
    });
    order.sort_by(|a, b| (b.1.partial_cmp(&a.1).unwrap()));
    let mut index = 0.0;
    order.iter().for_each(|x| {
        let median = (order.len() / 2) as f64;
        ordering.insert(x.0.clone(), index - median);
        index += 1.0;
    });
}

pub fn organize_positions_fruchterman_reingold(
    items: Vec<RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
) -> Vec<RwSignal<MoveBoxItem>> {
    let longest_text = find_longest_text_in_items(items.clone());
    let scale_value = longest_text * 6.0;
    let positive_offset = 500.0;
    let itemIds = items.iter().map(|x| x.get().key.clone()).collect();
    let connectionIds = connections
        .iter()
        .map(|x| {
            (
                x.get_untracked().from.get_untracked().key.clone(),
                x.get_untracked().to.get_untracked().key.clone(),
            )
        })
        .collect();
    let mut graph = Graph::new(itemIds, connectionIds);
    graph.fruchterman_reingold(1000, 2.0, 0.09, 0.1, 0.001);
    let positions: HashMap<_, _> = graph
        .nodes
        .iter()
        .map(|node| (&node.id, &node.pos))
        .collect();

    items.iter().for_each(|item| {
        let move_box = item.get_untracked();
        let position = positions[&move_box.key];
        let newPos = Position {
            x: position.x * scale_value + positive_offset,
            y: position.y * scale_value + 400.0,
        };
        move_box.realPosition.set(newPos);
    });
    fix_overlap(&items);

    items
}

fn fix_overlap(items: &Vec<RwSignal<MoveBoxItem>>) {
    let num_items = items.len();
    let mut iterations = 0;
    loop {
        let mut overlaps_resolved = false;
        for i in 0..num_items {
            for j in (i + 1)..num_items {
                let (node1, node2) = (&mut items[i].get(), &mut items[j].get());
                if check_overlap(node1, node2) {
                    resolve_overlap(node1, node2);
                    overlaps_resolved = true;
                }
            }
        }
        if !overlaps_resolved || iterations > 2 {
            break;
        }

        iterations += 1;
    }
}

pub fn find_longest_text(item: MoveBoxItem) -> f64 {
    let mut longest_len = 20.0;
    item.attributes.get().iter().for_each(|attribute| {
        let len = attribute.value.get().len() as f64;
        if len > longest_len {
            longest_len = len
        }
    });
    longest_len
}

pub fn find_longest_text_in_items(items: Vec<RwSignal<MoveBoxItem>>) -> f64 {
    let mut longest_len = 20.0;
    items.iter().for_each(|item| {
        let len = find_longest_text(item.get());
        if len > longest_len {
            longest_len = len
        }
    });
    longest_len
}

fn smallest_distances_for_each_position(positions: &Vec<i32>) -> Vec<Option<i32>> {
    if positions.len() < 2 {
        return vec![None; positions.len()]; // Handle the edge case of a single or empty list
    }

    // Step 1: Sort the positions and keep track of original indices
    let mut sorted_positions: Vec<(usize, i32)> =
        positions.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    sorted_positions.sort_by_key(|&(_, v)| v);

    // Step 2: Initialize a vector to store the smallest distances
    let mut smallest_distances = vec![i32::MAX; positions.len()];

    // Step 3: Iterate through the sorted positions and find the smallest distance for each position
    for i in 0..sorted_positions.len() {
        if i > 0 {
            let distance = sorted_positions[i].1 - sorted_positions[i - 1].1;
            smallest_distances[sorted_positions[i].0] =
                smallest_distances[sorted_positions[i].0].min(distance);
            smallest_distances[sorted_positions[i - 1].0] =
                smallest_distances[sorted_positions[i - 1].0].min(distance);
        }
    }

    // Step 4: Convert smallest distances to Option<i32> to handle positions with no valid distance
    smallest_distances
        .into_iter()
        .map(|d| if d == i32::MAX { None } else { Some(d) })
        .collect()
}

fn check_overlap_new_pos(position: &Position, size: &Position, move_box2: &MoveBoxItem) -> bool {
    let position2 = move_box2.position.get();
    let size2 = move_box2.size.get();
    let overlap_x = (position.x - position2.x).abs() < (size.x + size2.x) / 2.0;
    let overlap_y = (position.y - position2.y).abs() < (size.y + size2.y) / 2.0;
    overlap_x && overlap_y
}

fn check_overlap(move_box1: &MoveBoxItem, move_box2: &MoveBoxItem) -> bool {
    let position1 = move_box1.position.get();
    let position2 = move_box2.position.get();
    let size1 = move_box1.size.get();
    let size2 = move_box2.size.get();
    let overlap_x = (position1.x - position2.x).abs() < (size1.x + size2.x) / 2.0;
    let overlap_y = (position1.y - position2.y).abs() < (size1.y + size2.y) / 2.0;
    overlap_x && overlap_y
}

fn resolve_overlap(move_box1: &mut MoveBoxItem, move_box2: &mut MoveBoxItem) {
    let position1 = move_box1.realPosition.get();
    let position2 = move_box2.realPosition.get();
    let size1 = move_box1.size.get();
    let size2 = move_box2.size.get();
    let mut overlap_x = (size1.x + size2.x) / 2.0 - (position1.x - position2.x).abs();
    let mut overlap_y = (size1.y + size2.y) / 2.0 - (position1.y - position2.y).abs();

    let extra_space = 10.0;

    if overlap_x <= 0.0 || overlap_y <= 0.0 {
        return;
    }
    overlap_x += extra_space;
    overlap_y += extra_space;

    if position1.y < position2.y {
        move_box1.realPosition.set(Position {
            x: position1.x,
            y: position1.y - overlap_y,
        });
        move_box2.realPosition.set(Position {
            x: position2.x,
            y: position2.y + overlap_y,
        });
    } else {
        move_box1.realPosition.set(Position {
            x: position1.x,
            y: position1.y + overlap_y,
        });
        move_box2.realPosition.set(Position {
            x: position2.x,
            y: position2.y - overlap_y,
        });
    }
    if position1.x < position2.x {
        move_box1.realPosition.set(Position {
            x: position1.x + overlap_x,
            y: position1.y,
        });
        move_box2.realPosition.set(Position {
            x: position2.x - overlap_x,
            y: position2.y,
        });
    } else {
        move_box1.realPosition.set(Position {
            x: position1.x - overlap_x,
            y: position1.y,
        });
        move_box2.realPosition.set(Position {
            x: position2.x + overlap_x,
            y: position2.y,
        });
    }
}
pub fn set_size(items: Vec<RwSignal<MoveBoxItem>>) -> Vec<RwSignal<MoveBoxItem>> {
    let const_scale_x = 8.0;
    let const_scale_y = 20.0;

    items.iter().for_each(|item| {
        let longest_text_length = find_longest_text(item.get());
        let num_attributes = item.get().attributes.get().len() as f64;
        let mut x = longest_text_length * const_scale_x;
        let y = 40.0 + num_attributes * const_scale_y;
        if x <= 0.0 {
            x = 60.0;
        }

        item.get().size.set(Position { x, y })
    });
    items
}
