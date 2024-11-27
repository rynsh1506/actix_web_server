use serde::Serialize;

#[derive(Serialize)]
pub struct ResponseData<T> {
    pub data: T,
}

impl<T> ResponseData<T>
where
    T: Serialize,
{
    pub fn new(data: T) -> Self {
        ResponseData { data }
    }
}

#[derive(Serialize)]
pub struct ResponseDatas<T> {
    pub limit: String,
    pub page: i64,
    pub count: u64,
    pub page_count: usize,
    pub order: String,
    pub data: T,
}

impl<T> ResponseDatas<T>
where
    T: Serialize,
{
    pub fn new(
        limit: String,
        page: i64,
        count: u64,
        page_count: usize,
        order: String,
        data: T,
    ) -> Self {
        ResponseDatas {
            limit,
            page,
            count,
            page_count,
            order,
            data,
        }
    }
}
