mod astar;
mod common;
mod dijkstra;
mod floyd_warshall;

use alloc::vec::{IntoIter, Vec};
use std::iter::once;

use error_stack::{Context, Result};
use petgraph_core::{Graph, GraphStorage, Node};

pub use self::{astar::AStar, dijkstra::Dijkstra};
use crate::shortest_paths::common::path::Path;

pub struct Cost<T>(T);

impl<T> Cost<T> {
    fn value(&self) -> &T {
        &self.0
    }

    fn into_value(self) -> T {
        self.0
    }
}

pub struct Route<'a, S, T>
where
    S: GraphStorage,
{
    path: Path<'a, S>,

    cost: Cost<T>,
}

impl<'a, S, T> Route<'a, S, T>
where
    S: GraphStorage,
{
    fn reverse(self) -> Self {
        Self {
            path: self.path.reverse(),
            cost: self.cost,
        }
    }
}

pub struct DirectRoute<'a, S, T>
where
    S: GraphStorage,
{
    source: Node<'a, S>,
    target: Node<'a, S>,

    cost: Cost<T>,
}

impl<'a, S, T> DirectRoute<'a, S, T>
where
    S: GraphStorage,
{
    fn reverse(self) -> Self {
        Self {
            source: self.target,
            target: self.source,
            cost: self.cost,
        }
    }
}

pub trait ShortestPath<S>
where
    S: GraphStorage,
{
    type Cost;
    type Error: Context;

    fn path_to<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
        target: &'graph S::NodeId,
    ) -> Result<impl Iterator<Item = Route<'graph, S, Self::Cost>> + 'this, Self::Error> {
        let iter = self.every_path(graph)?;

        Ok(iter.filter(move |route| route.path.target.id() == target))
    }

    fn path_from<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
        source: &'graph S::NodeId,
    ) -> Result<impl Iterator<Item = Route<'graph, S, Self::Cost>> + 'this, Self::Error> {
        let iter = self.every_path(graph)?;

        Ok(iter.filter(move |route| route.path.source.id() == source))
    }

    fn path_between<'graph>(
        &self,
        graph: &'graph Graph<S>,
        source: &'graph S::NodeId,
        target: &'graph S::NodeId,
    ) -> Option<Route<'graph, S, Self::Cost>> {
        self.path_from(graph, source)
            .ok()?
            .find(|route| route.path.target.id() == target)
    }

    fn every_path<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
    ) -> Result<impl Iterator<Item = Route<'graph, S, Self::Cost>> + 'this, Self::Error>;
}

pub trait ShortestDistance<S>
where
    S: GraphStorage,
{
    type Cost;
    type Error: Context;

    fn distance_to<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
        target: &'graph S::NodeId,
    ) -> Result<impl Iterator<Item = DirectRoute<'graph, S, Self::Cost>> + 'this, Self::Error> {
        let iter = self.every_distance(graph)?;

        Ok(iter.filter(move |route| route.target.id() == target))
    }
    fn distance_from<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
        source: &'graph S::NodeId,
    ) -> Result<impl Iterator<Item = DirectRoute<'graph, S, Self::Cost>> + 'this, Self::Error> {
        let iter = self.every_distance(graph)?;

        Ok(iter.filter(move |route| route.source.id() == source))
    }
    fn distance_between<'graph>(
        &self,
        graph: &'graph Graph<S>,
        source: &'graph S::NodeId,
        target: &'graph S::NodeId,
    ) -> Option<Cost<Self::Cost>> {
        self.every_distance(graph)
            .ok()?
            .find(move |route| route.source.id() == source && route.target.id() == target)
            .map(|route| route.cost)
    }
    fn every_distance<'graph: 'this, 'this>(
        &'this self,
        graph: &'graph Graph<S>,
    ) -> Result<impl Iterator<Item = DirectRoute<'graph, S, Self::Cost>> + 'this, Self::Error>;
}
