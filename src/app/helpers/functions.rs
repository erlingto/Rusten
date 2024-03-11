use std::vec;

use super::dictionary::Dict;
use crate::app::structs::{
    connectionItem::ConnectionItem, moveBoxItem::MoveBoxItem, MoveBoxAttribute::MoveBoxAttribute,
};
use leptos::{create_rw_signal, RwSignal, SignalGet, SignalSet};
use leptos_use::core::Position;
use log::debug;

pub fn organizePositions(
    items: Vec<RwSignal<MoveBoxItem>>,
    connections: Vec<RwSignal<ConnectionItem>>,
) {
    let mut toRankDict = Dict::<String, i32>::new();
    let mut fromRankDict = Dict::<String, i32>::new();

    items.iter().for_each(|x| {
        let key = x.get().key.to_string();
        toRankDict.insert(key.clone(), 0);
        fromRankDict.insert(key.clone(), 0);
    });

    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();
        toRankDict.insert(to.clone(), toRankDict.get(to).unwrap() + 1);
        fromRankDict.insert(from.clone(), fromRankDict.get(from.clone()).unwrap() + 1);
    });

    let mut toRankneighbourWeighted = Dict::<String, i32>::new();
    let mut fromRankneighbourWeighted = Dict::<String, i32>::new();

    items.iter().for_each(|x| {
        let key = x.get().key.to_string();
        toRankneighbourWeighted.insert(key.clone(), toRankDict.get(key.clone()).unwrap().clone());
        fromRankneighbourWeighted
            .insert(key.clone(), fromRankDict.get(key.clone()).unwrap().clone());
    });

    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();
        let toNeighbourValue = toRankDict.get(from.clone());
        let fromNeighbourValue = fromRankDict.get(to.clone());
        if toNeighbourValue.is_some() {
            toRankneighbourWeighted.insert(
                to.clone(),
                *toNeighbourValue.unwrap() + toRankneighbourWeighted.get(to.clone()).unwrap(),
            );
        }
        if fromNeighbourValue.is_some() {
            fromRankneighbourWeighted.insert(
                from.clone(),
                *fromNeighbourValue.unwrap() + fromRankneighbourWeighted.get(from.clone()).unwrap(),
            );
        }
    });

    let mut toRankOverView = Dict::<i32, Vec<String>>::new();
    let mut fromRankOverView = Dict::<i32, Vec<String>>::new();

    toRankDict.into_iter().for_each(|x| {
        if toRankOverView.get(x.1).is_none() {
            toRankOverView.insert(x.1, vec![x.0]);
        } else {
            toRankOverView[x.1].push(x.0);
        }
    });

    fromRankDict.into_iter().for_each(|x| {
        if fromRankOverView.get(x.1).is_none() {
            fromRankOverView.insert(x.1, vec![x.0]);
        } else {
            fromRankOverView[x.1].push(x.0);
        }
    });

    let mut Xlevels = vec![];
    toRankOverView.clone().into_iter().for_each(|y| {
        Xlevels.push(y.1.len() as i32);
    });

    let (xOrdering, xRanks) = organizeXPositions(toRankOverView.clone(), connections.clone());
    items.iter().for_each(|item| {
        let ranks = toRankOverView
            .clone()
            .into_iter()
            .find(|x| x.1.contains(&item.get().key))
            .unwrap();
        let xLevel = xOrdering.get(item.get().key.clone()).unwrap();
        let yLevel = ranks.0;

        let size = item.get().size.get();
        let yPosition = 100.0 + (size.y + 145.0) * yLevel as f64;
        let xPosition = 500.0 + xLevel * (size.x + 145.0);
        item.get().position.set(Position {
            x: xPosition,
            y: yPosition,
        });
    });
}

fn getNeighBours(connections: Vec<RwSignal<ConnectionItem>>) -> Dict<String, Vec<String>> {
    let mut neighbours = Dict::<String, Vec<String>>::new();
    connections.iter().for_each(|x| {
        let from = x.get().from.get().key.to_string();
        let to = x.get().to.get().key.to_string();

        if (neighbours.get(from.clone()).is_none()) {
            neighbours.insert(from.clone(), vec![to.clone()]);
        } else {
            neighbours[from.clone()].push(to.clone());
        }
        if (neighbours.get(to.clone()).is_none()) {
            neighbours.insert(to.clone(), vec![from.clone()]);
        } else {
            neighbours[to.clone()].push(from.clone());
        }
    });
    neighbours
}

fn getXrank(itemId: String, xRanks: &Dict<String, f64>) -> f64 {
    let rank = xRanks.get(itemId);
    if (rank.is_some()) {
        return *rank.unwrap();
    }
    return 0.0;
}

fn organizeXPositions(
    toRankOverView: Dict<i32, Vec<String>>,
    connections: Vec<RwSignal<ConnectionItem>>,
) -> (Dict<String, f64>, Dict<String, f64>) {
    let mut xRanks = Dict::<String, f64>::new();
    let neighbourDict = getNeighBours(connections);
    let mut ordering = Dict::<String, f64>::new();
    let mut toRankKeys = toRankOverView
        .clone()
        .into_iter()
        .map(|x| x.0)
        .collect::<Vec<i32>>();

    toRankKeys.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let firstKey = toRankKeys.first().unwrap();
    initialize(&toRankOverView[firstKey.clone()], &mut xRanks);
    toRankKeys.into_iter().for_each(|key| {
        let items = toRankOverView[key].clone();
        populateWithNeighBourValues(&items, &neighbourDict, &mut xRanks);
        populateWithNeighBourValues(&items, &neighbourDict, &mut xRanks);
        order(&mut ordering, &items, &mut xRanks)
    });
    return (ordering, xRanks);
}

fn populateWithNeighBourValues(
    items: &Vec<String>,
    neighbourDict: &Dict<String, Vec<String>>,
    xRanks: &mut Dict<String, f64>,
) {
    items.iter().for_each(|item| {
        let neighbours = neighbourDict.get(item.clone());
        let mut xrank = getXrank(item.clone(), &xRanks);
        if (neighbours.is_some()) {
            neighbours.unwrap().iter().for_each(|neighbour| {
                let neighBourXrank = getXrank(neighbour.clone(), &xRanks);
                xrank += neighBourXrank;
            });
        }
        xRanks.insert(item.clone(), xrank);
    });
}
fn initialize(items: &Vec<String>, xRanks: &mut Dict<String, f64>) {
    let mut order = vec![];
    items.iter().for_each(|item| {
        order.push((item.clone(), getXrank(item.clone(), &xRanks)));
    });
    let mut index = 0.0;
    let increments = 100.0;
    order.iter().for_each(|x| {
        let median = (order.len() / 2) as f64;
        xRanks.insert(x.0.clone(), (index - median) * increments);
        index += 1.0;
    });
}

fn order(ordering: &mut Dict<String, f64>, items: &Vec<String>, xRanks: &mut Dict<String, f64>) {
    let mut order = vec![];
    items.iter().for_each(|item| {
        order.push((item.clone(), getXrank(item.clone(), &xRanks)));
    });
    order.sort_by(|a, b| (b.1.partial_cmp(&a.1).unwrap()));
    let mut index = 0.0;
    order.iter().for_each(|x| {
        let median = (order.len() / 2) as f64;
        ordering.insert(x.0.clone(), (index - median));
        index += 1.0;
    });
}
