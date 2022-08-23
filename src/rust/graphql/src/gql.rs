use async_graphql::{
    ComplexObject, Context, EmptyMutation, EmptySubscription, Object, Schema, SimpleObject,
};

pub type StudentSchema = Schema<StudentQuery, EmptyMutation, EmptySubscription>;

//
// Model
//

#[derive(SimpleObject)]
#[graphql(complex)]
pub struct Student {
    #[graphql(visible = false)]
    pub repeat: u8,
    pub name: String,
}

#[ComplexObject]
impl Student {
    async fn repeat_name(&self) -> String {
        format!("{} ", self.name).repeat(self.repeat.into())
    }
}

//
// Query
//

#[derive(Default)]
pub struct StudentQuery;

#[Object]
impl StudentQuery {
    pub async fn student(&self, _ctx: &Context<'_>, repeat: u8) -> Student {
        Student {
            repeat,
            name: "Ferris".to_string(),
        }
    }
}
