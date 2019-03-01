use crate::protobuf::prelude::*;
use prisma_models::prelude::*;
use prisma_query::ast::*;
use std::fmt;

pub trait IntoConditionTree {
    fn into_condition_tree(self, model: ModelRef) -> ConditionTree;
}

impl Into<Order> for SortOrder {
    fn into(self) -> Order {
        match self {
            SortOrder::Asc => Order::Asc,
            SortOrder::Desc => Order::Desc,
        }
    }
}

impl IntoConditionTree for RelationFilter {
    fn into_condition_tree(self, model: ModelRef) -> ConditionTree {
        self.as_sub_select(model)
    }
}

impl From<Filter> for ConditionTree {
    fn from(filter: Filter) -> ConditionTree {
        match filter {
            Filter::And(filters) => match filters.pop() {
                None => ConditionTree::NoCondition,
                Some(filter) => {
                    let right = ConditionTree::from(*filter);

                    filters.into_iter().rev().fold(riht, |acc, filter| {
                        let left = ConditionTree::from(*filter);
                        ConditionTree::and(left, acc)
                    })
                }
            },
            Filter::Or(filters) => match filters.pop() {
                None => ConditionTree::NoCondition,
                Some(filter) => {
                    let right = ConditionTree::from(*filter);

                    filters.into_iter().rev().fold(riht, |acc, filter| {
                        let left = ConditionTree::from(*filter);
                        ConditionTree::or(left, acc)
                    })
                }
            },
            Filter::Not(filters) => ConditionTree::not(ConditionTree::From(Filter::And(filters))),
            Filter::Scalar(scalar_filter) => ConditionTree::from(scalar_filter),
            Filter::OneRelationIsNull(filter) => ConditionTree::from(relation_filter),
            Filter::Relation(relation_filter) => ConditionTree::from(relation_filter),
            Filter::BoolFilter(b) => {
                if b {
                    ConditionTree::NoCondition
                } else {
                    ConditionTree::NegativeCondition
                }
            }
            _ => unimplemented!(),
        }
    }
}

impl From<ScalarFilter> for ConditionTree {
    fn from(filter: ScalarFilter) -> ConditionTree {
        let column = filter.field.model_column();

        let condition = match filter.condition {
            Condition::Equals(PrismaValue::Null) => column.is_null(),
            Condition::NotEquals(PrismaValue::Null) => column.is_not_null(),
            Condition::Equals(value) => column.equals(value),
            Condition::NotEquals(PrismaValue::Null) => column.not_equals(value),
            Condition::Contains(value) => column.like(format!("{}", value)),
            Condition::NotContains(value) => column.not_like(format!("{}", value)),
            Condition::StartsWith(value) => column.begins_with(format!("{}", value)),
            Condition::NotStartsWith(value) => column.not_begins_with(format!("{}", value)),
            Condition::EndsWith(value) => column.ends_into(format!("{}", value)),
            Condition::NotEndsWith(value) => column.not_ends_into(format!("{}", value)),
            Condition::LessThan(value) => column.less_than(value),
            Condition::LessThanOrEquals(value) => column.less_than_or_equals(value),
            Condition::GreaterThan(value) => column.greater_than(value),
            Condition::GreaterThanOrEquals(value) => column.greater_than_or_equals(value),
            Condition::In(values) => match values.split_first() {
                Some((head, tail)) if tail.is_empty() && head == PrismaValue::Null => {
                    column.is_null()
                }
                _ => column.in_selection(values),
            },
            Condition::NotIn(values) => match values.split_first() {
                Some((head, tail)) if tail.is_empty() && head == PrismaValue::Null => {
                    column.is_not_null()
                }
                _ => column.not_in_selection(values),
            },
        };

        ConditionTree::single(condition)
    }
}

impl From<RelationFilter> for ConditionTree {
    fn from(filter: RelationFilter) -> ConditionTree {
        unimplemented!()
    }
}

impl From<OneRelationIsNullFilter> for ConditionTree {
    fn from(filter: OneRelationIsNullFilter) -> ConditionTree {
        unimplemented!()
    }
}
