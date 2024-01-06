use validator::Validate;
use axum::async_trait;
use std:: {
    collections::HashMap,
    sync::{Arc, RwLock,RwLockReadGuard, RwLockWriteGuard},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;
use anyhow::{Context, Ok};

// リポジトリで発生しうるエラーを定義
#[derive(Debug, Error)]
enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static{
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>;
    async fn find(&self, id: i32) -> anyhow::Result<Todo>;
    async fn all(&self) -> anyhow::Result<Vec<Todo>>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}

// Todoに必要な構造体を定義
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct Todo {
    id: i32,
    text: String,
    completed: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct CreateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "Over text length"))]
    text: String,
}

#[cfg(test)]
impl CreateTodo {
    pub fn new(text: String) -> Self {
        Self { text }
    }
}

// Option<T>型は 取得できないかもしれない値 を表現する列挙型であり、値が無いことを示すNoneとあることを示すSome(T)のどちらかをとる
// cf, Result<T,E>は失敗するかもしれない処理の結果を表現する列挙型である。適切な使い分けが必要
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, Validate)]
pub struct UpdateTodo {
    #[validate(length(min = 1, message = "Can not be empty"))]
    #[validate(length(max = 100, message = "over text length"))]
    text: Option<String>,
    completed: Option<bool>,
}


impl Todo {
    pub fn new(id: i32, text:String) -> Self {
        Self{
            id, 
            text,
            completed: false,
        }
    }
}

type TodoDatas = HashMap<i32, Todo>;

#[derive(Debug, Clone)]
pub struct TodoRepositoryForMemory {
    store: Arc<RwLock<TodoDatas>>
}

impl TodoRepositoryForMemory {
    pub fn new() -> Self {
        TodoRepositoryForMemory {
            store: Arc::default(),
        }
    }
    
    fn write_store_ref(&self) -> RwLockWriteGuard<TodoDatas> {
        self.store.write().unwrap()
    }
    
    fn read_store_ref(&self) -> RwLockReadGuard<TodoDatas> {
        self.store.read().unwrap()
    }
}


#[async_trait]
impl TodoRepository for TodoRepositoryForMemory {
    async fn create(&self, payload: CreateTodo) -> anyhow::Result<Todo>{
        let mut store = self.write_store_ref();
        let id = (store.len() + 1) as i32; //as：型キャスト
        let todo = Todo::new(id, payload.text.clone());
        store.insert(id, todo.clone());
        Ok(todo)
    }

    async fn find(&self, id: i32) -> anyhow::Result<Todo> {
        let store = self.read_store_ref();
        let todo = store
            .get(&id)
            .map(|todo| todo.clone())
            .ok_or(RepositoryError::NotFound(id))?;
        Ok(todo)
    }

    // Note: find() の実装に Box を使うパターン. clone の回数が増えるならヒープの利用を検討する
    // fn find(&self, id: i32) -> Option<Box<Todo>> {
    //     let store = self.read_store_ref();
    //     let todo = store.get(&id);
    //     let todo = Box::new(todo.clone());
    //     Some(Todo)
    // }
    
    async fn all(&self) -> anyhow::Result<Vec<Todo>> {
        let store = self.read_store_ref();
        Ok(Vec::from_iter(store.values().map(|todo| todo.clone())))
    }

    async fn update(&self, id: i32, payload: UpdateTodo) -> 
    anyhow::Result<Todo> {
        let mut store = self.write_store_ref();
        let todo = store
                .get(&id)
                .context(RepositoryError::NotFound(id))?;
        let text = payload.text.unwrap_or(todo.text.clone());
        let completed = payload.completed.unwrap_or(todo.completed);
        let todo = Todo{
            id,
            text,
            completed,
        };
        store.insert(id, todo.clone());
        Ok(todo)
    }

    async fn delete(&self, id: i32) -> anyhow::Result<()> {
        let mut store = self.write_store_ref();
        store.remove(&id).ok_or(RepositoryError::NotFound(id))?;
        Ok(())
    }
}

// テストケース
// 1.create：Todoを作成 
// 2.find：1で作成したTodoをidで取得
// 3.all：すべてのTodoを取得
// 4.update：1で作成したTodoを更新
// 5.delete：1で作成したTodoを削除

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn todo_crud_scenario() {
        let text = "todo text".to_string();
        let id = 1;
        let expected = Todo::new(id, text.clone());

        // 1.create：Todoを作成
        let repository = TodoRepositoryForMemory::new();
        let todo = repository
                .create(CreateTodo { text })
                .await.expect("failed create todo");
        assert_eq!(expected, todo);

        // 2.find：1で作成したTodoをidで取得
        let todo = repository.find(todo.id).await.unwrap();
        assert_eq!(expected, todo);

        // 3.all：すべてのTodoを取得
        let todo = repository.all().await.expect("failed get all todo");
        assert_eq!(vec![expected], todo);

        // 4.update：1で作成したTodoを更新
        let text = "update todo text".to_string();
        let todo = repository
            .update(
                1,
                UpdateTodo {
                    text: (Some(text.clone())),
                    completed: Some(true), 
                },)
            .await
            .expect("failed update todo.");
        assert_eq!(
            Todo {
                id,
                text,
                completed:true,
            },
            todo
        );
            
        // 5.delete：1で作成したTodoを削除
        let res = repository.delete(id).await;
        assert!(res.is_ok())
    }
    
}

