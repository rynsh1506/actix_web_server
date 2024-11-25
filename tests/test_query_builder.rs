#[cfg(test)]
mod tests {
    use web_server::utils::query_builder::QueryBuilder;

    #[test]
    fn test_select_query() {
        let query = QueryBuilder::new().from("users", "id, name").build();
        assert_eq!(query, "SELECT id, name FROM users");
    }

    #[test]
    fn test_select_with_single_column() {
        let query = QueryBuilder::new().from("users", "id").build();
        assert_eq!(query, "SELECT id FROM users");
    }

    #[test]
    fn test_select_with_subquery() {
        let subquery = QueryBuilder::new().from("orders", "id, total");
        let query = QueryBuilder::new()
            .from_nested(subquery, "id, name", "users")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM (SELECT id, total FROM orders) AS users"
        );
    }

    #[test]
    fn test_insert_query() {
        let query = QueryBuilder::new()
            .insert("users", "name, email", "'Alice', 'alice@example.com'")
            .build();
        assert_eq!(
            query,
            "INSERT INTO users (name, email) VALUES ('Alice', 'alice@example.com')"
        );
    }

    #[test]
    fn test_update_query() {
        let query = QueryBuilder::new()
            .update("users", "name = 'Bob', email = 'bob@example.com'")
            .build();
        assert_eq!(
            query,
            "UPDATE users SET name = 'Bob', email = 'bob@example.com'"
        );
    }

    #[test]
    fn test_delete_query() {
        let query = QueryBuilder::new().delete("users").build();
        assert_eq!(query, "DELETE FROM users");
    }

    #[test]
    fn test_condition() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .build();
        assert_eq!(query, "SELECT id, name FROM users WHERE age > 30");
    }

    #[test]
    fn test_and_condition() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .and()
            .and_condition("status = 'active'", "role = 'admin'")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE age > 30 AND (status = 'active' AND role = 'admin')"
        );
    }

    #[test]
    fn test_or_condition() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .or()
            .or_condition("status = 'active'", "role = 'admin'")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE age > 30 OR (status = 'active' OR role = 'admin')"
        );
    }

    #[test]
    fn test_in_condition() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .in_condition("age", "25, 30, 35")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE age IN (25, 30, 35)"
        );
    }

    #[test]
    fn test_join() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .join("INNER", "orders", "users.id = orders.user_id")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users INNER JOIN orders ON users.id = orders.user_id"
        );
    }

    #[test]
    fn test_where_clause() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .build();
        assert_eq!(query, "SELECT id, name FROM users WHERE age > 30");
    }

    #[test]
    fn test_and_op() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .and()
            .condition("status = 'active'")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE age > 30 AND status = 'active'"
        );
    }

    #[test]
    fn test_or_op() {
        let query = QueryBuilder::new()
            .from("users", "id, name")
            .where_clause()
            .condition("age > 30")
            .or()
            .condition("status = 'active'")
            .build();
        assert_eq!(
            query,
            "SELECT id, name FROM users WHERE age > 30 OR status = 'active'"
        );
    }
}
