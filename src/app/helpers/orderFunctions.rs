use super::dictionary::Dict;
use crate::app::structs::{connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem};
use leptos::{RwSignal, SignalGet, SignalSet};
use leptos_use::core::Position;
use std::vec;

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
        to_rankneighbour_weighted
            .insert(key.clone(), to_rank_dict.get(key.clone()).unwrap().clone());
        from_rankneighbour_weighted.insert(
            key.clone(),
            from_rank_dict.get(key.clone()).unwrap().clone(),
        );
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
    let (xOrdering, x_ranks) = organize_xpositions(to_rank_over_view.clone(), connections.clone());
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
    return (items);
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
    return 0.0;
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
    if to_rank_keys.len() == 0 {
        return (Dict::<String, f64>::new(), Dict::<String, f64>::new());
    }
    let mut x_ranks = Dict::<String, f64>::new();
    let neighbour_dict = get_neighbours(connections);
    let mut ordering = Dict::<String, f64>::new();

    to_rank_keys.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let first_key = to_rank_keys.first().unwrap();
    initialize(&to_rank_over_view[first_key.clone()], &mut x_ranks);
    to_rank_keys.into_iter().for_each(|key| {
        let items = to_rank_over_view[key].clone();
        populate_with_neighbour_values(&items, &neighbour_dict, &mut x_ranks);
        populate_with_neighbour_values(&items, &neighbour_dict, &mut x_ranks);
        order(&mut ordering, &items, &mut x_ranks)
    });
    return (ordering, x_ranks);
}

fn populate_with_neighbour_values(
    items: &Vec<String>,
    neighbour_dict: &Dict<String, Vec<String>>,
    x_ranks: &mut Dict<String, f64>,
) {
    items.iter().for_each(|item| {
        let neighbours = neighbour_dict.get(item.clone());
        let mut xrank = get_xrank(item.clone(), &x_ranks);
        if neighbours.is_some() {
            neighbours.unwrap().iter().for_each(|neighbour| {
                let neigh_bour_xrank = get_xrank(neighbour.clone(), &x_ranks);
                xrank += neigh_bour_xrank;
            });
        }
        x_ranks.insert(item.clone(), xrank);
    });
}
fn initialize(items: &Vec<String>, x_ranks: &mut Dict<String, f64>) {
    let mut order = vec![];
    items.iter().for_each(|item| {
        order.push((item.clone(), get_xrank(item.clone(), &x_ranks)));
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
        order.push((item.clone(), get_xrank(item.clone(), &x_ranks)));
    });
    order.sort_by(|a, b| (b.1.partial_cmp(&a.1).unwrap()));
    let mut index = 0.0;
    order.iter().for_each(|x| {
        let median = (order.len() / 2) as f64;
        ordering.insert(x.0.clone(), index - median);
        index += 1.0;
    });
}
