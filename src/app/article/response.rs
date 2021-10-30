use crate::app::article::model::Article;
use crate::app::profile::model::Profile;
use crate::app::tag::model::Tag;
use crate::app::user::model::User;
use serde::{Deserialize, Serialize};
use std::convert::From;
type ArticleCount = i64;

#[derive(Deserialize, Serialize)]
pub struct SingleArticleResponse {
    pub article: ArticleContent,
}

impl From<(Article, Profile, Vec<Tag>)> for SingleArticleResponse {
    fn from((article, profile, tag_list): (Article, Profile, Vec<Tag>)) -> Self {
        Self {
            article: ArticleContent {
                slug: article.slug,
                title: article.title,
                description: article.description,
                body: article.body,
                tagList: tag_list
                    .iter()
                    .map(move |tag| tag.name.to_owned())
                    .collect(),
                createdAt: article.created_at.to_string(),
                updatedAt: article.updated_at.to_string(),
                author: AuthorContent {
                    username: profile.username,
                    bio: profile.bio,
                    image: profile.image,
                    following: profile.following,
                },
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct MultipleArticlesResponse {
    pub articles: Vec<ArticleContent>,
    pub articlesCount: ArticleCount,
}

type DEPRECATED_Info = ((Article, User), Vec<Tag>);
impl From<(Vec<DEPRECATED_Info>, ArticleCount)> for MultipleArticlesResponse {
    fn from((info, articles_count): (Vec<DEPRECATED_Info>, ArticleCount)) -> Self {
        let articles = info
            .iter()
            .map(|((article, user), tags_list)| {
                ArticleContent::DEPRECATED_from(
                    article.to_owned(),   // TODO: avoid copy
                    user.clone(),         // TODO: avoid copy
                    tags_list.to_owned(), // TODO: avoid copy
                )
            })
            .collect();
        Self {
            articlesCount: articles_count,
            articles: articles,
        }
    }
}

type Info = ((Article, Profile), Vec<Tag>);
impl From<(Vec<Info>, ArticleCount)> for MultipleArticlesResponse {
    fn from((info, articles_count): (Vec<Info>, ArticleCount)) -> Self {
        let articles = info
            .iter()
            .map(|((article, profile), tags_list)| {
                ArticleContent::from(article.to_owned(), profile.to_owned(), tags_list.to_owned())
            })
            .collect();
        Self {
            articlesCount: articles_count,
            articles: articles,
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct ArticleContent {
    pub slug: String,
    pub title: String,
    pub description: String,
    pub body: String,
    pub tagList: Vec<String>,
    pub createdAt: String,
    pub updatedAt: String,
    // TODO: add favorited info
    // pub favorited,
    // pub favoritesCount,
    pub author: AuthorContent,
}

impl ArticleContent {
    pub fn from(article: Article, profile: Profile, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            author: AuthorContent {
                username: profile.username,
                bio: profile.bio,
                image: profile.image,
                following: profile.following,
            },
        }
    }

    pub fn DEPRECATED_from(article: Article, user: User, tag_list: Vec<Tag>) -> Self {
        Self {
            slug: article.slug,
            title: article.title,
            description: article.description,
            body: article.body,
            tagList: tag_list.iter().map(move |tag| tag.name.clone()).collect(),
            createdAt: article.created_at.to_string(),
            updatedAt: article.updated_at.to_string(),
            author: AuthorContent {
                username: user.username,
                bio: user.bio,
                image: user.image,
                following: true, // TODO: get following by db
            },
        }
    }
}

#[derive(Deserialize, Serialize)]
pub struct AuthorContent {
    pub username: String,
    pub bio: Option<String>,
    pub image: Option<String>,
    pub following: bool,
}
