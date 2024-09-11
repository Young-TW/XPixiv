use crate::data::{Api, Illusts};

use std::collections::HashMap;

pub struct User {
    id: usize,
}

impl User {
    pub fn new(id: usize) -> Self {
        Self { id }
    }

    pub async fn get_artworks(&self) -> reqwest::Result<Vec<usize>> {
        let data = reqwest::get(format!(
            "https://www.pixiv.net/ajax/user/{}/profile/all",
            self.id
        ))
        .await?
        .error_for_status()?
        .json::<Api<Illusts<HashMap<usize, Option<bool>>>>>()
        .await?;
        let images = data
            .body
            .illusts
            .keys()
            .map(|k| k.clone())
            .collect::<Vec<usize>>();

        Ok(images)
    }

    pub async fn get_following(&self) -> reqwest::Result<Vec<usize>> {
        let data = reqwest::get(format!(
            "https://www.pixiv.net/ajax/user/{}/following",
            self.id
        ))
        .await?
        .error_for_status()?
        .json::<Api<HashMap<usize, Option<bool>>>>()
        .await?;
        let following = data.body.keys().map(|k| k.clone()).collect::<Vec<usize>>();

        println!("{:?}", following);

        Ok(following)
    }
}

#[cfg(test)]
mod test {
    use super::User;

    #[tokio::test]
    async fn test() {
        let images = User::new(3115085).get_artworks().await.unwrap();
        println!("{:?}", images);
    }
}
