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
    pub limit: Option<i64>,
    pub page: i64,
    pub count: u64,
    pub page_count: u64,
    pub order: String,
    pub data: T,
}

impl<T> ResponseDatas<T>
where
    T: Serialize,
{
    pub fn new(
        limit: i64,
        page: i64,
        count: i64,
        page_count: usize,
        order: String,
        data: T,
    ) -> Self {
        ResponseDatas {
            limit: if limit == i64::MAX { None } else { Some(limit) },
            page,
            count: count as u64,
            page_count: page_count as u64,
            order,
            data,
        }
    }
}
