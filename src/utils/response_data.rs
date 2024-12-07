use serde::Serialize;

use super::time::custom_timezone_with_fromat;

#[derive(Serialize, Debug)]
pub struct ResponseData<T> {
    pub message: &'static str,
    pub data: T,
    pub timestamp: String,
}

impl<T> ResponseData<T>
where
    T: Serialize,
{
    pub fn new(data: T, message: &'static str) -> Self {
        ResponseData {
            message,
            data,
            timestamp: custom_timezone_with_fromat(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ResponseDatas<T> {
    pub limit: Option<i64>,
    pub page: i64,
    pub page_count: u64,
    pub count: u64,
    pub current_count: u64,
    pub data: T,
}

impl<T> ResponseDatas<T>
where
    T: Serialize,
{
    pub fn new(limit: i64, page: i64, count: i64, current_count: usize, data: T) -> Self {
        let page_count: u64 = if limit == i64::MAX {
            1
        } else {
            ((count + limit - 1) / limit).try_into().unwrap_or(0)
        };

        ResponseDatas {
            limit: if limit == i64::MAX { None } else { Some(limit) },
            page,
            count: count as u64,
            page_count,
            current_count: current_count as u64,
            data,
        }
    }
}
