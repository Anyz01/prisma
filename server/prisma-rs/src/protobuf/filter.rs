use super::prisma as pb;
use prisma_models::prelude::*;
use std::sync::Arc;

pub trait IntoFilter {
    fn into_filter(self, model: ModelRef) -> Filter;
}

impl IntoFilter for pb::Filter {
    fn into_filter(self, model: ModelRef) -> Filter {
        match self.type_ {
            pb::filter::Type::And(and) => {
                Filter::And(and.filters.map(|filter| filter.into_filter(model)))
            }
            pb::filter::Type::Or(or) => {
                Filter::Or(or.filters.map(|filter| filter.into_filter(model)))
            }
            pb::filter::Type::Not(not) => {
                Filter::Not(not.filters.map(|filter| filter.into_filter(model)))
            }
            pb::filter::Type::Scalar(scalar) => scalar.into_filter(model),
            pb::filter::Type::ScalarList(scalar_list) => scalar_list.into_filter(model),
            pb::filter::Type::OneRelationIsNull(relation_field) => {
                let field = model
                    .fields()
                    .find_from_relation_fields(relation_field.field.as_ref())
                    .unwrap();

                Filter::OneRelationIsNull(field)
            }
            pb::filter::Type::Relation(relation_filter) => relation_filter.into_filter(model),
            pb::filter::Type::NodeSubscription(_) => Filter::NodeSubscription,
            pb::filter::Type::BoolFilter(boo) => Filter::BoolFilter(boo),
        }
    }
}

impl IntoFilter for pb::ScalarFilter {
    fn into_filter(self, model: ModelRef) -> Filter {
        use pb::scalar_filter::Condition::*;

        let field = model
            .fields()
            .find_from_scalar(self.field.as_ref())
            .unwrap();

        let condition = match self.condition.unwrap() {
            Equals(value) => ScalarCondition::Equals(value.into()),
            NotEquals(value) => ScalarCondition::NotEquals(value.into()),
            Contains(value) => ScalarCondition::Contains(value.into()),
            NotContains(value) => ScalarCondition::NotContains(value.into()),
            StartsWith(value) => ScalarCondition::StartsWith(value.into()),
            NotStartsWith(value) => ScalarCondition::NotStartsWith(value.into()),
            EndsWith(value) => ScalarCondition::EndsWith(value.into()),
            NotEndsWith(value) => ScalarCondition::NotEndsWith(value.into()),
            LessThan(value) => ScalarCondition::LessThan(value.into()),
            LessThanOrEquals(value) => ScalarCondition::LessThanOrEquals(value.into()),
            GreaterThan(value) => ScalarCondition::GreaterThan(value.into()),
            GreaterThanOrEquals(value) => ScalarCondition::GreaterThanOrEquals(value.into()),
            In(values) => ScalarCondition::In(values.map(|value| value.into()).collect()),
            NotIn(values) => ScalarCondition::NotIn(values.map(|value| value.into()).collect()),
        };

        Filter::Scalar(ScalarFilter { field, condition })
    }
}

impl IntoFilter for pb::ScalarFilter {
    fn into_filter(self, model: ModelRef) -> Filter {
        use pb::scalar_list_condition::Condition::*;

        let field = model
            .fields()
            .find_from_scalar(self.field.as_ref())
            .unwrap();

        let condition = match self.condition.unwrap() {
            Contains(value) => ScalarListCondition::Contains(value.into()),
            ContainsEvery(values) => {
                ScalarListCondition::ContainsEvery(values.map(|value| value.into()).collect())
            }
            ContainsSome(values) => {
                ScalarListCondition::ContainsSome(values.map(|value| value.into()).collect())
            }
        };

        Filter::ScalarList(field, condition)
    }
}

impl IntoFilter for pb::RelationFilter {
    fn into_filter(self, model: ModelRef) -> Filter {
        let field = model
            .fields()
            .find_from_scalar(self.field.as_ref())
            .unwrap();

        let nested_filter: Box<Filter> = Box::new((*self.nested_filter).into());

        let condition = match self.condition() {
            pb::relation_filter::Condition::EveryRelatedNode => RelationCondition::EveryRelatedNode,
            pb::relation_filter::Condition::AtLeastOneRelatedNode => {
                RelationCondition::AtLeastOneRelatedNode
            }
            pb::relation_filter::Condition::NoRelatedNode => RelationCondition::NoRelatedNode,
            pb::relation_filter::Condition::ToOneRelatedNode => RelationCondition::ToOneRelatedNode,
        };

        Filter::Relation(RelationFilter {
            field,
            nested_filter,
            condition,
        })
    }
}

impl From<pb::ValueContainer> for PrismaValue {
    fn from(container: pb::ValueContainer) -> PrismaValue {
        use pb::value_container as vc;

        match container.prisma_value.unwrap() {
            vc::PrismaValue::String(v) => PrismaValue::String(v),
            vc::PrismaValue::Float(v) => PrismaValue::Float(v),
            vc::PrismaValue::Boolean(v) => PrismaValue::Boolean(v),
            vc::PrismaValue::DateTime(v) => PrismaValue::DateTime(v),
            vc::PrismaValue::Enum(v) => PrismaValue::Enum(v),
            vc::PrismaValue::Json(v) => PrismaValue::Json(v),
            vc::PrismaValue::Int(v) => PrismaValue::Int(v),
            vc::PrismaValue::Relation(v) => PrismaValue::Relation(v as usize),
            vc::PrismaValue::Null(_) => PrismaValue::Null,
            vc::PrismaValue::Uuid(v) => PrismaValue::Uuid(v),
            vc::PrismaValue::GraphqlId(v) => PrismaValue::GraphqlId(v.into()),
        }
    }
}

impl From<pb::GraphqlId> for GraphqlId {
    fn from(id: pb::GraphqlId) -> GraphqlId {
        match id.id_value.unwrap() {
            pb::graphql_id::IdValue::String(s) => GraphqlId::String(s),
            pb::graphql_id::IdValue::Int(i) => GraphqlId::Int(i as usize),
        }
    }
}
