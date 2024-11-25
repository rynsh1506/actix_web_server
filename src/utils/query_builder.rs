use core::fmt;

#[derive(Debug, Default)]
pub struct QueryBuilder {
    query: String,
}

impl fmt::Display for QueryBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.build())
    }
}

impl QueryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn raw(mut self, query: &str) -> Self {
        self.query.push_str(query);
        self
    }

    // Dynamic column handling
    pub fn from(mut self, table: &str, columns: &str) -> Self {
        self.query
            .push_str(&format!("SELECT {} FROM {}", columns, table));
        self
    }

    pub fn from_nested(mut self, subquery: QueryBuilder, columns: &str, alias: &str) -> Self {
        self.query.push_str(&format!(
            "SELECT {} FROM ({}) AS {}",
            columns,
            subquery.build(),
            alias
        ));
        self
    }

    pub fn insert(mut self, table: &str, columns: &str, values: &str) -> Self {
        self.query.push_str(&format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table, columns, values
        ));
        self
    }

    pub fn update(mut self, table: &str, set_values: &str) -> Self {
        self.query
            .push_str(&format!("UPDATE {} SET {}", table, set_values));
        self
    }

    pub fn delete(mut self, table: &str) -> Self {
        self.query.push_str(&format!("DELETE FROM {}", table));
        self
    }

    pub fn condition(mut self, condition: &str) -> Self {
        self.query.push_str(condition);
        self
    }

    pub fn and_condition(mut self, condition1: &str, condition2: &str) -> Self {
        self.query
            .push_str(&format!("({} AND {})", condition1, condition2));
        self
    }

    pub fn or_condition(mut self, condition1: &str, condition2: &str) -> Self {
        self.query
            .push_str(&format!("({} OR {})", condition1, condition2));
        self
    }

    pub fn in_condition(mut self, columns: &str, values: &str) -> Self {
        self.query.push_str(&format!("{} IN ({})", columns, values));
        self
    }

    pub fn join(mut self, join_type: &str, table: &str, on_condition: &str) -> Self {
        self.query.push_str(&format!(
            " {} JOIN {} ON {}",
            join_type, table, on_condition
        ));
        self
    }

    pub fn where_clause(mut self) -> Self {
        self.query.push_str(" WHERE ");
        self
    }

    pub fn and(mut self) -> Self {
        self.query.push_str(" AND ");
        self
    }

    pub fn or(mut self) -> Self {
        self.query.push_str(" OR ");
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.query.push_str(&format!(" LIMIT {}", limit));
        self
    }

    pub fn offset(mut self, offset: i64) -> Self {
        self.query.push_str(&format!(" OFFSET {}", offset));
        self
    }

    pub fn order_by(mut self, columns: &str, direction: &str) -> Self {
        self.query
            .push_str(&format!(" ORDER BY {} {}", columns, direction));
        self
    }

    pub fn group_by(mut self, columns: &str) -> Self {
        self.query.push_str(&format!(" GROUP BY {}", columns));
        self
    }

    pub fn returning(mut self, columns: &str) -> Self {
        self.query.push_str(&format!(" RETURNING {}", columns));
        self
    }

    pub fn build(&self) -> String {
        self.query.clone()
    }
}
